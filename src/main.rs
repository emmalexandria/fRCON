use std::{io, thread::sleep, time::Duration};

use colored::*;

use argh::FromArgs;
use shell::RCONShell;

mod rcon;
mod shell;


#[derive(FromArgs)]
#[argh(description = "Minecraft RCON Implementation for Rust")]
struct Args {
    #[argh(option, description="IPv4 address of the server")]
    address: String,

    #[argh(option, description="RCON port of the server", default="25575")]
    port: u16,

    #[argh(option, description="RCON password for the server")]
    password: String,

    #[argh(positional, description="if passed, commands will be executed and shell mode will not be entered")]
    commands: Vec<String>,

    #[argh(option, description="seconds to wait between each passed command")]
    wait: Option<u64>,
}




//RCON packet IDs

//RCON packet

#[tokio::main]
async fn main() {
    let args: Args = argh::from_env();


    let pid = std::process::id();

    let mut rcon = rcon::RCONConnection::new(&args.address, args.port, pid as i32).await.unwrap();
    rcon.auth(&args.password).await.unwrap();



    if args.commands.len() > 0 {
        for cmd in args.commands {
            match rcon.send_command(cmd.trim()).await {
                Ok(s) => println!("{}", s),
                Err(e) => eprintln!("{}", e)
            }

            if args.wait.is_some() {
                sleep(Duration::from_secs(args.wait.unwrap()))
            }

        }
        
        std::process::exit(0);
    }

    let mut shell = RCONShell::new(&mut rcon, String::from("/"), args.address, args.port);

    shell.run().await;
}
