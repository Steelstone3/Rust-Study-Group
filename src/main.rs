use chrono::prelude::*;
use rusqlite::{Connection, Result};

fn main() {
    println!("Hello, world!");
}

trait AccountService {
    fn deposit(&mut self, amount: u32);
    fn withdraw(&mut self, amount: u32);
    fn print(&self) -> String;
}

trait Repository {
    fn store(&self, account: Account);
    fn retrieve(&self);
}

struct Transaction {
    date: NaiveDate,
    amount: u32,
    balance: u32,
}

impl Transaction {
    fn new(amount: u32, balance: u32) -> Transaction {
        Transaction {
            date: NaiveDate::from_ymd(2022, 8, 30),
            amount,
            balance,
        }
    }
}

struct Account {
    transactions: Vec<Transaction>,
}

impl Account {
    fn new() -> Account {
        Account {
            transactions: vec![],
        }
    }

    fn balance(&self) -> u32 {
        self.transactions.last().map(|t| t.balance).unwrap_or(0)
    }
}

impl AccountService for Account {
    fn deposit(&mut self, amount: u32) {
        self.transactions
            .push(Transaction::new(amount, self.balance() + amount));
    }

    fn withdraw(&mut self, amount: u32) {
        todo!()
    }

    fn print(&self) -> String {
        let mut statement = "Date || Amount || Balance\n".to_string();
        let mut balance = 0;
        for entry in self.transactions.iter() {
            let date = entry.date.format("%Y-%m-%d");
            let amount = entry.amount;
            balance += amount;
            statement.push_str(format!("{date} || {amount} || {balance}\n").as_ref());
        }
        statement
    }
}

struct SqliteRepository {
    connection: Connection,
}

impl SqliteRepository {
    pub fn new() -> Result<SqliteRepository> {
        let connection = Connection::open("bank.db")?;
        connection.execute(
            "create table if not exists account_transaction (
             account_id integer,
             date text not null,
             amount integer not null,
             balance integer not null
         )",
            [],
        )?;

        Ok(SqliteRepository { connection })
    }
}

impl Repository for SqliteRepository {
    fn store(&self, account: Account) {
        let transaction = account.transactions.get(0);
        self.connection.execute(
            "INSERT INTO account_transaction (account_id, date, amount, balance)\
        VALUES (?1, ?2, ?3, ?4)",
            (0, "2022-08-30", 20, 20),
        );
    }

    fn retrieve(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use test_context::{test_context, TestContext};

    #[test]
    fn call_print_function_no_actions() {
        let account = Account::new();

        let result = account.print();

        assert_eq!(result, "Date || Amount || Balance\n");
    }

    #[test]
    fn call_print_function_adding_deposit() {
        let mut account = Account::new();

        account.deposit(20);
        let result = account.print();

        assert_eq!(
            result,
            "Date || Amount || Balance\n\
             2022-08-30 || 20 || 20\n"
        );
    }

    #[test]
    fn call_print_function_adding_deposit_twice() {
        let mut account = Account::new();

        account.deposit(20);
        account.deposit(30);
        let result = account.print();

        assert_eq!(
            result,
            "Date || Amount || Balance\n\
             2022-08-30 || 20 || 20\n\
             2022-08-30 || 30 || 50\n"
        );
    }

    #[test]
    fn given_no_transactions_balance_returns_0() {
        let mut account = Account::new();

        let result = account.balance();

        assert_eq!(result, 0);
    }

    struct DbContext {
        filename: String,
    }

    impl TestContext for DbContext {
        fn setup() -> Self {
            DbContext {
                filename: "bank.db".to_string(),
            }
        }

        fn teardown(self) {
            fs::remove_file("bank.db");
        }
    }

    #[test_context(DbContext)]
    #[test]
    fn store_in_database(ctx: &mut DbContext) {
        let repository = SqliteRepository::new().unwrap();
        let mut account = Account::new();

        account.deposit(20);

        repository.store(account);
        let conn = Connection::open(&ctx.filename).unwrap();
        let mut statement = conn
            .prepare(
                "SELECT account_id, date, amount, balance \
                FROM account_transaction",
            )
            .unwrap();
        let transaction = statement
            .query_map([], |row| {
                Ok((
                    row.get::<usize, u32>(0).unwrap(),
                    row.get::<usize, String>(1).unwrap(),
                    row.get::<usize, u32>(2).unwrap(),
                    row.get::<usize, u32>(3).unwrap(),
                ))
            })
            .unwrap()
            .last()
            .expect("No results from query")
            .unwrap();

        assert_eq!(transaction.0, 0);
        assert_eq!(transaction.1, "2022-08-30");
        assert_eq!(transaction.2, 20);
        assert_eq!(transaction.3, 20);
    }
}
