use std::io;

use toy_payment_engine::{account::Ledger, csv::read_tx, print_accounts};

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
    let mut accounts = Ledger::default();
    for txn in read_tx(input_file).map(|rtxr| {
        rtxr.map_err(|e| io::Error::new(io::ErrorKind::Other, e))
            .and_then(|txr| txr.into_transaction())
    }) {
        let txn = txn?;
        if let Err(e) = accounts.update(&txn) {
            eprintln!("{} for transaction:\n{:#?}", e, txn);
        }
    }
    print_accounts(&accounts);
    Ok(())
}
