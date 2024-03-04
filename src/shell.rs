use crossterm::execute;
use crossterm::style::{Attribute, Color, ContentStyle, SetStyle, Stylize};
use std::borrow::Cow;
use std::io::{self, Write};
use std::marker::PhantomData;

use crate::game_mapper::{self, Commands, GameMap, GameMapper, Response};
use crate::minecraft::responses::MinecraftResponse;
use crate::rcon::RCONConnection;

use reedline::{ExampleHighlighter, Prompt, PromptEditMode, PromptHistorySearch, Reedline, Signal};

pub struct RCONShell<'a, Game, Res> {
    conn: &'a mut RCONConnection,
    //Held reference to stdout to flush once per loop
    stdout: io::Stdout,
    game: PhantomData<Game>,
    res: PhantomData<Res>,

    //Used for generating a fancy little prelude to the prompt
    line_editor: Reedline,
    prompt: RCONPrompt,
}

impl<Game, Res> RCONShell<'_, Game, Res>
where
    Game: GameMap,
    Res: Response,
{
    pub fn new(conn: &mut RCONConnection, ip: String) -> RCONShell<Game, Res> {
        let mut shell = RCONShell {
            conn,
            stdout: io::stdout(),
            game: PhantomData,
            res: PhantomData,

            line_editor: Reedline::create(),
            prompt: RCONPrompt::create(ip),
        };

        //had to do this dumb stuff to get rust to stop complaining at me for some reason. might have just been a formatting mistake trying to do
        //it inline in the structure

        //Using the example highlighter is fine for now. Implementing custom highlighters to match the command systems of games
        //would be a lot of work for an admittably cool result, but for now its not going to be done.
        let mut highlighter =
            ExampleHighlighter::new(game_mapper::GameMapper::<Game, Res>::get_commands());
        highlighter.change_colors(
            nu_ansi_term::Color::Green,
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
        let res_type = MinecraftResponse::from_response_str(&res);
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
        Color::Green
    }

    fn get_indicator_color(&self) -> Color {
        Color::White
    }

    fn right_prompt_on_last_line(&self) -> bool {
        return true;
    }
}
