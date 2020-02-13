use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Link {
    dev: String,
    peer: String,
    addr: String,
    prefix: u8,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let link = Link {
        dev: "veth-host-1".to_string(),
        peer: "veth-guest-1".to_string(),
        addr: String::from("100.200.0.42"),
        prefix: 24,
    };

    println!("Link: {:?}", link);

    let ser = serde_json::to_string(&link)?;
    println!("Serialized: {}", ser);

    let de: Link = serde_json::from_str(&ser)?;
    println!("Deserialized: {:?}", de);

    Ok(())
}
