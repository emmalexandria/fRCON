use crossterm::cursor::{MoveToColumn, MoveUp, SetCursorStyle};
use crossterm::style::{
    Attribute, Color, ContentStyle, ResetColor, SetAttribute, SetForegroundColor, SetStyle, Stylize,
};
use crossterm::terminal::Clear;
use crossterm::{execute, terminal, terminal::ClearType};
use std::io::{self, Write};

use crate::rcon::RCONConnection;

//Used to parse commands for this response, because the way it normally prints is very ugly
const UNKNOWN_COMMAND_RESPONSE: &str = "Unknown or incomplete command, see below for error";

pub struct RCONShell<'a> {
    conn: &'a mut RCONConnection,
    prompt_chars: String,
    //Held reference to stdout to flush once per loop
    stdout: io::Stdout,

    //Used for generating a fancy little prelude to the prompt
    ip: String,
    port: u16,
}

impl RCONShell<'_> {
    pub fn new(
        conn: &mut RCONConnection,
        prompt_chars: String,
        ip: String,
        port: u16,
    ) -> RCONShell {
        RCONShell {
            conn,
            prompt_chars,
            stdout: io::stdout(),
            ip,
            port,
        }
    }

    pub async fn run(&mut self) -> io::Result<()> {
        //Doesn't matter if this errors
        match execute!(self.stdout, SetCursorStyle::SteadyBar) {
            _ => {}
        }
        println!("{}", "\nCTRL+C or Q to quit.");
        println!("{}", "────────────────────");
        self.shell_loop().await?;
        Ok(())
    }

    async fn shell_loop(&mut self) -> std::io::Result<()> {
        loop {
            let prompt_len = self.print_prompt()?;

            let mut line = String::new();
            io::stdin().read_line(&mut line)?;

            let command = line.trim();
            if command == "Q" {
                break;
            }

            let last_lines = self.split_input(prompt_len, line.clone())?;

            execute!(self.stdout, MoveUp((last_lines) as u16))?;
            execute!(self.stdout, Clear(ClearType::FromCursorDown))?;

            let res = self.conn.send_command(&command).await?;
            self.add_history_line(command.to_string(), res)?;

            self.stdout.flush()?;
        }

        Ok(())
    }

    //returns the number of lines that a given input would be split over if printed next to the prompt
    fn split_input(&self, prompt_len: usize, line: String) -> std::io::Result<usize> {
        let term_width = terminal::size()?.0;

        if line.len() < term_width as usize - prompt_len {
            return Ok(1);
        }

        let mut line_count: usize = 0;
        let mut input = line.clone();

        let first_line_index: usize = term_width as usize - prompt_len;

        line_count += 1;
        input.replace_range(0..first_line_index as usize, "");

        while input.len() > 0 {
            if input.len() >= term_width.into() {
                line_count += 1;
                input.replace_range(0..(term_width) as usize, "");
            } else {
                line_count += 1;
                break;
            }
        }

        return Ok(line_count);
    }

    fn print_prompt(&mut self) -> std::io::Result<usize> {
        execute!(self.stdout, SetForegroundColor(Color::Green))?;
        self.stdout.write(&self.gen_prompt_addr())?;

        execute!(self.stdout, SetForegroundColor(Color::White))?;
        self.stdout.write(self.prompt_chars.as_bytes())?;

        execute!(self.stdout, Clear(ClearType::FromCursorDown))?;

        Ok(self.gen_prompt_addr().len() + self.prompt_chars.len())
    }

    fn parse_response_for_unknown_command(response: &str) -> bool {
        //check if the response says the command is unknown or incomplete to print it in a prettier way
        let response_len = UNKNOWN_COMMAND_RESPONSE.len();
        if response.len() > response_len {
            return &response[0..response_len] == UNKNOWN_COMMAND_RESPONSE;
        }

        return false;
    }

    ///Adds a line to the history vec and prints it to the screen.
    //There's a lot of ugliness in this function purely due to my desire to parse the unknown command response
    //and print something nicer
    fn add_history_line(&mut self, command: String, response: String) -> std::io::Result<()> {
        let response_lines = RCONShell::get_styled_response(response.clone());

        execute!(self.stdout, MoveToColumn(0), Clear(ClearType::CurrentLine))?;
        self.stdout.write(self.prompt_chars.as_bytes())?;
        self.stdout.write(command.as_bytes())?;
        self.stdout.write(&[b'\n'])?;

        //If the response length is greater than 0 (to avoid empty lines)
        if response.len() > 0 {
            //Go through the lines of the parsed response (either the response alone or the unknown command response styled)
            for (line, style) in response_lines {
                execute!(self.stdout, SetStyle(style))?;

                self.stdout.write(line.as_bytes())?;
                self.stdout.write(&[b'\n'])?;

                //Reset the color (only necessary if it was an error)
                execute!(
                    self.stdout,
                    ResetColor,
                    SetAttribute(Attribute::NoBold),
                    //For some ungodly reason, crossterm is applying what seems to be a random underline. Don't ask me.
                    SetAttribute(Attribute::NoUnderline)
                )?;
            }
        }
        self.stdout.write(&[b'\n'])?;

        Ok(())
    }

    ///If the reponse indicates the command is unknown or incomplete, this function returns
    ///a multiline version of the response with content styles to make
    ///the error clearer. Otherwise, it returns only the original response formatted in white
    fn get_styled_response(response: String) -> Vec<(String, ContentStyle)> {
        let mut response_lines = Vec::<(String, ContentStyle)>::new();

        if RCONShell::parse_response_for_unknown_command(&response) {
            let sections = response.split_at(UNKNOWN_COMMAND_RESPONSE.len());
            response_lines.push((sections.0.to_string(), ContentStyle::new().red().bold()));
            response_lines.push((sections.1.to_string(), ContentStyle::new().red()));
        } else {
            response_lines.push((response, ContentStyle::new().white()));
        }

        return response_lines;
    }

    fn gen_prompt_addr(&self) -> Vec<u8> {
        let prompt_string = String::from("[".to_owned() + &self.ip + "] ");
        return prompt_string.as_bytes().to_vec();
    }
}
