use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {
    let mut reader = csv::ReaderBuilder::new().from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    for result in reader.deserialize::<Player>() {
        let player: Player = result?;
        ret.push(player);
    }
    let json = serde_json::to_string_pretty(&ret)?;
    std::fs::write(output, json)?;
    Ok(())
}
