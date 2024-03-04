use crossterm::execute;
use crossterm::style::{Attribute, Color, ContentStyle, SetStyle, Stylize};
use std::borrow::Cow;
use std::io::{self, Write};

use crate::minecraft::responses::MinecraftResponse;
use crate::rcon::RCONConnection;
use crate::response::Response;

use reedline::{Prompt, PromptEditMode, PromptHistorySearch, Reedline, Signal};

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
        println!("{}", "\nCTRL+C or CTRL+D to quit. \n");

        self.shell_loop().await?;
        Ok(())
    }

    async fn shell_loop(&mut self) -> std::io::Result<()> {
        loop {
            let sig = self.line_editor.read_line(&self.prompt);

            match sig {
                Ok(Signal::Success(buffer)) => {
                    let res = self.conn.send_command(&buffer).await?;
                    self.print_command_response(res);
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

    fn print_command_response(&mut self, res: String) -> std::io::Result<()> {
        let res_type = MinecraftResponse::get_from_response_str(&res);
        let response_lines = res_type.get_output(res.clone());
        for line in response_lines {
            let line_with_newline: String = line.0 + "\n";
            execute!(
                self.stdout,
                SetStyle(ContentStyle::new().attribute(Attribute::Reset))
            )?;
            execute!(self.stdout, SetStyle(line.1))?;
            self.stdout.write(line_with_newline.as_bytes())?;
        }
        execute!(
            self.stdout,
            SetStyle(ContentStyle::new().attribute(Attribute::Reset))
        )?;
        self.stdout.flush()?;

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
