use std::collections::HashMap;

use crate::model::{Amount, Client, Transaction, Tx};

#[derive(Debug)]
pub(crate) struct Balance {
    pub(crate) available: Amount,
    pub(crate) held: Amount,
    pub(crate) locked: bool,
}

impl Default for Balance {
    fn default() -> Self {
        Self {
            available: Amount(0.into()),
            held: Amount(0.into()),
            locked: false,
        }
    }
}

impl Balance {
    fn deposit(&mut self, amount: &Amount) {
        self.available.0 += amount.0;
    }

    fn withdraw(&mut self, amount: &Amount) -> Result<(), std::io::Error> {
        if self.available.0 > amount.0 {
            self.available.0 -= amount.0;
            Ok(())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "insufficient balance",
            ))
        }
    }

    fn dispute(&mut self, amount: &Amount) {
        self.available.0 -= amount.0;
        self.held.0 += amount.0;
    }

    fn resolve(&mut self, amount: &Amount) {
        self.available.0 += amount.0;
        self.held.0 -= amount.0;
    }

    fn chargeback(&mut self, amount: &Amount) {
        self.held.0 -= amount.0;
        self.locked = true;
    }
}

pub struct Ledger {
    pub(crate) balances: HashMap<Client, Balance>,
    deposits: HashMap<(Client, Tx), Amount>,
    disputes: HashMap<(Client, Tx), Amount>,
}

impl Default for Ledger {
    fn default() -> Self {
        Self {
            balances: HashMap::new(),
            deposits: HashMap::new(),
            disputes: HashMap::new(),
        }
    }
}

impl Ledger {
    pub fn update(&mut self, txn: &Transaction) -> Result<(), std::io::Error> {
        use Transaction::*;
        match txn {
            Deposit(deposit) => {
                self.balances
                    .entry(deposit.client)
                    .or_insert_with(Balance::default)
                    .deposit(&deposit.amount);
                self.deposits
                    .insert((deposit.client, deposit.tx), deposit.amount.clone());
            }
            Withdrawal(withdrawal) => self
                .balances
                .get_mut(&withdrawal.client)
                .ok_or_else(|| {
                    std::io::Error::new(std::io::ErrorKind::Other, "client does not exist")
                })?
                .withdraw(&withdrawal.amount)?,
            Dispute(dispute) => {
                let key = (dispute.client, dispute.tx);
                if self.disputes.contains_key(&key) {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "dispute already exists",
                    ));
                }
                let amount = self.deposits.get(&key).ok_or_else(|| {
                    std::io::Error::new(std::io::ErrorKind::Other, "deposit does not exist")
                })?;
                self.disputes
                    .insert((dispute.client, dispute.tx), amount.clone());
                self.balances
                    .get_mut(&dispute.client)
                    .expect("client with disputed transaction should exist")
                    .dispute(amount);
            }
            Resolve(resolve) => {
                let amount = self
                    .disputes
                    .remove(&(resolve.client, resolve.tx))
                    .ok_or_else(|| {
                        std::io::Error::new(std::io::ErrorKind::Other, "dispute does not exist")
                    })?;
                self.balances
                    .get_mut(&resolve.client)
                    .expect("client with resolved transaction should exist")
                    .resolve(&amount);
            }
            Chargeback(chargeback) => {
                let amount = self
                    .disputes
                    .remove(&(chargeback.client, chargeback.tx))
                    .ok_or_else(|| {
                        std::io::Error::new(std::io::ErrorKind::Other, "dispute does not exist")
                    })?;
                self.balances
                    .get_mut(&chargeback.client)
                    .expect("client with chargeback transaction should exist")
                    .chargeback(&amount);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::*;

    fn balances(txns: &[Transaction]) -> Result<HashMap<Client, Balance>, std::io::Error> {
        let mut ledger = Ledger::default();
        for txn in txns {
            ledger.update(txn)?;
        }
        Ok(ledger.balances)
    }

    #[test]
    fn test_deposit() -> Result<(), std::io::Error> {
        let client = Client(0);
        let balances = balances(&[Transaction::Deposit(Deposit {
            client: client,
            tx: Tx(0),
            amount: Amount(1.into()),
        })])?;
        let Balance {
            available,
            held,
            locked,
        } = balances.get(&client).expect("client exists");
        assert_eq!((available.0, held.0, locked), (1.into(), 0.into(), &false));
        Ok(())
    }

    #[test]
    fn test_withdrawal() -> Result<(), std::io::Error> {
        let client = Client(0);
        let balances = balances(&[
            Transaction::Deposit(Deposit {
                client: client,
                tx: Tx(0),
                amount: Amount(2.into()),
            }),
            Transaction::Withdrawal(Withdrawal {
                client: client,
                tx: Tx(1),
                amount: Amount(1.into()),
            }),
        ])?;
        let Balance {
            available,
            held,
            locked,
        } = balances.get(&client).expect("client exists");
        assert_eq!((available.0, held.0, locked), (1.into(), 0.into(), &false));
        Ok(())
    }

    #[test]
    fn test_dispute() -> Result<(), std::io::Error> {
        let client = Client(0);
        let balances = balances(&[
            Transaction::Deposit(Deposit {
                client: client,
                tx: Tx(0),
                amount: Amount(2.into()),
            }),
            Transaction::Dispute(Dispute {
                client: client,
                tx: Tx(0),
            }),
        ])?;
        let Balance {
            available,
            held,
            locked,
        } = balances.get(&client).expect("client exists");
        assert_eq!((available.0, held.0, locked), (0.into(), 2.into(), &false));
        Ok(())
    }

    #[test]
    fn test_resolve() -> Result<(), std::io::Error> {
        let client = Client(0);
        let balances = balances(&[
            Transaction::Deposit(Deposit {
                client: client,
                tx: Tx(0),
                amount: Amount(2.into()),
            }),
            Transaction::Dispute(Dispute {
                client: client,
                tx: Tx(0),
            }),
            Transaction::Resolve(Resolve {
                client: client,
                tx: Tx(0),
            }),
        ])?;
        let Balance {
            available,
            held,
            locked,
        } = balances.get(&client).expect("client exists");
        assert_eq!((available.0, held.0, locked), (2.into(), 0.into(), &false));
        Ok(())
    }

    #[test]
    fn test_chargeback() -> Result<(), std::io::Error> {
        let client = Client(0);
        let balances = balances(&[
            Transaction::Deposit(Deposit {
                client: client,
                tx: Tx(0),
                amount: Amount(2.into()),
            }),
            Transaction::Dispute(Dispute {
                client: client,
                tx: Tx(0),
            }),
            Transaction::Chargeback(Chargeback {
                client: client,
                tx: Tx(0),
            }),
        ])?;
        let Balance {
            available,
            held,
            locked,
        } = balances.get(&client).expect("client exists");
        assert_eq!((available.0, held.0, locked), (0.into(), 0.into(), &true));
        Ok(())
    }
}
