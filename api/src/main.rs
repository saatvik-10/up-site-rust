use std::fmt::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    print!("Hello World");
    Ok(())
}