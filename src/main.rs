use ccwc::{process_flag, read_arguments, Arguments};

fn main() {
    let Ok(args) = read_arguments() else {
        eprintln!("{}", read_arguments().err().unwrap());
        return;
    };

    let Arguments(flag, filename) = args;

    let _ = process_flag(&flag, &filename).inspect_err(|err| {
        eprintln!("{}", err);
    });
}
