pub mod csv;
pub(crate) mod model;

impl From<crate::csv::Client> for model::Client {
    fn from(src: crate::csv::Client) -> Self {
        model::Client(src.0)
    }
}

impl From<crate::csv::Tx> for model::Tx {
    fn from(src: crate::csv::Tx) -> Self {
        model::Tx(src.0)
    }
}

impl From<crate::csv::Amount> for model::Amount {
    fn from(src: crate::csv::Amount) -> Self {
        model::Amount(src.0)
    }
}

impl crate::csv::TxRecord {
    pub fn into_transaction(self) -> Result<model::Transaction, std::io::Error> {
        use crate::csv::TxType::*;
        let txn = match self.tx_type {
            Deposit => model::Transaction::Deposit(model::Deposit {
                client: self.client.into(),
                tx: self.tx.into(),
                amount: self
                    .amount
                    .ok_or_else(|| {
                        std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "deposit missing amount",
                        )
                    })?
                    .into(),
            }),
            Withdrawal => model::Transaction::Withdrawal(model::Withdrawal {
                client: self.client.into(),
                tx: self.tx.into(),
                amount: self
                    .amount
                    .ok_or_else(|| {
                        std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "withdrawal missing amount",
                        )
                    })?
                    .into(),
            }),
            Dispute => {
                if self.amount.is_some() {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "dispute has unexpected amount",
                    ));
                }
                model::Transaction::Dispute(model::Dispute {
                    client: self.client.into(),
                    tx: self.tx.into(),
                })
            }
            Resolve => {
                if self.amount.is_some() {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "resolve has unexpected amount",
                    ));
                }
                model::Transaction::Resolve(model::Resolve {
                    client: self.client.into(),
                    tx: self.tx.into(),
                })
            }
            Chargeback => {
                if self.amount.is_some() {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "chargeback has unexpected amount",
                    ));
                }
                model::Transaction::Chargeback(model::Chargeback {
                    client: self.client.into(),
                    tx: self.tx.into(),
                })
            }
        };
        Ok(txn)
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;

    use crate::csv;

    #[test]
    fn test_deposit_with_amount() -> Result<(), std::io::Error> {
        csv::TxRecord {
            tx_type: csv::TxType::Deposit,
            client: csv::Client(0),
            tx: csv::Tx(0),
            amount: Some(csv::Amount(Decimal::from(1))),
        }
        .into_transaction()
        .map(|_| ())
    }

    #[test]
    fn test_deposit_without_amount() {
        assert!(csv::TxRecord {
            tx_type: csv::TxType::Deposit,
            client: csv::Client(0),
            tx: csv::Tx(0),
            amount: None,
        }
        .into_transaction()
        .is_err())
    }

    #[test]
    fn test_withdrawal_with_amount() -> Result<(), std::io::Error> {
        csv::TxRecord {
            tx_type: csv::TxType::Withdrawal,
            client: csv::Client(0),
            tx: csv::Tx(0),
            amount: Some(csv::Amount(Decimal::from(1))),
        }
        .into_transaction()
        .map(|_| ())
    }

    #[test]
    fn test_withdrawal_without_amount() {
        assert!(csv::TxRecord {
            tx_type: csv::TxType::Withdrawal,
            client: csv::Client(0),
            tx: csv::Tx(0),
            amount: None,
        }
        .into_transaction()
        .is_err())
    }

    #[test]
    fn test_dispute() -> Result<(), std::io::Error> {
        csv::TxRecord {
            tx_type: csv::TxType::Dispute,
            client: csv::Client(0),
            tx: csv::Tx(0),
            amount: None,
        }
        .into_transaction()
        .map(|_| ())
    }

    #[test]
    fn test_dispute_with_amount() {
        assert!(csv::TxRecord {
            tx_type: csv::TxType::Dispute,
            client: csv::Client(0),
            tx: csv::Tx(0),
            amount: Some(csv::Amount(Decimal::from(1))),
        }
        .into_transaction()
        .is_err())
    }

    #[test]
    fn test_resolve() -> Result<(), std::io::Error> {
        csv::TxRecord {
            tx_type: csv::TxType::Resolve,
            client: csv::Client(0),
            tx: csv::Tx(0),
            amount: None,
        }
        .into_transaction()
        .map(|_| ())
    }

    #[test]
    fn test_resolve_with_amount() {
        assert!(csv::TxRecord {
            tx_type: csv::TxType::Resolve,
            client: csv::Client(0),
            tx: csv::Tx(0),
            amount: Some(csv::Amount(Decimal::from(1))),
        }
        .into_transaction()
        .is_err())
    }

    #[test]
    fn test_chargeback() -> Result<(), std::io::Error> {
        csv::TxRecord {
            tx_type: csv::TxType::Chargeback,
            client: csv::Client(0),
            tx: csv::Tx(0),
            amount: None,
        }
        .into_transaction()
        .map(|_| ())
    }

    #[test]
    fn test_chargeback_with_amount() {
        assert!(csv::TxRecord {
            tx_type: csv::TxType::Chargeback,
            client: csv::Client(0),
            tx: csv::Tx(0),
            amount: Some(csv::Amount(Decimal::from(1))),
        }
        .into_transaction()
        .is_err())
    }
}
