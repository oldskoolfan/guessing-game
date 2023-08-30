use std::{io, cmp::Ordering};
use rand::Rng;

struct Game {
  secret_number: u32,
  guess: Option<u32>,
}

impl Game {
  fn get_guess(&mut self) -> bool {
    let mut guess = String::new();

    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => return false,
    };

    self.guess = Some(guess);

    true
  }

  fn check_guess(&self) -> bool {
    match self.guess.unwrap().cmp(&self.secret_number) {
      Ordering::Less => self.too_small(),
      Ordering::Greater => self.too_big(),
      Ordering::Equal => self.win(),
    }
  }

  fn too_small(&self) -> bool {
    println!("Too small!");
    false
  }

  fn too_big(&self) -> bool {
    println!("Too big!");
    false
  }

  fn win(&self) -> bool {
    println!("You win!");
    true
  }

  fn init_game() -> Self {
    println!("Guess a number between 1 and 100.");

    Self {
      secret_number: rand::thread_rng().gen_range(1..=100),
      guess: None,
    }
  }
}

pub fn main_loop() {
  let mut game = Game::init_game();

  loop {
    println!("Please input your guess.");

    if !game.get_guess() {
      continue;
    }

    if game.check_guess() {
      break;
    }
  }
}
