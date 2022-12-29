pub mod commands;
pub mod superstack;

use std::env;

use anyhow::{Ok, Result};
use dotenv::dotenv;

use crate::superstack::SuperStack;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let mut api = SuperStack::new(
        env::var("ADDRESS").expect("ADDRESS is not set").as_str(),
        env::var("USERNAME").expect("USERNAME is not set").as_str(),
        env::var("PASSWORD").expect("PASSWORD is not set").as_str(),
    )
    .connect()
    .await?;

    let vlan = api.show_vlan(1).await?;

    println!("{:#?}", vlan);

    Ok(())
}
