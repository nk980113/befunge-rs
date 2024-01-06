fn main() -> Result<(), std::io::Error> {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        println!("No input file provided; Use befunge-rs <file> to run file");
        Ok(())
    } else {
        let file = std::fs::read_to_string(&args[1])?;
        befunge_rs::run(file);
        Ok(())
    }
}
