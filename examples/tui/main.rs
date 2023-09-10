mod command;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::*,
    widgets::*,
};
use rubik::{
    parser::singmaster,
    prelude::{CubeFace, RubikColor},
    transform::RubikTransform,
    Rubik, RubikLayer,
};
use std::{
    collections::VecDeque,
    error::Error,
    io::{self, Stdout},
    time::Duration,
};

use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;

#[derive(Debug, Default)]
pub struct InputComponent {
    input: Input,
    history: Vec<String>,
    buffer: Option<String>,
    history_cursor: usize,
    state: InputComponentState,
}
#[derive(Debug, Default)]
pub enum InputComponentState {
    #[default]
    Input,
    History {
        buffered_input: Input,
        idx: usize,
    },
}
impl InputComponent {
    pub fn set(&mut self, s: String) {
        let new_input = self.input.clone().with_value(s);
        self.input = new_input;
    }
    pub fn prev(&mut self, offset: usize) {
        if self.history.is_empty() {
            return;
        }
        self.history_cursor = self.history_cursor.wrapping_sub(offset);
        self.set_by_history_cursor()
    }
    pub fn next(&mut self, offset: usize) {
        let new_history_cursor = self.history_cursor + offset;
        if new_history_cursor >= self.history.len() {
            self.history_cursor = self.history.len();
            if let Some(s) = self.buffer.take() {
                self.set(s)
            }
        } else {
            self.set_by_history_cursor()
        }
        self.history_cursor = new_history_cursor;
    }
    pub fn set_by_history_cursor(&mut self) {
        if self.history.is_empty() {
            return;
        }
        self.buffer.get_or_insert(self.input.to_string());
        self.set(self.history[self.history_cursor].to_string())
    }
    pub fn enter(&mut self) -> &str {
        let val = self.input.to_string();
        self.set(val.clone());
        self.history.push(val);
        self.history_cursor = self.history.len();
        self.input.reset();
        self.history.last().expect("history is empty")
    }
}
#[derive(Debug, Default)]
pub struct RubikComponent {
    cube: Rubik,
    history: VecDeque<RubikTransform>,
}

impl RubikComponent {
    pub fn execute(&mut self, tf: &RubikTransform) -> &mut Self {
        self.cube.execute(tf);
        self
    }
    pub fn revoke(&mut self) -> &mut Self {
        if let Some(latest_op) = self.history.pop_back() {
            self.cube.execute(&latest_op.inverse());
        }
        self
    }
    pub fn clear(&mut self) -> &mut Self {
        self.cube.reset();
        self
    }
}

impl Widget for &RubikComponent {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // 12 * 9
        if area.width < 12 || area.height < 9 {
            return;
        }
        let color_map = RubikColor::classic_map();
        fn print_color(c: RubikColor, aligned: bool) -> Span<'static> {
            let mut span = Span::from("  ");
            if aligned {
                span = span.slow_blink()
            };
            match c {
                RubikColor::White => span.on_white(),
                RubikColor::Yellow => span.on_yellow(),
                RubikColor::Red => span.on_red(),
                RubikColor::Orange => span.on_light_magenta(),
                RubikColor::Blue => span.on_blue(),
                RubikColor::Green => span.on_green(),
            }
        }
        let (mut x, mut y) = (area.x, area.y);
        let mut counter = 0;
        let rubik = &self.cube;
        for cube in rubik.iter_by_layer(&RubikLayer::U) {
            if counter % 3 == 0 {
                x = area.x + 6;
            }
            let color = color_map[cube.get(CubeFace::U)];
            buf.set_span(x, y, &print_color(color, rubik.is_aligned(cube)), 2);
            counter += 1;
            x += 2;
            if counter % 3 == 0 {
                y += 1;
            }
        }
        for (block_cnt, (layer, face)) in [
            (&RubikLayer::L, CubeFace::L),
            (&RubikLayer::F, CubeFace::F),
            (&RubikLayer::R, CubeFace::R),
            (&RubikLayer::B, CubeFace::B),
        ]
        .into_iter()
        .enumerate()
        {
            counter = 0;
            for cube in rubik.iter_by_layer(layer) {
                if counter % 3 == 0 {
                    x = area.x + (block_cnt as u16) * 6;
                }
                let color = color_map[cube.get(face)];
                buf.set_span(x, y, &print_color(color, rubik.is_aligned(cube)), 2);
                counter += 1;
                x += 2;
                if counter % 3 == 0 {
                    y += 1;
                }
            }
            y -= 3;
        }
        y += 3;
        counter = 0;
        for cube in rubik.iter_by_layer(&RubikLayer::D) {
            if counter % 3 == 0 {
                x = area.x + 6;
            }
            let color = color_map[cube.get(CubeFace::D)];
            buf.set_span(x, y, &print_color(color, rubik.is_aligned(cube)), 2);
            counter += 1;
            x += 2;
            if counter % 3 == 0 {
                y += 1;
            }
        }
    }
}

