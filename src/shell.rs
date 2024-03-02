use crossterm::cursor::{MoveLeft, MoveToColumn, SetCursorStyle};
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::style::{Color, SetForegroundColor};
use crossterm::terminal::Clear;
use crossterm::{execute, terminal, terminal::ClearType};
use std::io::{self, Write};
use std::time::Duration;

use crate::rcon::RCONConnection;

const UNKNOWN_COMMAND_RESPONSE: &str = "Unknown or incomplete command, see below for error";

pub struct RCONShell<'a> {
    conn: &'a mut RCONConnection,
    prompt_chars: String,
    stdout: io::Stdout,

    //Used for generating a fancy little prelude to the prompt
    ip: String,
    port: u16,

    /// Current user input independent of what the terminal is printing
    current_input: String,
    /// Line history (used for ability to seek through past commands with arrows)
    history: Vec<String>,
    history_offset: usize,
    /// Stores the negative offset of the cursor from the end of current_input for the purpose of line editing
    cursor_offset: u16,
}

impl RCONShell<'_> {
    pub fn new(
        conn: &mut RCONConnection,
        prompt_chars: String,
        ip: String,
        port: u16,
    ) -> RCONShell {
        terminal::enable_raw_mode().unwrap();
        RCONShell {
            conn,
            prompt_chars,
            stdout: io::stdout(),

            ip,
            port,
            current_input: String::new(),
            history: Vec::<String>::new(),
            cursor_offset: 0,
            history_offset: 0,
        }
    }

    pub async fn run(&mut self) -> io::Result<()> {
        //Doesn't matter if this errors
        match execute!(self.stdout, SetCursorStyle::SteadyBar) {
            _ => {}
        }

        while self.poll_events().await? {
            self.print()?;
        }
        RCONShell::release();
        Ok(())
    }

    //enter the shell's loop, returns false if the loop should exit
    async fn poll_events(&mut self) -> std::io::Result<bool> {
        if poll(Duration::from_millis(50)).is_ok() {
            let event = read().unwrap();

            match event {
                Event::Key(KeyEvent {
                    code,
                    kind: KeyEventKind::Press,
                    ..
                }) => return self.handle_char_events(code).await,
                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    ..
                }) => {
                    return Ok(false);
                }

                _ => {}
            }
        }

        return Ok(true);
    }

    ///Where all character based shell input is handled. Inherits the return type of poll_events to give it the ability to 
    ///arbitrarily cause the loop to exit
    async fn handle_char_events(&mut self, code: KeyCode) -> std::io::Result<bool> {
        match code {
            KeyCode::Char(c) => {
                self.current_input.push(c);
            }
            KeyCode::Backspace => {
                self.current_input.pop();
            }
            KeyCode::Left => {
                if usize::from(self.cursor_offset) < self.current_input.len() {
                    self.cursor_offset += 1;
                }
            }
            KeyCode::Right => {
                if self.cursor_offset > 0 {
                    self.cursor_offset -= 1;
                }
            }
            KeyCode::Up => {
                if self.history_offset < self.history.len() {
                    self.history_offset += 1;
                    self.current_input =
                        self.history[self.history.len() - self.history_offset].clone();
                }
            }
            KeyCode::Down => {
                if self.history_offset > 0 {
                    self.history_offset -= 1;
                }

                if self.history_offset == 0 {
                    self.current_input.clear();
                } else {
                    self.current_input =
                        self.history[self.history.len() - self.history_offset].clone();
                }
            }
            KeyCode::Enter => {
                let res = self.conn.send_command(&self.current_input).await.unwrap();
                match self.add_history_line(self.current_input.clone(), res) {
                    Ok(_) => {}
                    Err(_) => {
                        if self
                            .print_friendly_error("Failed to add line to history")
                            .is_err()
                        {
                            //The decision to exit the shell if this function fails is explained above its definition.
                            //Someone tell me if its a stupid decision
                            return Ok(false);
                        }
                    }
                }
                self.cursor_offset = 0;
                self.current_input.clear();
            }
            _ => {}
        }
        Ok(true)
    }

    //Unfortunately requires mutability due to our held reference of stdout
    fn print(&mut self) -> std::io::Result<()> {
        execute!(self.stdout, MoveToColumn(0))?;
        execute!(self.stdout, Clear(ClearType::CurrentLine))?;

        execute!(self.stdout, SetForegroundColor(Color::Green))?;
        self.stdout.write(&self.gen_prompt_addr())?;

        execute!(self.stdout, SetForegroundColor(Color::White))?;
        self.stdout.write(self.prompt_chars.as_bytes())?;

        self.stdout.write(self.current_input.as_bytes())?;

        //must check if greater than 0, because MoveLeft(0) still moves left one
        if self.cursor_offset > 0 {
            execute!(self.stdout, MoveLeft(self.cursor_offset))?;
        }

        self.stdout.flush()?;

        Ok(())
    }

    fn parse_response_for_unknown_command(response: &str) -> bool {
        //check if the response says the command is unknown or incomplete to print it in a prettier way
        let response_len = UNKNOWN_COMMAND_RESPONSE.len();
        if response.len() > response_len {
            return &response[0..response_len] == UNKNOWN_COMMAND_RESPONSE;
        }

        return false;
    }

    fn add_history_line(&mut self, command: String, response: String) -> std::io::Result<()> {
        let mut parsed_response = response.clone();
        let unknown_command: bool = RCONShell::parse_response_for_unknown_command(&response);
        if unknown_command {
            parsed_response = response[0..UNKNOWN_COMMAND_RESPONSE.len()].to_string()
                + "\n"
                + &response[UNKNOWN_COMMAND_RESPONSE.len()..];
        }

        execute!(self.stdout, MoveToColumn(0), Clear(ClearType::CurrentLine))?;
        self.stdout.write(self.prompt_chars.as_bytes())?;
        self.stdout.write(command.as_bytes())?;
        if response.len() > 0 {
            self.stdout.write(&[b'\n'])?;
            if unknown_command {
                execute!(self.stdout, SetForegroundColor(Color::Red))?;
            }
            self.stdout.write(parsed_response.as_bytes())?;
        }
        self.stdout.write(&[b'\n'])?;

        execute!(self.stdout, SetForegroundColor(Color::White))?;

        self.history.push(command);

        Ok(())
    }

    ///To be used when an operation fails but its not a big deal
    ///If this fails, assume something is wrong with our ability to print and panic
    // Panic because the majority of our potential ignorable errors are generated from terminal interactions
    // Given that all this function does is write to the terminal in red, if we get an error, ignore it and print
    // a user friendly message, and then this function errors, something must be up.
    fn print_friendly_error(&mut self, output: &str) -> std::io::Result<()> {
        execute!(self.stdout, SetForegroundColor(Color::Red))?;
        self.stdout.write(output.as_bytes())?;
        self.stdout.write(&[b'\n'])?;

        Ok(())
    }

    fn gen_prompt_addr(&self) -> Vec<u8> {
        let prompt_string =
            String::from("[".to_owned() + &self.ip + ":" + self.port.to_string().as_str() + "] ");
        return prompt_string.as_bytes().to_vec();
    }

    ///Releases the terminal from raw mode. Should be called at the end of the application.
    pub fn release() {
        terminal::disable_raw_mode().unwrap();
    }
}
