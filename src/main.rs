fn main() -> std::result::Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("expected exactly one arg".into());
    }
    let input_path = std::path::Path::new(&args[1]);
    if !input_path.exists() {
        return Err("no such file".into());
    }
    Ok(())
}
