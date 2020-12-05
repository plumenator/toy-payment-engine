# Toy Payment Engine

## To run the binary

```sh
$ # errors are printed to stderr
$ cargo run -- transactions.csv > accounts.csv
```

## To run tests

```sh
$ cargo test
```

## Additional assumptions

1. Disputes are only made against deposits. It's not clear to me how
   disputes with withdrawal are to be handled (can held amount be
   negative?). So, instead I decided to ignore (and report error) for
   disputes against withdrawals.
   
2. Transaction amounts are non negative.

3. A disputes can cause available amount to become negative.

4. It's not specified if a locked account can receive deposits and
   withdrawals. The only thing that is specified is that a chargeback
   causes and account be marked as locked.

5. A chargeback doesn't change the available amount.

## Design notes

1. The code is divided into the following modules:
   1. `csv`: read from and write to CSVs, types for CSV records.
   2. `ledger`: maintain balances and disputes.
   3. `model`: data types to define the business models.
   4. `lib`: wrap everything else and define functions to drive the engine.
   5. `main`: drive the engine via `lib`.

2. Deposits and disputes are stored separately from balances, in
   memory. This makes it trivial to retrieve all the balances and print
   them.

3. The `Balance` for a `Client` is only created on first deposit. It
   suffices because none of the other transactions would have an
   effect on the `Balance` of a `Client` with no deposits.

4. Amounts are represented by `rust_decimal::Decimal`.

## Remaining work

1. Implement a custom deserializer for Amount, to reject negative
   amounts from the input CSV.

2. Generate a large sample CSV to test against.

3. Look into writing property based tests with QuickCheck

4. If locked accounts are to not accept deposits and withdrawals,
   handle that in `ledger`.

5. Use something like `failure`, `anyhow` and such to handle errors
   instead of overloading `std::io::Error` everywhere.

6. Unit test the error cases in `ledger`.
