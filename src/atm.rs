// bankamatik fonkiyonu yazılacak, çekmek istenen tutar bakiye'den yüksek ise ek hesap kullanılsın mı diye kullanıcıya sorsun.

use std::collections::HashMap;

pub struct Atm {
    account: HashMap<String, String>,
}

impl Atm {
    pub fn init() -> Result<Atm, std::io::Error> {
        let mut account_map = HashMap::new();
        account_map.insert("name".into(), "Burcak Kam".into());
        account_map.insert("account_no".into(), "987654".into());
        account_map.insert("balance".into(), "3000".into());
        account_map.insert("additional_balance".into(), "2000".into());

        Ok(Atm {
            account: account_map,
        })
    }
    pub fn deposit(&mut self) -> std::io::Result<()> {
        //TO DO: Implement
        println!("This operation is not implemented yet.");

        Ok(())
    }

    pub fn withdraw(&mut self) -> std::io::Result<()> {
        println!("Enter the amount you want to withdraw:");

        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer)?;
        let amount = buffer.trim().parse::<f32>().unwrap();

        if amount <= 0.0 {
            return Ok(());
        }

        let mut balance = self.account["balance"].parse::<f32>().unwrap();
        if amount <= balance {
            balance -= amount;
            self.account
                .insert("balance".to_owned(), balance.to_string());
        } else {
            println!("Your balance is not sufficient for this process. Do you want to use the additional balance?(Y/N)");
            buffer = String::new();
            std::io::stdin().read_line(&mut buffer)?;
            let answer = buffer.trim();
            
            match answer {
                "Y" => {
                    let mut additional_balance =
                        self.account["additional_balance"].parse::<f32>().unwrap();
                    let total_balance = additional_balance + balance;
                    if amount <= total_balance {
                        let additional_drawn_amount = amount - balance;
                        balance = 0.0;
                        additional_balance -= additional_drawn_amount;
                        self.account
                            .insert("balance".to_owned(), balance.to_string());
                        self.account.insert(
                            "additional_balance".to_owned(),
                            additional_balance.to_string(),
                        );
                    } else {
                        println!("Your total balance is not sufficient for this process.");
                        println!("Process is cancelled.");
                    }
                }
                _ => println!("Process is cancelled."),
            }
        }
        Ok(())
    }

    pub fn show_account_info(&self) -> std::io::Result<()> {
        println!("name:{}", self.account["name"]);
        println!("account_no:{}", self.account["account_no"]);
        println!("balance:{}", self.account["balance"]);
        println!("additional_balance:{}", self.account["additional_balance"]);

        Ok(())
    }
}
