use std::fmt::Display;

const PROGRAM: &str = "as-root";


pub fn info<M>(message: M)
where M: Display {
    eprintln!("{}: {}", PROGRAM, message)
}
pub fn error<M>(message: M)
where M: Display {
    eprintln!("{}: ERROR: {}", PROGRAM, message);
}
