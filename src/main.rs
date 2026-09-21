use std::fs;
use std::fs::File;
use std::path::PathBuf;

use anyhow::Context as _;
use askama::Template;
use clap::Parser;
use duct::cmd;
use ua_nodeset_generator::{Namespace, RootElement};

#[derive(Parser)]
#[command(about, version)]
struct Cli {
    /// Path of the namespace description file (JSON5 format).
    description_file: PathBuf,
}

/// Represents the template for generating ModelDesign file.
#[derive(Template)]
#[template(path = "modeldesign.xml")]
struct ModelDesign {
    /// Namespace prefix.
    ns_prefix: String,
    /// Namespace URL.
    ns_url: String,
    /// Root elements of this namespace.
    root_elements: Vec<RootElement>,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    // Take the target directory from the description file path.
    let target_dir = args
        .description_file
        .parent()
        .context("Failed to get the parent directory of the provided description file")?;

    // Get the namespace prefix.
    let ns_prefix = args
        .description_file
        .file_stem()
        .context("Failed to get the file name stem of the provided description file")?
        .to_string_lossy();

    // Get the namespace description from the provided file.
    let input_file_contents = fs::File::open(&args.description_file)
        .context("Failed to open provided description file")?;
    let namespace: Namespace = yaml_serde::from_reader(input_file_contents)
        .context("Failed to deserialize provided description file")?;

    // Create the Model Design template.
    let model_design = ModelDesign {
        ns_prefix: ns_prefix.to_string(),
        ns_url: namespace.namespace_url.clone(),
        root_elements: namespace.root_elements,
    };

    // Create and write the model design file.
    let model_design_file_path = target_dir.join(format!("{ns_prefix}.Model.xml"));
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
    let identifier_filename = format!("{ns_prefix}.Model.csv");
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
        namespace.version,
    );
    compile_command
        .run()
        .context("Failed to compile OPC-UA Model Design")?;

    Ok(())
}
