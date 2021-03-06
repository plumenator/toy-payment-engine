use std::io;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct TxRecord {
    #[serde(alias = "type")]
    pub(crate) tx_type: TxType,
    pub(crate) client: Client,
    pub(crate) tx: Tx,
    pub(crate) amount: Option<Amount>, // TODO: ensure non negative
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum TxType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Debug, Serialize)]
pub(crate) struct LedgerRecord {
    pub(crate) client: Client,
    pub(crate) available: Amount,
    pub(crate) held: Amount,
    pub(crate) total: Amount,
    pub(crate) locked: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Client(pub(crate) u16);

#[derive(Debug, Deserialize)]
pub(crate) struct Tx(pub(crate) u32);

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Amount(pub(crate) Decimal);

pub fn read_tx(reader: impl io::Read) -> impl Iterator<Item = Result<TxRecord, csv::Error>> {
    let rdr = csv::Reader::from_reader(reader);
    rdr.into_deserialize()
}

pub(crate) fn write_ledger(
    writer: impl io::Write,
    records: impl Iterator<Item = LedgerRecord>,
) -> Result<(), csv::Error> {
    let mut wtr = csv::Writer::from_writer(writer);
    for record in records {
        wtr.serialize(record)?
    }
    Ok(())
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
