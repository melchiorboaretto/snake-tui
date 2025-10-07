mod apple;
mod snake;

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::Rect,
    style::{Color, Stylize},
    text::Line,
    widgets::{Block, Paragraph},
};

use std::time::{Duration, Instant};

use apple::*;
use snake::*;

#[derive(Default)]
pub struct App {
    score: u16,
    snake: Snake,
    apple: Apple,

    running: bool,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;

        let playable_area = terminal.size()?;
        let playable_area = Rect::new(0, 0, playable_area.width, playable_area.height);

        let temp_borders = Block::bordered().title("");
        let temp_borders = temp_borders.inner(playable_area);

        let playable_area_x = temp_borders.width;
        let playable_area_y = temp_borders.height;

        let snake_x = playable_area_x / 2;
        let snake_y = playable_area_y / 2;

        self.snake = Snake::new(snake_x, snake_y);
        self.apple = Apple::spawn((playable_area_x, playable_area_y));

        const TICK_RATE: Duration = Duration::from_millis(1000 / 60);

        while self.running {

            let time_zero = Instant::now();

            let playable_area = terminal.size()?;
            let playable_area = Rect::new(0, 0, playable_area.width, playable_area.height);

            let temp_borders = Block::bordered().title("");
            let temp_borders = temp_borders.inner(playable_area);

            let playable_area_x = temp_borders.width;
            let playable_area_y = temp_borders.height;

            if let Some(()) = self.snake.move_or_die() {
                if self.snake.body[0].0 > playable_area_x - 1
                    || self.snake.body[0].1 > playable_area_y - 1
                {
                    self.running = false;
                }
            } else {
                self.running = false;
            }

            if self.apple.position() == self.snake.body[0] {
                self.snake.eat();
                self.apple = Apple::spawn((playable_area_x, playable_area_y));
                self.inc_score();
            }

            terminal.draw(|frame| self.render(frame))?;

            let elapsed_time = time_zero.elapsed();
            let remaining_time = TICK_RATE.saturating_sub(elapsed_time);

            self.handle_crossterm_events(remaining_time)?;
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        // Borders rendering
        let frame_area = frame.area();
        let title = format!("Speed: {}          Score: {}", self.snake.speed, self.score,);

        let title = Line::from(title).bold().fg(Color::Indexed(11)).centered();

        let game_borders = Block::bordered().title(title);
        let map_rect = game_borders.inner(frame_area);

        frame.render_widget(game_borders, frame.area());

        // Game grid rendering
        let map_width = map_rect.width as usize;
        let map_height = map_rect.height as usize;

        let mut map_to_render = vec![vec![' '; map_width]; map_height];

        // Apple drawing
        let apple_coords = self.apple.position();

        map_to_render[apple_coords.1 as usize][apple_coords.0 as usize] = '$';

        // Snake drawing
        for (index, segment) in self.snake.body.iter().enumerate() {
            if (segment.1 as usize) < map_height && (segment.0 as usize) < map_width as usize {
                if index == 0 {
                    map_to_render[segment.1 as usize][segment.0 as usize] = '@';
                } else {
                    map_to_render[segment.1 as usize][segment.0 as usize] = '#';
                }
            }
        }

        let drawable_map: Vec<Line> = map_to_render
            .into_iter()
            .map(|row| Line::from(String::from_iter(row)))
            .collect();

        let drawable_map = Paragraph::new(drawable_map);

        frame.render_widget(drawable_map, map_rect)
    }

    fn handle_crossterm_events(&mut self, timeout: Duration) -> Result<()> {
        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
                _ => {}
            }
        }
        Ok(())
    }

    fn on_key_event(&mut self, key: KeyEvent) {
        if let KeyCode::Char(input) = key.code {
            match input.to_ascii_lowercase() {
                'q' => self.quit(),
                '[' => self.snake.dec_speed(),
                ']' => self.snake.inc_speed(),
                _ => {}
            }
        } else if self.snake.dir.change_direction_no_reverse_arrow(key.code) {
            self.snake.changing_dir = true;
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }

    fn inc_score(&mut self) {
        self.score = self.score.saturating_add(1);
    }
}
