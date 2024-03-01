use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

use argh::FromArgs;

mod rcon;


#[derive(FromArgs)]
#[argh(description = "Minecraft RCON Implementation for Rust")]
struct Args {
    #[argh(option, description="IPv4 address of the server")]
    address: String,

    #[argh(option, description="RCON port of the server", default="25575")]
    port: u16,

    #[argh(option, description="RCON password for the server")]
    password: String,

    #[argh(option, description="when passed, the given command will be ran and no interactive shell will be entered")]
    run: Option<String>
}




//RCON packet IDs

//RCON packet

#[tokio::main]
async fn main() {
    let args: Args = argh::from_env();


    let pid = std::process::id();

    let mut rcon = rcon::RCONConnection::new(&args.address, args.port, pid as i32).await.unwrap();
    rcon.auth("chocolatedog36").await.unwrap();

    if args.run.is_some() {
        match rcon.send_command(args.run.unwrap().trim()).await {
            Ok(s) => println!("{}", s),
            Err(e) => eprintln!("{}", e)
        }
        std::process::exit(0);
    }

    let mut rl = DefaultEditor::new().unwrap();

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let response = rcon.send_command(&line).await.unwrap();
                println!("{}", response);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
}
