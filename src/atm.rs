// bankamatik fonkiyonu yazılacak, çekmek istenen tutar bakiye'den yüksek ise ek hesap kullanılsın mı diye kullanıcıya sorsun.

use std::collections::HashMap;

pub struct Atm {
    account: HashMap<String, String>,
}

impl Atm {
    pub fn init() -> Result<Atm, std::io::Error>{
        let mut account_map = HashMap::new();
        account_map.insert("name".into(), "Burcak Kam".into());
        account_map.insert("account_no".into(), "987654".into());
        account_map.insert("balance".into(), "3000".into());
        account_map.insert("additional_balance".into(), "2000".into());

        Ok(Atm{account:account_map})
    }
    pub fn deposit(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    pub fn withdraw(&mut self) -> std::io::Result<()> {

        Ok(())
    }
    pub fn show_account_info(&self)-> std::io::Result<()>{
        println!("name:{}", self.account["name"]);
        println!("account_no:{}", self.account["account_no"]);
        println!("balance:{}", self.account["balance"]);
        println!("additional_balance:{}", self.account["additional_balance"]);

        Ok(())
    }
}
