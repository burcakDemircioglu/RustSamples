mod atm;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let mut atm = atm::Atm::init()?;

    println!("Select the action you want to take:");
    println!("1) Deposit");
    println!("2) Withdraw");
    println!("3) See the account info");
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;
    let answer = buffer.trim();

    match answer {
        "1" => atm.deposit()?,
        "2" => atm.withdraw()?,
        "3" => atm.show_account_info()?,
        _ => println!("You entered an invalid option"),
    }

    Ok(())
}
