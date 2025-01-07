use std::{env, fmt::Display, fs::read_to_string, path::Path};

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

    match args.len() {
        1 => Err(WordCountErr::NotEnoughLen(format!(
            "ccwc: not enough arguments were passed"
        ))),
        2 => Ok(Arguments(args[1].to_string(), String::from(""))),
        3 => Ok(Arguments(args[1].to_string(), args[2].to_string())),
        _ => Err(WordCountErr::NotEnoughLen(format!(
            "ccwc: too many arguments were passed"
        ))),
    }
}

fn open_file(filename: &str) -> Result<String, WordCountErr> {
    let Ok(contents) = read_to_string(&filename) else {
        return Err(WordCountErr::FileNotFound(format!(
            "ccwc: {}: No such file or directory",
            filename
        )));
    };

    Ok(contents)
}

pub fn process_flag(flag: &str, filename: &str) -> Result<(), WordCountErr> {
    match flag {
        "-c" => {
            let contents = open_file(filename)?;
            println!("{} {}", contents.len(), filename);
            Ok(())
        }
        "-l" => {
            let contents = open_file(filename)?;
            println!("{} {}", contents.lines().count(), filename);

            Ok(())
        }
        "-w" => {
            let contents = open_file(filename)?;

            let mut word_count = 0;

            for line in contents.lines().into_iter() {
                word_count += line.split_whitespace().count();
            }

            println!("{} {}", word_count, filename);

            Ok(())
        }
        "-m" => {
            let contents = open_file(filename)?;

            println!("{} {}", contents.chars().count(), filename);

            Ok(())
        }
        other => {
            if Path::new(other).exists() {
                let contents = open_file(other)?;

                let mut word_count = 0;

                for line in contents.lines().into_iter() {
                    word_count += line.split_whitespace().count();
                }

                println!(
                    "{} {} {} {}",
                    contents.lines().count(),
                    word_count,
                    contents.len(),
                    other
                );

                Ok(())
            } else {
                Err(WordCountErr::UnrecognizedFlag(format!(
                    "ccwc: invalid option -- '{}'",
                    other.replace("-", "")
                )))
            }
        }
    }
}
