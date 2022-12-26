use std::vec;

use anyhow::{Context, Ok, Result};

#[derive(Debug)]
pub struct Vlan {
    pub id: u32,
    pub name: String,
}

#[derive(Debug)]
pub struct VlanDetails {
    pub id: u32,
    pub name: String,
    pub untagged: Vec<u32>,
    pub tagged: Vec<u32>,
}

pub fn parse_vlan_summary(input: String) -> Result<Vec<Vlan>> {
    let vlans: Vec<Vlan> = input
        .split("\n")
        .enumerate()
        .filter(|(i, l)| *i > 1 && !l.is_empty())
        .map(|(_, l)| l.split_once(" "))
        .filter_map(|l| l)
        .map(|(vlan, name)| Vlan {
            id: vlan.parse::<u32>().expect("Failed to parse vlan id"),
            name: name.trim().to_string(),
        })
        .collect();

    Ok(vlans)
}

pub fn parse_vlan_details(input: String) -> Result<VlanDetails> {
    let lines: Vec<&str> = input.split("\n").filter(|l| !l.is_empty()).collect();

    let info: Vec<&str> = lines[0]
        .split("  ")
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.split_once(": ").and_then(|(_, v)| Some(v)))
        .filter_map(|l| l)
        .collect();

    let ports: Vec<&str> = lines[3]
        .split("  ")
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();

    Ok(VlanDetails {
        id: info[0].parse::<u32>()?,
        name: info[1].to_string(),
        untagged: parse_ports(ports[1])?,
        tagged: parse_ports(ports[2])?,
    })
}

fn parse_ports(input: &str) -> Result<Vec<u32>> {
    if input == "none" {
        return Ok(vec![]);
    }
    let ports = input
        .split(",")
        .map(|p| {
            if !p.contains("-") {
                return Ok(vec![p.parse::<u32>()?]);
            }
            let (st, ed) = p.split_once("-").context("Failed to parse ports")?;
            Ok((st.parse::<u32>()?..ed.parse::<u32>()? + 1)
                .into_iter()
                .collect::<Vec<u32>>())
        })
        .filter_map(|p| p.ok())
        .flat_map(|p| p)
        .collect();

    Ok(ports)
}
