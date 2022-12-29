use crate::commands::vlan::{parse_vlan_details, parse_vlan_summary, Vlan, VlanDetails, VlanError};
use mini_telnet::Telnet;
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SuperStackError {
    #[error("Failed to connect to telnet")]
    Connect,
    #[error("Invalid credentials")]
    Login,
    #[error("Telnet not connected")]
    NotConnected,
}

pub struct SuperStack {
    username: String,
    password: String,
    address: String,
    telnet: Option<Telnet>,
}

impl SuperStack {
    pub fn new(address: &str, username: &str, password: &str) -> Self {
        SuperStack {
            address: address.to_string(),
            username: username.to_string(),
            password: password.to_string(),
            telnet: None,
        }
    }

    pub async fn connect(mut self) -> Result<Self, SuperStackError> {
        let mut telnet = Telnet::builder()
            .prompt(": ")
            .login_prompt("Login: ", "Password: ")
            .connect_timeout(Duration::from_secs(5))
            .timeout(Duration::from_secs(2))
            .connect(&self.address)
            .await
            .map_err(|_| SuperStackError::Connect)?;

        telnet
            .login(&self.username, &self.password)
            .await
            .map_err(|_| SuperStackError::Login)?;

        self.telnet = Some(telnet);

        Ok(self)
    }

    async fn execute(&mut self, command: &str) -> Result<String, SuperStackError> {
        let response = self
            .telnet
            .as_mut()
            .expect("Not connected")
            .execute(command)
            .await
            .map_err(|_| SuperStackError::NotConnected)?;
        Ok(response)
    }

    pub async fn list_vlans(&mut self) -> Result<Vec<Vlan>, VlanError> {
        let res = self.execute("bridge vlan summary all").await?;
        parse_vlan_summary(res)
    }

    pub async fn show_vlan(&mut self, vlan_id: u32) -> Result<VlanDetails, VlanError> {
        let res = self
            .execute(format!("bridge vlan detail {vlan_id}").as_str())
            .await?;

        if res.contains("is invalid.") {
            self.execute("\n").await?;
            return Err(VlanError::VlanNotFound { vlan_id });
        }

        parse_vlan_details(res)
    }

    pub async fn create_vlan(&mut self, vlan_id: u32, vlan_name: &str) -> Result<(), VlanError> {
        let res = self
            .execute(format!("bridge vlan create {vlan_id} {vlan_name}").as_str())
            .await?;

        if res.contains("VLAN ID in use by another VLAN.") {
            return Err(VlanError::Exists { vlan_id });
        }
        Ok(())
    }

    pub async fn delete_vlan(&mut self, vlan_id: u32) -> Result<(), VlanError> {
        let res = self
            .execute(format!("bridge vlan delete {vlan_id} yes").as_str())
            .await?;

        if res.contains(format!("\"{vlan_id}\" is invalid.").as_str()) {
            self.execute("\n").await?;
            return Err(VlanError::VlanNotFound { vlan_id });
        }

        Ok(())
    }

    pub async fn rename_vlan(&mut self, vlan_id: u32, vlan_name: &str) -> Result<(), VlanError> {
        let res = self
            .execute(format!("bridge vlan modify name {vlan_id} {vlan_name}").as_str())
            .await?;

        if res.contains(format!("\"{vlan_id}\" is invalid.").as_str()) {
            self.execute("\n").await?;
            return Err(VlanError::VlanNotFound { vlan_id });
        }

        Ok(())
    }

    pub async fn add_vlan_port(
        &mut self,
        vlan_id: u32,
        port: u32,
        tagged: bool,
    ) -> Result<(), VlanError> {
        let tagged = if tagged { "tagged" } else { "untagged" };
        let res = self
            .execute(format!("bridge vlan modify addPort {vlan_id} 1:{port} {tagged}").as_str())
            .await?;

        if res.contains(format!("\"{vlan_id}\" is invalid.").as_str()) {
            self.execute("\n").await?;
            return Err(VlanError::VlanNotFound { vlan_id });
        }

        if res.contains(format!("\"1:{port}\" is invalid.").as_str()) {
            self.execute("\n").await?;
            return Err(VlanError::PortNotFound { port });
        }

        Ok(())
    }

    pub async fn remove_vlan_port(&mut self, vlan_id: u32, port: u32) -> Result<(), VlanError> {
        let res = self
            .execute(format!("bridge vlan modify addPort {vlan_id} 1:{port}").as_str())
            .await?;

        if res.contains(format!("\"{vlan_id}\" is invalid.").as_str()) {
            self.execute("\n").await?;
            return Err(VlanError::VlanNotFound { vlan_id });
        }

        if res.contains(format!("\"1:{port}\" is invalid.").as_str()) {
            self.execute("\n").await?;
            return Err(VlanError::PortNotFound { port });
        }

        Ok(())
    }
}
