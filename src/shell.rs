use crossterm::cursor::{MoveToColumn, MoveUp, SetCursorStyle};
use crossterm::style::{
    Attribute, Color, ContentStyle, ResetColor, SetAttribute, SetForegroundColor, SetStyle, Stylize,
};
use crossterm::terminal::Clear;
use crossterm::{execute, terminal, terminal::ClearType};
use std::borrow::Cow;
use std::io::{self, Write};

use crate::minecraft::responses::{self, Response};
use crate::rcon::RCONConnection;

use reedline::{DefaultPrompt, Prompt, PromptEditMode, PromptHistorySearch, Reedline, Signal};

pub struct RCONShell<'a> {
    conn: &'a mut RCONConnection,
    //Held reference to stdout to flush once per loop
    stdout: io::Stdout,

    //Used for generating a fancy little prelude to the prompt
    ip: String,

    line_editor: Reedline,
    prompt: RCONPrompt,
}

impl RCONShell<'_> {
    pub fn new(conn: &mut RCONConnection, ip: String) -> RCONShell {
        RCONShell {
            conn,
            stdout: io::stdout(),
            ip: ip.clone(),

            line_editor: Reedline::create(),
            prompt: RCONPrompt::create(ip),
        }
    }

    pub async fn run(&mut self) -> io::Result<()> {
        println!("{}", "\nCTRL+C or CTRL+D to quit.");
        println!("{}", "────────────────────");
        self.shell_loop().await?;
        Ok(())
    }

    async fn shell_loop(&mut self) -> std::io::Result<()> {
        loop {
            let sig = self.line_editor.read_line(&self.prompt);

            match sig {
                Ok(Signal::Success(buffer)) => {
                    let res = self.conn.send_command(&buffer).await?;
                    let res_type = Response::get_from_response_str(&res);
                    let response_lines = res_type.get_output(res.clone());
                    for line in response_lines {
                        execute!(
                            self.stdout,
                            SetStyle(ContentStyle::new().attribute(Attribute::Reset))
                        )?;
                        execute!(self.stdout, SetStyle(line.1))?;
                        println!("{}", line.0);
                        //self.stdout.write(line.0.as_bytes())?;
                        // self.stdout.write(&[b'\n'])?;
                    }
                    execute!(
                        self.stdout,
                        SetStyle(ContentStyle::new().attribute(Attribute::Reset))
                    )?;
                    self.stdout.flush()?;
                }
                Ok(Signal::CtrlD) | Ok(Signal::CtrlC) => {
                    println!("Exiting...");
                    break;
                }
                _ => {}
            }
        }

        Ok(())
    }
}

struct RCONPrompt {
    prompt: String,
    left: String,
}

impl RCONPrompt {
    fn create(ip: String) -> RCONPrompt {
        return RCONPrompt {
            prompt: " >>".to_string(),
            left: "[".to_string() + &ip + "]",
        };
    }
}

impl Prompt for RCONPrompt {
    fn render_prompt_left(&self) -> Cow<'_, str> {
        return Cow::from(self.left.as_str());
    }

    fn render_prompt_right(&self) -> Cow<'_, str> {
        let right =
            "[".to_string() + chrono::Utc::now().format("%H:%M:%S").to_string().as_str() + "]";
        return Cow::Owned(right);
    }

    //I think this is technically abusing reedline a bit. This is meant to render an indicator, but I'm using it to render my prompt characters differently
    fn render_prompt_indicator(&self, _: PromptEditMode) -> Cow<'_, str> {
        return Cow::Borrowed(self.prompt.as_str());
    }

    fn render_prompt_multiline_indicator(&self) -> Cow<'_, str> {
        return Cow::Borrowed("");
    }

    fn render_prompt_history_search_indicator(&self, _: PromptHistorySearch) -> Cow<'_, str> {
        return Cow::Borrowed("");
    }

    fn get_prompt_right_color(&self) -> Color {
        Color::White
    }

    fn get_prompt_color(&self) -> Color {
        Color::DarkGreen
    }

    fn get_indicator_color(&self) -> Color {
        Color::Green
    }

    fn right_prompt_on_last_line(&self) -> bool {
        return true;
    }
}