#[derive(Debug, Default)]
pub enum AppState {
    Exit(isize),
    #[default]
    Running,
}
#[derive(Debug, Default)]
pub struct App {
    input: InputComponent,
    rubik: RubikComponent,
    history: Vec<String>,
    hint: String,
    state: AppState,
}

impl App {
    pub fn handle_input(&mut self, input: &str) {
        if let Some(cmd) = input.strip_prefix('/') {
            match cmd {
                "revoke" | "r" => {
                    self.rubik.revoke();
                }
                "clear" | "c" | "reset" => {
                    self.rubik.clear();
                }
                "exit" | "e" => {
                    self.state = AppState::Exit(0);
                }
                "shuffle" | "s" => {
                    let _shuffle = self.rubik.cube.shuffle(32);
                }
                _ => {}
            }
        } else if let Ok(transform) = singmaster::parse(input) {
            self.history.push(input.to_owned());
            self.rubik.execute(&transform);
        } else {
            self.hint = "invalid input".to_string();
        }
    }
}
/*

History | Cube
_______________
input:

*/

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = setup_terminal()?;
    run(&mut terminal)?;
    restore_terminal(&mut terminal)?;
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    Ok(terminal.show_cursor()?)
}

fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn Error>> {
    let mut app: App = App::default();
    loop {
        terminal.draw(|frame| renderer(frame, &app))?;
        'poll_evt: {
            if event::poll(Duration::from_millis(250))? {
                let evt = event::read()?;
                match evt {
                    Event::Key(key) => match key.code {
                        KeyCode::Enter => {
                            let input_val = app.input.enter().to_owned();
                            app.handle_input(&input_val);
                            break 'poll_evt;
                        }
                        KeyCode::Up => {
                            app.input.prev(1);
                            break 'poll_evt;
                        }
                        KeyCode::Down => {
                            app.input.next(1);
                            break 'poll_evt;
                        }
                        _ => {}
                    },
                    Event::Mouse(_) => {}
                    _ => {}
                }
                app.input.input.handle_event(&evt);
            }
        }
        match app.state {
            AppState::Exit(_) => return Ok(()),
            AppState::Running => {
                continue;
            }
        }
    }
}

fn renderer<B: ratatui::backend::Backend>(frame: &mut Frame<B>, app: &App) {
    let v_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([Constraint::Min(13), Constraint::Length(3)].as_ref())
        .split(frame.size());
    let main_block = v_chunks[0];
    let h_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([Constraint::Min(24), Constraint::Max(24)].as_ref())
        .split(main_block);
    let rubik_rect = h_chunks[0];
    let history_rect = h_chunks[1];
    let cmd_rect = v_chunks[1];
    let rubik_out = Block::default()
        .title("Rubik's Cube")
        .borders(Borders::all())
        .border_style(Style::default().fg(Color::White))
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::Black));
    let history_out = Block::default()
        .title("History")
        .borders(Borders::all())
        .border_style(Style::default().fg(Color::White))
        .border_type(BorderType::Rounded)
        .style(Style::default().bg(Color::Black));
    let input_width = cmd_rect.width.max(3) - 3; // keep 2 for borders and 1 for cursor

    let scroll = app.input.input.visual_scroll(input_width as usize);
    let cmd_out = Paragraph::new(app.input.input.value())
        .scroll((0, scroll as u16))
        .block(Block::default().borders(Borders::ALL).title("Input"));
    frame.render_widget(rubik_out, rubik_rect);
    frame.render_widget(&app.rubik, rubik_rect.inner(&Margin::new(2, 3)));
    frame.render_widget(history_out, history_rect);
    frame.render_widget(
        List::new(
            app.history
                .iter()
                .enumerate()
                .rev()
                .take((history_rect.height - 2) as usize)
                .rev()
                .map(|(idx, cmd)| {
                    ListItem::new(Line::from(vec![
                        Span::from(idx.to_string()).set_style(Style::new().on_blue()),
                        Span::from(" "),
                        Span::from(cmd),
                    ]))
                })
                .collect::<Vec<_>>(),
        ),
        history_rect.inner(&Margin::new(1, 1)),
    );
    frame.set_cursor(
        // Put cursor past the end of the input text
        cmd_rect.x + ((app.input.input.visual_cursor()).max(scroll) - scroll) as u16 + 1,
        // Move one line down, from the border to the input line
        cmd_rect.y + 1,
    );
    frame.render_widget(cmd_out, cmd_rect);
}
