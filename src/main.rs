pub mod snake;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    snake::game::run();
    Ok(())
}
