struct BankAccount {
    balance: f32,
}

impl BankAccount {
    fn deposit(&mut self, amount: f32) -> Result<f32, &'static str> {
        if amount <= 0.0 {
            return Err("Invalid deposit amount.");
        }

        self.balance += amount;
        Ok(amount)
    }

    fn withdraw(&mut self, amount: f32) -> Result<f32, &'static str> {
        if amount <= 0.0 {
            return Err("Invalid withdrawal amount.");
        }

        if self.balance > amount {
            self.balance -= amount;
            Ok(amount)
        } else {
            Err("Insufficient Balance.")
        }
    }

    fn get_balance(&self) -> f32 {
        println!("Current Balance: ${:.2}", self.balance);
        self.balance
    }
}

fn main() {
    let mut myacc = BankAccount { balance: 348.60 };

    myacc.get_balance();

    match myacc.deposit(500.0) {
        Ok(amount) => println!("Deposited: ${:.2} | Current Balance: ${:.2}", amount, myacc.balance),
        Err(error) => println!("Deposit Failed. {}", error),
    }

    match myacc.withdraw(200.0) {
        Ok(amount) => println!("Withdrawn: ${:.2} | Current Balance: ${:.2}", amount, myacc.balance),
        Err(error) => println!("Withdrawal Failed. {}", error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deposit() {
        let mut account = BankAccount { balance: 100.0 };
        assert_eq!(account.deposit(50.0), Ok(50.0));
        assert_eq!(account.get_balance(), 150.0);
        assert_eq!(account.deposit(-10.0), Err("Invalid deposit amount."));
    }

    #[test]
    fn test_withdraw() {
        let mut account = BankAccount { balance: 100.0 };
        assert_eq!(account.withdraw(50.0), Ok(50.0));
        assert_eq!(account.get_balance(), 50.0);
        assert_eq!(account.withdraw(100.0), Err("Insufficient Balance."));
        assert_eq!(account.withdraw(-10.0), Err("Invalid withdrawal amount."));
    }
}
