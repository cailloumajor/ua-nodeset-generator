use std::ffi::OsStr;
use std::fs::File;
use std::path::PathBuf;
use std::{env, fs};

use anyhow::{Context as _, anyhow};
use askama::Template;
use duct::cmd;
use glob::glob;
use ua_nodeset_generator::ObjectType;
use urn::{Urn, UrnBuilder};

/// Represents the template for generating ModelDesign file.
#[derive(Template)]
#[template(path = "modeldesign.xml")]
struct ModelDesign {
    /// The namespace URN.
    ns_urn: Urn,
    /// The list of object types.
    object_types: Vec<ObjectType>,
}

fn main() -> anyhow::Result<()> {
    // Take the target directory from the first positional argument.
    let target_dir = env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .context("Failed to get the target directory from first positional argument")?;

    let mut target_path_components = target_dir.iter().map(OsStr::to_str);
    // Get the model name from the last component of the target path.
    let model_name = target_path_components
        .next_back()
        .flatten()
        .context("Failed to get last component of target path")?;
    // Get the namespace identifier for the UA namespace URN from the last component
    // of the target path.
    let ns_urn_nid = target_path_components
        .next_back()
        .flatten()
        .context("Failed to get penultimate component of target path")?;

    // Read the version.
    let nodeset_version = fs::read_to_string(target_dir.join("version.txt"))
        .map(|raw| raw.trim().to_owned())
        .context("Failed to read the version file")?;

    // Get an iterator over TOML design files.
    let toml_paths = glob(target_dir.join("*.toml").to_string_lossy().as_ref())
        .context("Failed to create TOML files glob")?
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to get TOML file glob entry")?;

    if toml_paths.is_empty() {
        return Err(anyhow!("No TOML file was found"));
    }

    // Get ObjectType structures from TOML files.
    let object_types = toml_paths
        .into_iter()
        .map(|toml_path| {
            let input_file_contents = fs::read_to_string(&toml_path)
                .with_context(|| format!("Failed to read {}", toml_path.display()))?;
            let object_type: ObjectType = toml::from_str(&input_file_contents)
                .with_context(|| format!("Failed to deserialize {}", toml_path.display()))?;
            Ok(object_type)
        })
        .collect::<Result<Vec<_>, anyhow::Error>>()?;

    let ns_urn = UrnBuilder::new(ns_urn_nid, model_name)
        .build()
        .context("Failed to build OPC-UA namespace URN")?;

    // Create the Model Design template.
    let model_design = ModelDesign {
        ns_urn,
        object_types,
    };

    // Write the model design file.
    let model_design_file_path = target_dir.join(format!("{model_name}Model.xml"));
    let mut model_design_file =
        File::create(&model_design_file_path).context("Failed to create Model Design file")?;
    model_design
        .write_into(&mut model_design_file)
        .context("Failed to write Model Design file")?;

    // Ensure model compiler tool availability.
    let restore_command = cmd!("dotnet", "tool", "restore");
    restore_command
        .run()
        .context("Failed to run `dotnet tool restore`")?;

    // Compile the Model Design.
    let identifier_filename = format!("{model_name}Model.csv");
    let compile_command = cmd!(
        "dotnet",
        "tool",
        "run",
        "Opc.Ua.ModelCompiler",
        "compile",
        // Path to the ModelDesign file.
        "-d2",
        model_design_file_path,
        // Output directory.
        "-o2",
        &target_dir,
        // Path to the identifier file (will be created if needed).
        "-cg",
        target_dir.join(identifier_filename),
        // The first node ID identifier to use.
        "-id",
        "1000",
        // OPC-UA v1.05.
        "-version",
        "v105",
        // Suppress unwanted generated output.
        "-suppress",
        "PredefinedNodes,Constants,JsonSchema,Classes,DataTypes",
        // Version of the generated NodeSet.
        "-mv",
        nodeset_version,
    );
    compile_command
        .run()
        .context("Failed to compile OPC-UA Model Design")?;

    Ok(())
}
