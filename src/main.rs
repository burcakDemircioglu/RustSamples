mod atm;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    atm::atm()?;

    Ok(())
}


