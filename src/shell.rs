use colored::Colorize;
use crossterm::cursor::MoveToColumn;
use crossterm::event::{self, poll, read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::style::{Color, ContentStyle, SetForegroundColor, SetStyle, Stylize};
use crossterm::terminal::Clear;
use crossterm::{execute, ExecutableCommand, cursor, terminal, terminal::ClearType};
use core::time;
use std::io::{self, Write};
use std::time::Duration;

use crate::rcon::RCONConnection;

pub struct RCONShell<'a> {
    conn: &'a mut RCONConnection,
    prompt_chars: String, 

    //Used for generating a fancy little prelude to the prompt
    ip: String,
    port: u16,

    /// Current user input independent of what the terminal is printing
    current_input: String,
    history: Vec<String>
}

impl RCONShell<'_> {
    pub fn new(conn: &mut RCONConnection, prompt_chars: String, ip: String, port: u16) -> RCONShell {
        terminal::enable_raw_mode().unwrap();
        RCONShell {
            conn,
            prompt_chars,
            ip,
            port,
            current_input: String::new(),
            history: Vec::<String>::new(),
        }
    }

    pub async fn run(&mut self) {
        while self.poll_events().await {
            self.print();
        }
    }

    //enter the shell's loop, returns false if the loop should exit
    async fn poll_events(&mut self) -> bool {
        if poll(Duration::from_millis(50)).is_ok() {
            let event = read().unwrap();

            match event {
                Event::Key(KeyEvent { code, kind: KeyEventKind::Press, .. }) => {
                    match code {
                        KeyCode::Char(c) => {
                            self.current_input.push(c);
                        }
                        KeyCode::Backspace => {
                            self.current_input.pop();
                        }
                        KeyCode::Enter => {
                            let res = self.conn.send_command(&self.current_input).await.unwrap();
                            self.add_history_line(self.current_input.clone(), res);

                            self.current_input.clear();
                            
                        }
                        _ => {},
                    }
                }
                Event::Key(KeyEvent {code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL, ..})=>{
                    terminal::disable_raw_mode();
                    return false;
                }

                _ => {},
            }
        }

        return true;
    }

    //Unfortunately requires mutability due to our held reference of stdout
    fn print(&self) -> std::io::Result<()> {
        let mut stdout = io::stdout();

        execute!(stdout, MoveToColumn(0))?;
        execute!(stdout, Clear(ClearType::CurrentLine))?;

        execute!(stdout, SetForegroundColor(Color::Green))?;
        stdout.write(&self.gen_prompt_addr())?;

        execute!(stdout, SetForegroundColor(Color::White))?;
        stdout.write(self.prompt_chars.as_bytes())?;


        stdout.write(self.current_input.as_bytes())?;
        
        stdout.flush()?;

        Ok(())
    }

    fn add_history_line(&mut self, command: String, response: String) {
        let mut stdout = io::stdout();
        stdout.write(command.as_bytes());
        execute!(stdout, cursor::MoveDown(1));
        stdout.write(response.as_bytes());


        self.history.push(command);
    }

    fn gen_prompt_addr(&self) -> Vec<u8> {
        let prompt_string = String::from("[".to_owned() + &self.ip + ":" + self.port.to_string().as_str() + "] ");
        return prompt_string.as_bytes().to_vec();
    }

    ///Releases the terminal from raw mode. Should be called at the end of the application.
    pub fn release() {
        terminal::disable_raw_mode().unwrap();
    }
}
