mod atm;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    println!("Select the operation to run:");
    println!("1) ATM");

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;
    let answer = buffer.trim();

    match answer {
        "1" => atm::atm()?,
        _ => println!("You entered an invalid input"),
    }

    Ok(())
}


