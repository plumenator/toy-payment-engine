use std::io;

use toy_payment_engine::csv::read_tx;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "expected exactly one arg",
        ));
    }
    let input_path = std::path::Path::new(&args[1]);
    let input_file = std::fs::File::open(input_path)?;
    for tx_record in read_tx(input_file) {
        println!("{:#?}", tx_record?);
    }
    Ok(())
}
