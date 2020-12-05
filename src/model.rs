use rust_decimal::Decimal;

#[derive(Debug)]
pub enum Transaction {
    Deposit(Deposit),
    Withdrawal(Withdrawal),
    Dispute(Dispute),
    Resolve(Resolve),
    Chargeback(Chargeback),
}

#[derive(Debug)]
pub struct Deposit {
    pub(crate) client: Client,
    pub(crate) tx: Tx,
    pub(crate) amount: Amount,
}

#[derive(Debug)]
pub struct Withdrawal {
    pub(crate) client: Client,
    pub(crate) tx: Tx,
    pub(crate) amount: Amount,
}

#[derive(Debug)]
pub struct Dispute {
    pub(crate) client: Client,
    pub(crate) tx: Tx,
}

#[derive(Debug)]
pub struct Resolve {
    pub(crate) client: Client,
    pub(crate) tx: Tx,
}

#[derive(Debug)]
pub struct Chargeback {
    pub(crate) client: Client,
    pub(crate) tx: Tx,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub(crate) struct Client(pub(crate) u16);

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub(crate) struct Tx(pub(crate) u32);

#[derive(Debug, Clone)]
pub(crate) struct Amount(pub(crate) Decimal);
