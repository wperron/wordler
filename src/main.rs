use std::{fmt::Display, io::{self, Write}};

#[derive(PartialEq, Eq, Debug)]
enum Result {
    Absent,
    OutOfPlace,
    Correct,
    OutOfBounds,
}

impl Display for Result {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Result::Absent => write!(f, "⬛"),
            Result::OutOfPlace => write!(f, "🟨"),
            Result::Correct => write!(f, "🟩"),
            Result::OutOfBounds => write!(f, "❌"),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Guess {
    inner: Vec<Result>,
}

impl Guess {
    fn compare(a: String, b: String) -> Self {
        let mut res = vec![];
        let mut a_chars = a.chars();
        for c in b.chars() {
            let maybe_next = a_chars.next();
            match maybe_next {
                None => res.push(Result::OutOfBounds),
                Some(same_pos) => {
                    if c == same_pos {
                        res.push(Result::Correct);
                    } else if a.contains(c) {
                        res.push(Result::OutOfPlace);
                    } else {
                        res.push(Result::Absent);
                    }
                }
            }
        }

        Self { inner: res }
    }

    fn correct(self) -> bool {
        self.inner.iter().all(|r| r == &Result::Correct)
    }
}

impl Display for Guess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in &self.inner {
            write!(f, "{}", r)?;
        }
        Ok(())
    }
}

struct Wordle {
    word: String,
}

impl Wordle {
    fn guess(&self, guess: String) -> Guess {
        Guess::compare(self.word.clone(), guess)
    }

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

impl From<String> for Wordle {
    fn from(word: String) -> Self {
        Self { word }
    }
}

fn main() {
    // TODO(wperron) choose word at random from a list somewhere
    let wordle = Wordle::from(String::from("fudge"));
    wordle.repl().unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_wordle() {
        let wordle = Wordle::from(String::from("fudge"));

        assert_eq!(
            wordle.guess(String::from("reads")),
            Guess {
                inner: vec![
                    Result::Absent,
                    Result::OutOfPlace,
                    Result::Absent,
                    Result::OutOfPlace,
                    Result::Absent,
                ]
            }
        );
        assert_eq!(
            wordle.guess(String::from("lodge")),
            Guess {
                inner: vec![
                    Result::Absent,
                    Result::Absent,
                    Result::Correct,
                    Result::Correct,
                    Result::Correct,
                ]
            }
        );
    }

    #[test]
    fn test_doubles() {
        let wordle = Wordle::from(String::from("sassy"));

        assert_eq!(
            wordle.guess(String::from("space")),
            Guess {
                inner: vec![
                    Result::Correct,
                    Result::Absent,
                    Result::OutOfPlace,
                    Result::Absent,
                    Result::Absent,
                ]
            }
        );
    }

    #[test]
    fn test_oob() {
        let wordle = Wordle::from(String::from("fudge"));

        assert_eq!(
            wordle.guess(String::from("lodging")),
            Guess {
                inner: vec![
                    Result::Absent,
                    Result::Absent,
                    Result::Correct,
                    Result::Correct,
                    Result::Absent,
                    Result::OutOfBounds,
                    Result::OutOfBounds,
                ]
            }
        );
    }
}
