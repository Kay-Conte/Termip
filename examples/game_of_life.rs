#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Cell {
    Alive,
    Dead,
}

impl Cell {
    fn value(&self) -> usize {
        match self {
            Cell::Alive => 1,
            Cell::Dead => 0,
        }
    }
}

struct Game<const U: usize> {
    board: [Cell; U],
    width: usize,
    height: usize,
}

impl<const U: usize> Game<U> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            board: [Cell::Dead; U],
            width,
            height,
        }
    }

    fn calc_idx(&self, x: usize, y: usize) -> Option<usize> {
        if y > self.height {
            return None;
        }

        Some(x * self.width + y)
    }

    fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        let idx = self.calc_idx(x, y)?;

        Some(&self.board[idx])
    }

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        let idx = self.calc_idx(x, y)?;

        Some(&mut self.board[idx])
    }

    fn should_live(&mut self, x: usize, y: usize) -> bool {
        let mut neighbourhood = 0;

        for u in x.saturating_sub(1)..=x + 1 {
            for v in y.saturating_sub(1)..=y + 1 {
                if let Some(cell) = self.get(u, v) {
                    neighbourhood += 1
                }
            }
        } 

        true
    }

    fn next_step(&mut self) {
        
    } 
}

fn main() {

}
