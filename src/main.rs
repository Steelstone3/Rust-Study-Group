fn main() {
    println!("Hello, world!");
}

trait AccountService {
    fn deposit(&mut self, amount: u32);
    fn withdraw(&mut self, amount: u32);
    fn print(&self) -> String;
}

trait Repository {
    fn add(&self, account: Account);
    fn retrieve(&self);
}

struct Account {}

impl Account {
    fn new() -> Account {
        Account {}
    }
}

impl AccountService for Account {
    fn deposit(&mut self, amount: u32) {
        todo!()
    }

    fn withdraw(&mut self, amount: u32) {
        todo!()
    }

    fn print(&self) -> String {
        "Date || Amount || Balance\n".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_print_function_no_actions() {
        let account = Account::new();

        let result = account.print();

        assert_eq!(result, "Date || Amount || Balance\n");
    }
}
