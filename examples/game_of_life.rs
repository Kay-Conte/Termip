use std::{
    io::{stdin, stdout, Stdin, Stdout, Write},
    time::Instant,
    usize,
};

use termip::{
    events::{Event, KeyCode, KeyEvent},
    utils::{
        disable_raw_mode, enable_raw_mode, enter_alternate_buffer, erase_entire_screen, get_size,
        leave_alternate_buffer, move_cursor, read_batch, 
    }, 
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Cell {
    Alive,
    Dead,
}

impl Cell {
    fn value(&self) -> u16 {
        match self {
            Cell::Alive => 1,
            Cell::Dead => 0,
        }
    }
}

#[derive(Clone)]
struct Game {
    board: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            board: vec![Cell::Dead; (width * height) as usize],
            width,
            height,
        }
    }

    fn calc_idx(&self, x: usize, y: usize) -> Option<usize> {
        if y > self.height || x > self.width {
            return None;
        }

        Some(y * self.width + x)
    }

    fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        let idx = self.calc_idx(x, y)?;

        self.board.get(idx as usize)
    }

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        let idx = self.calc_idx(x, y)?;

        self.board.get_mut(idx as usize)
    }

    fn set(&mut self, x: usize, y: usize, cell: Cell) {
        let c = self.get_mut(x, y).expect("Index not in range");

        *c = cell;
    }

    fn new_state(&self, x: usize, y: usize) -> Cell {
        let mut neighbourhood = 0;

        for u in x.saturating_sub(1)..=x + 1 {
            for v in y.saturating_sub(1)..=y + 1 {
                if (u, v) == (x, y) {
                    continue;
                }

                if let Some(cell) = self.get(u, v) {
                    neighbourhood += cell.value();
                }
            }
        }

        let Some(cell) = self.get(x, y) else {
            panic!("X: {} Y: {} Width: {} Height: {}", x, y, self.width, self.height);
        };

        match neighbourhood {
            2 => cell.clone(),
            3 => Cell::Alive,
            _ => Cell::Dead,
        }
    }

    fn next_step(&mut self) {
        let buf = self.clone();

        for idx in 0..buf.board.len() {
            let x = idx % buf.width as usize;
            let y = idx / buf.width as usize;

            let cell = buf.new_state(x, y);
            self.set(x, y, cell);
        }
    }

    fn display(&self) -> String {
        let mut buf = String::new();

        for (idx, cell) in self.board.iter().enumerate() {
            if idx % self.width as usize == 0 && idx != 0 {
                buf.push_str("\n");
            }

            match cell {
                Cell::Alive => buf.push_str("█"),
                Cell::Dead => buf.push_str(" "),
            }
        }

        buf
    }
}

#[derive(Debug)]
struct Cursor {
    x: u16,
    y: u16,
    x_max: u16,
    y_max: u16,
}

impl Cursor {
    fn new(x_max: u16, y_max: u16) -> Self {
        Self {
            x: 1,
            y: 1,
            x_max,
            y_max,
        }
    }

    fn left(&mut self) {
        let e = self.x.saturating_sub(1);

        if e >= 1 {
            self.x = e;
        }
    }

    fn right(&mut self) {
        let e = self.x + 1;

        if e < self.x_max {
            self.x = e;
        }
    }

    fn up(&mut self) {
        let e = self.y.saturating_sub(1);

        if e >= 1 {
            self.y = e;
        }
    }

    fn down(&mut self) {
        let e = self.y + 1;

        if e < self.y_max {
            self.y = e;
        }
    }

    fn x(&self) -> u16 {
        self.x
    }

    fn y(&self) -> u16 {
        self.y
    }
}

enum State {
    Editing,
    Simulating,
}

impl State {
    fn toggle(&mut self) {
        match self {
            Self::Editing => *self = Self::Simulating,
            Self::Simulating => *self = Self::Editing,
        }
    }
}

fn un_setup(out: &mut Stdout, inp: &mut Stdin) -> std::io::Result<()> {
    disable_raw_mode(inp)?;
    leave_alternate_buffer(out)?;

    out.flush()?;

    Ok(())
}

fn setup(out: &mut Stdout, inp: &mut Stdin) -> std::io::Result<()> {
    move_cursor(out, 1, 1)?;

    enable_raw_mode(inp)?;

    enter_alternate_buffer(out)?;
    erase_entire_screen(out)?;


    out.flush()?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut out = stdout();
    let mut inp = stdin();

    setup(&mut out, &mut inp)?;
    out.flush()?;

    let (height, width) = get_size(&mut out)?;

    let mut cursor = Cursor::new(width, height);
    let mut game = Game::new(width as usize, height as usize);
    let mut state = State::Editing;

    let mut last_time: Instant;

    'outer: loop {
        last_time = Instant::now();

        let batch = read_batch(&mut inp)?;

        if batch.pressed(KeyCode::Char(' ')) {
           state.toggle(); 
        } 

        for ev in batch {
            match ev {
                Event::Key(KeyEvent {
                    code: KeyCode::LeftArrow,
                    ..
                }) => {
                    cursor.left();
                    move_cursor(&mut out, cursor.y(), cursor.x())?;
                    out.flush()?;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::RightArrow,
                    ..
                }) => {
                    cursor.right();
                    move_cursor(&mut out, cursor.y(), cursor.x())?;
                    out.flush()?;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::UpArrow,
                    ..
                }) => {
                    cursor.up();
                    move_cursor(&mut out, cursor.y(), cursor.x())?;
                    out.flush()?;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::DownArrow,
                    ..
                }) => {
                    cursor.down();
                    move_cursor(&mut out, cursor.y(), cursor.x())?;
                    out.flush()?;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('a'),
                    ..
                }) => {
                    game.set((cursor.x() - 1) as usize, (cursor.y() - 1) as usize, Cell::Alive);
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('d'),
                    ..
                }) => {
                    game.set((cursor.x() - 1) as usize, (cursor.y() - 1) as usize, Cell::Dead);
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('e'),
                    ..
                }) => {
                    game.next_step();
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    ..
                }) => {
                    break 'outer;
                }
                _ => {}
            }
        }

        move_cursor(&mut out, 1, 1)?;

        write!(out, "{}", game.display())?;

        move_cursor(&mut out, cursor.y(), cursor.x())?;

        out.flush()?;

        std::thread::sleep(Instant::now() - last_time);
    }

    un_setup(&mut out, &mut inp)?;

    Ok(())
}
