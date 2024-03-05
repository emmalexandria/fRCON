use crossterm::execute;
use crossterm::style::{Attribute, Color, ContentStyle, SetStyle, Stylize};
use std::borrow::Cow;
use std::io::{self, Write};

use crate::games::{Game, GameMapper};
use crate::rcon::RCONConnection;

use reedline::{ExampleHighlighter, Prompt, PromptEditMode, PromptHistorySearch, Reedline, Signal};

pub struct RCONShell<'a> {
    conn: &'a mut RCONConnection,
    stdout: io::Stdout,

    command_fn: &'a dyn Fn() -> Vec<String>,
    response_fn: &'a dyn Fn(&str) -> Vec<(String, ContentStyle)>,

    line_editor: Reedline,
    prompt: RCONPrompt,
}

impl RCONShell<'_> {
    pub fn new(conn: &mut RCONConnection, game: Game, ip: String) -> RCONShell {
        let mut shell = RCONShell {
            conn,
            stdout: io::stdout(),
            command_fn: GameMapper::get_command_fn(&game),
            response_fn: GameMapper::get_response_fn(&game),

            line_editor: Reedline::create(),
            prompt: RCONPrompt::create(ip),
        };

        let mut highlighter = ExampleHighlighter::new((shell.command_fn)());
        highlighter.change_colors(
            nu_ansi_term::Color::LightYellow,
            nu_ansi_term::Color::White,
            nu_ansi_term::Color::White,
        );

        shell.line_editor = Reedline::create().with_highlighter(Box::new(highlighter));

        return shell;
    }

    pub async fn run(&mut self) -> io::Result<()> {
        println!("{}", "\nCTRL+C or CTRL+D to quit.");

        self.shell_loop().await?;
        Ok(())
    }

    async fn shell_loop(&mut self) -> std::io::Result<()> {
        loop {
            let sig = self.line_editor.read_line(&self.prompt);

            match sig {
                Ok(Signal::Success(buffer)) => {
                    let res = self.conn.send_command(&buffer).await?;
                    self.print_command_response(res)?;
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
        let response_lines = (self.response_fn)(&res);
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
        Color::Green
    }

    fn get_indicator_color(&self) -> Color {
        Color::White
    }

    fn right_prompt_on_last_line(&self) -> bool {
        return true;
    }
}
