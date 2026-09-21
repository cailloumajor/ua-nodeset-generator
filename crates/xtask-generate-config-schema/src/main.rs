use anyhow::Context;
use schemars::schema_for;
use ua_nodeset_generator::Namespace;

fn main() -> anyhow::Result<()> {
    let schema = schema_for!(Namespace);
    let json = serde_json::to_string_pretty(&schema).context("Failed to serialize JSON schema")?;

    println!("{json}");

    Ok(())
}
