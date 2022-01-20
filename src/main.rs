mod dict;

use std::{fmt::Display, fmt::Debug, io::{self, Write}};
use rand::{thread_rng, Rng};

use dict::DICT;

#[derive(PartialEq, Eq, Debug)]
enum GuessChar {
    Absent,
    OutOfPlace,
    Correct,
}

impl Display for GuessChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GuessChar::Absent => write!(f, "⬛"),
            GuessChar::OutOfPlace => write!(f, "🟨"),
            GuessChar::Correct => write!(f, "🟩"),
        }
    }
}

/// Guess represents a complete guessed word, made up of a list of guessed
/// charaters.
#[derive(PartialEq, Eq, Debug)]
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

struct Error {
    kind: ErrorKind
}

enum ErrorKind {
    GuessTooShort,
    GuessTooLong,
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        todo!()
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self { kind }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            ErrorKind::GuessTooShort => write!(f, "guess too short, guesses must be 5 letters."),
            ErrorKind::GuessTooLong => write!(f, "guess too long, guesses must be 5 letters."),
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

struct Game {
    word: String,
}

impl Game {
    fn guess(&self, guess: String) -> Result<Guess, Error> {
        if guess.len() < 5 {
            return Err(Error::from(ErrorKind::GuessTooShort));
        }
        else if guess.len() > 5 {
            return Err(Error::from(ErrorKind::GuessTooLong));
        }

        let mut res = vec![];
        let mut word_chars = self.word.chars();
        for c in guess.chars() {
            let maybe_next = word_chars.next();
            match maybe_next {
                // The None case should never happen since the length is checked
                // earlier, this makes the compiler happy at the cost of a
                // little added verbosity
                None => return Err(Error::from(ErrorKind::GuessTooLong)),
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

        Ok(Guess::from(res))
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
            match self.guess(input) {
                Ok(g) => {
                    println!("{}", g);
                    if g.correct() {
                        println!("Congrats! 🎉");
                        break Ok(())
                    }
                },
                Err(e) => println!("{}", e),
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
            wordle.guess(String::from("reads")).unwrap(),
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
            wordle.guess(String::from("lodge")).unwrap(),
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
            wordle.guess(String::from("space")).unwrap(),
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
    fn test_out_of_bounds() {
        let wordle = Game::from(String::from("fudge"));

        assert!(wordle.guess(String::from("lodging")).is_err());
        assert!(wordle.guess(String::from("lol")).is_err());
    }
}
