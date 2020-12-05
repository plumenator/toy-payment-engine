fn main() -> std::result::Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        Err("expected exactly one arg".into())
    } else {
        let input_path = std::path::PathBuf::from(&args[1]);
        println!("Got {:?}", input_path);
        Ok(())
    }
}
