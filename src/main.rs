use std::{
    env,
    fs::File,
    io::{BufReader, Error},
};

use stl::process_stl;

fn main() -> Result<(), Error> {
    if let Some(file_arg) = env::args().nth(1) {
        let input = File::open(file_arg)?;
        let buff = BufReader::new(input);
        let stats = process_stl(buff);
        println!("{}", stats);
    } else {
        println!("usage:  fastl <file>")
    }
    Ok(())
}
