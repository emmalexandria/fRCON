use std::io;

use colored::*;

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

use argh::FromArgs;

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
    rcon.auth(&args.password).await.unwrap();

    if args.run.is_some() {
        match rcon.send_command(args.run.unwrap().trim()).await {
            Ok(s) => println!("{}", s),
            Err(e) => eprintln!("{}", e)
        }
        std::process::exit(0);
    }

    let mut rl = DefaultEditor::new().unwrap();

    //This is literally a crime but its relatively unimportant.
    println!("{} {}{}{}{}{}", "Sucessfully connected to server".green().bold(), "(".green(), args.address.green(), ":".green(), args.port.to_string().green(), ")".green());

    println!("CTRL+C or type Q to quit");

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                if line == "Q" {
                    std::process::exit(0)
                }
                let response = rcon.send_command(&line).await;
                match response {
                    Ok(s) => {
                        if s.len() > 0 {
                            println!("{}", s)
                        }
                    }, 
                    Err(e) => {
                        if e.kind() == io::ErrorKind::InvalidInput {
                            println!("{} {}", "Input error:".red().bold(), e.to_string().red())
                        }
                    }

                }

            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(err) => {
                println!("Rustyline error: {:?}", err);
                break
            }
        }
    }
}
