use std::io;

use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TxRecord {
    #[serde(alias = "type")]
    tx_type: TxType,
    client: Client,
    tx: Tx,
    amount: Option<Amount>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum TxType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Debug, Deserialize)]
struct Client(u16);

#[derive(Debug, Deserialize)]
struct Tx(u32);

#[derive(Debug, Deserialize)]
struct Amount(Decimal);

pub fn read_tx(reader: impl io::Read) -> impl Iterator<Item = Result<TxRecord, csv::Error>> {
    let rdr = csv::Reader::from_reader(reader);
    rdr.into_deserialize()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_tx() -> Result<(), csv::Error> {
        let input = "type,client,tx,amount\n\
                     deposit,1,1,1.0\n\
                     deposit,2,2,2.0\n\
                     deposit,1,3,2.0\n\
                     withdrawal,1,4,1.5\n\
                     withdrawal,2,5,3.0\n\
                     dispute,2,5,\n\
                     chargeback,2,5,\n\
                     resolve,2,5,\n"
            .as_bytes();

        let result: Result<Vec<_>, _> = read_tx(input).collect();
        result.map(|_| ())
    }
}
