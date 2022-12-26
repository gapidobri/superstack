pub mod commands;
pub mod superstack;

use std::env;

use anyhow::Result;
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

    let vlans = api.list_vlans().await?;
    println!("{:#?}", vlans);

    api.add_vlan_port(2, 20, true).await?;

    api.remove_vlan_port(2, 20).await?;

    Ok(())
}
