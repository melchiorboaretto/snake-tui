use crossterm::event::KeyCode;

use std::collections::VecDeque;

#[derive(Debug, Default)]
pub struct Snake {
    pub body: VecDeque<(u16, u16)>,
    pub speed: u16,
    pub tick_counter: u16,
    pub dir: Direction,
    pub has_eaten: bool,
    pub changing_dir: bool,
}

impl Snake {
    pub fn new(x: u16, y: u16) -> Self {
        let mut body = VecDeque::from([(x, y)]);
        body.push_back((x - 1, y));
        Snake {
            body,
            speed: 1,
            tick_counter: 0,
            dir: Direction::default(),
            has_eaten: false,
            changing_dir: false,
        }
    }

    // If None, snake is dead.
    // But the upper limits must be checked from an outer function.
    pub fn move_or_die(&mut self) -> Option<()> {
        use Direction::*;
        let speed = self.speed;
        const SPEED_TUNER: u16 = 20;
        let speed_tuning: u16 = SPEED_TUNER
            * match &self.dir {
                Right | Left => 1,
                Up | Down => 2,
            };

        self.tick_counter = self.tick_counter.wrapping_add(1);
        if self.speed != 0 && self.tick_counter.is_multiple_of((speed_tuning / speed).max(1)) {
            let head_ref = self.body.front();
            let head: (i32, i32);
            if let Some(value) = head_ref {
                let head_x = value.0 as i32;
                let head_y = value.1 as i32;
                head = (head_x, head_y);
            } else {
                return None;
            }

            // Suppose head exists and its coords as i32 are in head
            let dxdy = self.dir.to_delta_vec();

            let new_head_x = head.0 + dxdy.0;
            let new_head_y = head.1 + dxdy.1;

            // Test if the snake hit the wall
            if (new_head_x < 0 || new_head_x > u16::MAX.into())
                || (new_head_y < 0 || new_head_y > u16::MAX.into())
            {
                return None;
            }

            // Suppose new_head_x and new_head_y are both greater than or eq. to zero.
            let new_head = (new_head_x as u16, new_head_y as u16);

            self.body.push_front(new_head);

            // Pops the snake tail
            if !self.has_eaten {
                self.body.pop_back();
            } else {
                self.has_eaten = false;
            }

            // Test if the snake hit itself
            if !self.changing_dir {
                for segment in self.body.iter().skip(1) {
                    if new_head == *segment {
                        return None;
                    }
                }
                self.changing_dir = false;
            }
        }

        Some(())
    }

    pub fn inc_speed(&mut self) {
        self.speed = self.speed.saturating_add(1);
    }
    pub fn dec_speed(&mut self) {
        self.speed = self.speed.saturating_sub(1);
    }

    pub fn eat(&mut self) {
        self.has_eaten = true;
    }
}

#[derive(Debug, Default, PartialEq)]
pub enum Direction {
    #[default]
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    pub fn change_direction(&mut self, new_direction: Direction) {
        *self = new_direction;
    }

    pub fn change_direction_no_reverse(&mut self, new_direction: Direction) {
        use Direction::*;
        match *self {
            Right if new_direction != Left => self.change_direction(new_direction),
            Left if new_direction != Right => self.change_direction(new_direction),
            Up if new_direction != Down => self.change_direction(new_direction),
            Down if new_direction != Up => self.change_direction(new_direction),
            _ => (),
        };
    }

    pub fn change_direction_no_reverse_arrow(&mut self, new_dir: KeyCode) -> bool {
        use Direction::*;
        match new_dir {
            KeyCode::Right => {self.change_direction_no_reverse(Right); true},
            KeyCode::Left => {self.change_direction_no_reverse(Left); true},
            KeyCode::Up => {self.change_direction_no_reverse(Up); true},
            KeyCode::Down => {self.change_direction_no_reverse(Down); true},
            _ => false,
        }
    }

    // Returns the direction in the (dx, dy) form
    pub fn to_delta_vec(&self) -> (i32, i32) {
        use Direction::*;
        match self {
            Right => (1, 0),
            Left => (-1, 0),
            Up => (0, -1),
            Down => (0, 1),
        }
    }
}
