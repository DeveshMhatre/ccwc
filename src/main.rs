use ccwc::{process_flag, read_arguments, Arguments, WordCountErr};

fn main() -> Result<(), Box<WordCountErr>> {
    let args: Arguments = read_arguments().inspect_err(|err| {
        eprintln!("{}", err);
    })?;

    let Arguments(flag, filename) = args;

    let _ = process_flag(&flag, &filename).inspect_err(|err| {
        eprintln!("{}", err);
    });

    Ok(())
}
