use std::{env, fmt::Display, fs::read_to_string};

#[derive(Debug)]
pub struct Arguments(pub String, pub String);

#[derive(Debug)]
pub enum WordCountErr {
    NotEnoughLen(String),
    UnrecognizedFlag(String),
    FileNotFound(String),
}

impl Display for WordCountErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotEnoughLen(v) => write!(f, "{}", v),
            Self::UnrecognizedFlag(v) => write!(f, "{}", v),
            Self::FileNotFound(v) => write!(f, "{}", v),
        }
    }
}

pub fn read_arguments() -> Result<Arguments, WordCountErr> {
    let args: Vec<String> = env::args().collect();

    if let [flag, filename] = &args[1..] {
        Ok(Arguments(flag.to_string(), filename.to_string()))
    } else {
        if args.len() < 3 {
            return Err(WordCountErr::NotEnoughLen(String::from(
                "Not enough arguments were passed!",
            )));
        } else {
            return Err(WordCountErr::NotEnoughLen(String::from(
                "Too many arguments were passed!",
            )));
        }
    }
}

fn open_file(filename: &str) -> Result<String, WordCountErr> {
    let Ok(contents) = read_to_string(&filename) else {
        return Err(WordCountErr::FileNotFound(String::from(
            "File not found or file is not accessible!",
        )));
    };

    Ok(contents)
}

pub fn process_flag(flag: &str, filename: &str) -> Result<(), WordCountErr> {
    match flag {
        "-c" => {
            let contents = open_file(&filename)?;
            println!("{} {}", contents.len(), filename);
            Ok(())
        }
        "-l" => {
            let contents = open_file(&filename)?;
            println!("{} {}", contents.lines().count(), filename);

            Ok(())
        }
        "-w" => {
            let contents = open_file(&filename)?;

            let mut word_count = 0;

            for line in contents.lines().into_iter() {
                word_count += line.split_whitespace().count();
            }

            println!("{} {}", word_count, filename);

            Ok(())
        }
        _ => Err(WordCountErr::UnrecognizedFlag(String::from(
            "ccwc: invalid option",
        ))),
    }
}
