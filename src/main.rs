use term_size;
use termion;
use rand::Rng;
use crossterm::{QueueableCommand};
use ctrlc;

struct Game {
  board: Vec<Vec<char>>,
  delay: u64,
  peice: char,
}

impl Game {
  fn new(width: usize, height: usize, delay: u64, peice: char) -> Self {
    Self {
      board: vec![vec![' '; width]; height],
      delay,
      peice,
    }
  }

  fn display_board(&self) -> String {
    let mut res = String::new();
    for i in self.board.iter() {
      for (index, c) in i.iter().enumerate() {
        res.push(*c);
        if index == i.iter().len() {
          res.push('\n')
        }
      }
    }
    res
  }

  fn game_loop(&mut self) {
    std::io::stdout().queue(crossterm::cursor::Hide).unwrap();
    clear();
    loop {
      self.set_rand_pos();
      print!("{}", &self.display_board());
      sleep(self.delay);
      std::io::stdout().queue(crossterm::cursor::MoveTo(0, 0)).unwrap();
    }    
  }

  fn set_rand_pos(&mut self) {
    if self.board.iter().all(|b| *b == vec![self.peice; term_size::dimensions().unwrap().0]) {
      std::io::stdout().queue(crossterm::cursor::Show).unwrap();
      sleep(5000);
      std::process::exit(0);
    }
    'inner: loop {
      let mut rng = rand::thread_rng();
      let w = rng.gen_range(0, term_size::dimensions().unwrap().0);
      let h = rng.gen_range(0, term_size::dimensions().unwrap().1);
      if self.board[h][w] != self.peice {
        self.board[h][w] = self.peice;
        break 'inner;
      }     
    }
  }
}

fn clear() {
  println!("{}", termion::clear::All);
}

fn sleep(ms: u64) {
  std::thread::sleep(std::time::Duration::from_millis(ms));
}

fn main() {
  ctrlc::set_handler(move || {
    std::io::stdout().queue(crossterm::cursor::Show).unwrap();
    std::process::exit(0);
  }).expect("Error setting Ctrl-C handler");

  let mut game = Game::new(term_size::dimensions().unwrap().0, term_size::dimensions().unwrap().1, 0, '0');
  println!("{}", termion::clear::All);
  game.game_loop()
}

