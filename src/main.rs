mod dict;

use std::{fmt::Display, io::{self, Write}};
use rand::{thread_rng, Rng};

use dict::DICT;

#[derive(PartialEq, Eq, Debug)]
enum GuessChar {
    Absent,
    OutOfPlace,
    Correct,
    OutOfBounds,
}

impl Display for GuessChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GuessChar::Absent => write!(f, "⬛"),
            GuessChar::OutOfPlace => write!(f, "🟨"),
            GuessChar::Correct => write!(f, "🟩"),
            GuessChar::OutOfBounds => write!(f, "❌"),
        }
    }
}

/// Guess represents a complete guessed word, made up of a list of guessed
/// charaters.
struct Guess {
    inner: Vec<GuessChar>
}

impl Display for Guess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for gc in &self.inner {
            write!(f, "{}", gc)?;
        }
        Ok(())
    }
}

impl From<Vec<GuessChar>> for Guess {
    fn from(guess: Vec<GuessChar>) -> Self {
        Self {
            inner: guess,
        }
    }
}

impl Guess {
    fn correct(self) -> bool {
        self.inner.iter().all(|r| r == &GuessChar::Correct)
    }
}

struct Game {
    word: String,
}

impl Game {
    fn guess(&self, guess: String) -> Guess {
        let mut res = vec![];
        let mut word_chars = self.word.chars();
        for c in guess.chars() {
            let maybe_next = word_chars.next();
            match maybe_next {
                None => res.push(GuessChar::OutOfBounds),
                Some(same_pos) => {
                    if c == same_pos {
                        res.push(GuessChar::Correct);
                    } else if self.word.contains(c) {
                        res.push(GuessChar::OutOfPlace);
                    } else {
                        res.push(GuessChar::Absent);
                    }
                }
            }
        }

        Guess::from(res)
    }

    /// Starts a repl for the current game instance. This assumes the process
    /// is a TTY.
    fn repl(self) -> io::Result<()> {
        let mut input = String::new();
        loop {
            print!("> ");
            io::stdout().flush()?;
    
            io::stdin().read_line(&mut input)?;
            input = input.trim().into();
            let g = self.guess(input);
            println!("{}", g);
            if g.correct() {
                println!("Congrats! 🎉");
                break Ok(())
            }
    
            // reset input on each guess
            input = String::new();
        }
    }
}

/// Forms a new game by splitting the provided dictionary into individual words
/// and picking one at random.
impl From<String> for Game {
    fn from(dict: String) -> Self {
        let words = dict.lines();
        let word = words.clone()
            .nth(thread_rng().gen_range(0..words.count()))
            .unwrap_or("fudge")
            .to_string();
        println!("{:?}", word);
        Self { word }
    }
}

fn main() {
    let wordle = Game::from(String::from(DICT));
    wordle.repl().unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_wordle() {
        let wordle = Game::from(String::from("fudge"));

        assert_eq!(
            wordle.guess(String::from("reads")),
            Guess {
                inner: vec![
                    GuessChar::Absent,
                    GuessChar::OutOfPlace,
                    GuessChar::Absent,
                    GuessChar::OutOfPlace,
                    GuessChar::Absent,
                ]
            }
        );
        assert_eq!(
            wordle.guess(String::from("lodge")),
            Guess {
                inner: vec![
                    GuessChar::Absent,
                    GuessChar::Absent,
                    GuessChar::Correct,
                    GuessChar::Correct,
                    GuessChar::Correct,
                ]
            }
        );
    }

    #[test]
    fn test_doubles() {
        let wordle = Game::from(String::from("sassy"));

        assert_eq!(
            wordle.guess(String::from("space")),
            Guess {
                inner: vec![
                    GuessChar::Correct,
                    GuessChar::Absent,
                    GuessChar::OutOfPlace,
                    GuessChar::Absent,
                    GuessChar::Absent,
                ]
            }
        );
    }

    #[test]
    fn test_oob() {
        let wordle = Game::from(String::from("fudge"));

        assert_eq!(
            wordle.guess(String::from("lodging")),
            Guess {
                inner: vec![
                    GuessChar::Absent,
                    GuessChar::Absent,
                    GuessChar::Correct,
                    GuessChar::Correct,
                    GuessChar::Absent,
                    GuessChar::OutOfBounds,
                    GuessChar::OutOfBounds,
                ]
            }
        );
    }
}
