use std::io;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[derive(Debug)]
enum RCONCommand {
    ServerAuth,
    ServerAuthResponse,
    ServerExec,
    ServerResponseValue,
    Unknown(i32),
}

impl RCONCommand {
    pub fn to_i32(&self) -> i32 {
        return match self {
            RCONCommand::ServerAuth => 3,
            RCONCommand::ServerAuthResponse => 2,
            RCONCommand::ServerExec => 2,
            RCONCommand::ServerResponseValue => 0,
            RCONCommand::Unknown(n) => *n,
        };
    }

    pub fn from_i32(n: i32, is_response: bool) -> RCONCommand {
        match n {
            3 => RCONCommand::ServerAuth,
            2 if is_response => RCONCommand::ServerAuthResponse,
            2 => RCONCommand::ServerExec,
            0 => RCONCommand::ServerResponseValue,
            n => RCONCommand::Unknown(n),
        }
    }
}

//defines the size of the data preceeding body in RCONPacket
const PACKET_SIZE_CONST: usize = 10;

//max size of the body in bytes
const MAX_PACKET_SIZE: usize = 4096;

#[derive(Debug)]
struct RCONPacket {
    length: i32,
    id: i32,
    command: RCONCommand,
    body: String,
}

impl RCONPacket {
    pub fn new(id: i32, command: RCONCommand, body: String) -> io::Result<RCONPacket> {
        if body.len() > (MAX_PACKET_SIZE - PACKET_SIZE_CONST) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Command exceeds max size of body",
            ));
        }

        //length = id: 4, command: 4, null terminator after command: 1, null terminator at end: 1
        Ok(RCONPacket {
            length: (PACKET_SIZE_CONST + body.len()) as i32,
            id,
            command,
            body,
        })
    }

    pub async fn serialize(&self, stream: &mut TcpStream) -> io::Result<()> {
        let mut buf = Vec::with_capacity(self.length as usize);

        //RCON requires LE encoding
        buf.extend_from_slice(&self.length.to_le_bytes());
        buf.extend_from_slice(&self.id.to_le_bytes());
        buf.extend_from_slice(&self.command.to_i32().to_le_bytes());
        buf.extend_from_slice(self.body.as_bytes());

        //two null terminator bytes (one for the body and one required by the RCON spec)
        buf.extend_from_slice(&[0x00, 0x00]);

        stream.write_all(&buf).await?;

        Ok(())
    }

    pub async fn deserialize(stream: &mut TcpStream) -> io::Result<RCONPacket> {
        //buffer to read exactly one i32 at a time
        let mut buf = [0u8; 4];

        stream.read_exact(&mut buf).await?;
        let length = i32::from_le_bytes(buf);
        stream.read_exact(&mut buf).await?;
        let id = i32::from_le_bytes(buf);
        stream.read_exact(&mut buf).await?;
        let command = i32::from_le_bytes(buf);
        let body_length = length - (PACKET_SIZE_CONST as i32);
        let mut body_buffer = Vec::with_capacity(body_length as usize);

        stream
            .take(body_length as u64)
            .read_to_end(&mut body_buffer)
            .await?;

        let body = String::from_utf8(body_buffer)
            .map_err(|_| io::Error::from(io::ErrorKind::InvalidData))?;

        //buffer for terminating null pointers at the end of response
        let mut buf = [0u8; 2];
        stream.read_exact(&mut buf).await?;

        let packet = RCONPacket {
            length,
            id,
            command: RCONCommand::from_i32(command, true),
            body,
        };

        Ok(packet)
    }
}

pub struct RCONConnection {
    id: i32,
    stream: TcpStream,
}

impl RCONConnection {
    pub async fn new(
        address: &str,
        port: u16,
        id: i32,
    ) -> Result<RCONConnection, tokio::io::Error> {
        let conn = RCONConnection {
            stream: TcpStream::connect(String::from(address) + ":" + port.to_string().as_str())
                .await?,
            id,
        };

        return Ok(conn);
    }

    pub async fn auth(&mut self, password: &str) -> std::io::Result<()> {
        //this should almost never panic, cleaner to just unwrap
        let packet = RCONPacket::new(self.id, RCONCommand::ServerAuth, String::from(password))?;
        packet.serialize(&mut self.stream).await.unwrap();

        let response = RCONPacket::deserialize(&mut self.stream).await?;
        if response.id != self.id {
            return Err(std::io::Error::new(
                std::io::ErrorKind::ConnectionRefused,
                "Failed to connect to Minecraft server",
            ));
        }

        Ok(())
    }

    pub async fn send_command(&mut self, command: &str) -> std::io::Result<String> {
        let packet = RCONPacket::new(self.id, RCONCommand::ServerExec, String::from(command))?;
        packet.serialize(&mut self.stream).await.unwrap();

        let response = RCONPacket::deserialize(&mut self.stream).await?;

        Ok(response.body)
    }
}
