use std::ffi::OsString;
use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::path::PathBuf;

use anyhow::{Context as _, anyhow};
use askama::Template;
use clap::Parser;
use duct::cmd;
use ua_nodeset_generator::{Namespace, ObjectInstance, RootElement};

#[derive(Parser)]
#[command(about, version)]
struct Cli {
    /// Path of the namespace description file (YAML format).
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
    /// Symbolic name of the namespace metadata object.
    ns_metadata_obj_name: String,
    /// Namespace version.
    ns_version: String,
    /// Namespace publication date
    ns_pub_date: String,
    /// Namespaces this namespace depends on, as tuples of the XML prefix used to
    /// reference the dependency elements, the dependency namespace URL, and the
    /// dependency namespace prefix.
    ns_dependencies: Vec<(String, String, String)>,
    /// Root elements of this namespace.
    root_elements: Vec<RootElement>,
}

impl ModelDesign {
    /// Return the template for the children of a root object instance.
    fn instance_children<'a>(&self, objects: &'a [ObjectInstance]) -> ObjectInstanceChildren<'a> {
        ObjectInstanceChildren {
            objects,
            // The `Children` element is two XML levels deep, as it is enclosed in the
            // root `ModelDesign` element and in the `Object` element of the instance,
            // hence two indentation levels.
            indent: "    ".to_string(),
        }
    }
}

/// Represents the template for the children of an object instance.
///
/// The template renders itself for the children of each child, as templates macros
/// can not be recursive.
#[derive(Template)]
#[template(path = "object_instance_children.xml")]
struct ObjectInstanceChildren<'a> {
    /// Children of the object instance.
    objects: &'a [ObjectInstance],
    /// Indentation of the `Children` element.
    indent: String,
}

impl<'a> ObjectInstanceChildren<'a> {
    /// Return the template for the children of a child of the object instance.
    fn nested(&self, objects: &'a [ObjectInstance]) -> Self {
        Self {
            objects,
            // The nested `Children` element is two XML levels deeper, as it is enclosed
            // in the `Object` element of the child, hence two indentation levels.
            indent: format!("{}    ", self.indent),
        }
    }
}

/// Represents the template for the node identifiers files.
#[derive(Template)]
#[template(path = "node_identifiers.csv", escape = "none")]
struct NodeIdentifiers {
    /// Symbolic name of the namespace metadata object.
    ns_metadata_obj_name: String,
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
    let namespace = Namespace::from_yaml_file(&args.description_file)
        .context("Failed to get namespace description from file")?;

    let ns_metadata_obj_name = format!("{}NamespaceMetadata", ns_prefix.replace('.', ""));
    let ns_pub_date = namespace.publication_date.to_string();

    // Get the dependencies information, for the template and for the model compiler.
    let (ns_dependencies, dependencies_nodeset_paths): (Vec<_>, Vec<_>) = namespace
        .dependencies
        .into_iter()
        .map(|(key, dep_path)| {
            let dep_description = Namespace::from_yaml_file(target_dir.join(&dep_path))
                .with_context(|| {
                    format!(
                        "Failed to get dependency description from {}",
                        dep_path.display()
                    )
                })?;
            let prefix = dep_path
                .file_stem()
                .with_context(|| {
                    format!(
                        "Dependency description {} has no filename",
                        dep_path.display()
                    )
                })?
                .to_string_lossy()
                .into_owned();

            // The NodeSet file of the dependency is generated next to its description file.
            let nodeset_path = target_dir
                .join(&dep_path)
                .with_file_name(format!("{prefix}.NodeSet2.xml"));

            Ok(((key, dep_description.namespace_url, prefix), nodeset_path))
        })
        .collect::<Result<Vec<_>, anyhow::Error>>()?
        .into_iter()
        .unzip();

    // Create the Model Design template.
    let model_design = ModelDesign {
        ns_prefix: ns_prefix.to_string(),
        ns_url: namespace.namespace_url.clone(),
        ns_metadata_obj_name: ns_metadata_obj_name.clone(),
        ns_version: namespace.version,
        ns_pub_date,
        ns_dependencies,
        root_elements: namespace.root_elements,
    };

    // Create and write the model design file.
    let model_design_file_path = target_dir.join(format!("{ns_prefix}.Model.xml"));
    let mut model_design_file =
        File::create(&model_design_file_path).context("Failed to create Model Design file")?;
    model_design
        .write_into(&mut model_design_file)
        .context("Failed to write Model Design file")?;

    // Create and fill the node identifiers numbering file from template if it does
    // not already exist.
    let identifier_filename = format!("{ns_prefix}.Model.csv");
    let identifier_file_path = target_dir.join(identifier_filename);
    match File::create_new(&identifier_file_path) {
        Ok(mut file) => {
            let node_identifiers = NodeIdentifiers {
                ns_metadata_obj_name,
            };
            node_identifiers
                .write_into(&mut file)
                .context("Failed to write node identifiers file")?;
        }
        Err(err) if err.kind() == ErrorKind::AlreadyExists => {}
        result => {
            result.context("Failed to create node identifiers file")?;
        }
    }

    // Ensure model compiler tool availability.
    let restore_command = cmd!("dotnet", "tool", "restore");
    restore_command
        .run()
        .context("Failed to run `dotnet tool restore`")?;

    // Delete existing NodeSet2 file, to be able to detect if the model compiler tool
    // has done its job. There is a bug (fixed but not released) that makes the
    // compiler tool always return a success exit code.
    // https://github.com/OPCFoundation/UA-ModelCompiler/issues/258
    let generated_nodeset_path = target_dir.join(format!("{ns_prefix}.NodeSet2.xml"));
    match fs::remove_file(&generated_nodeset_path) {
        Ok(_) => {}
        Err(err) if err.kind() == ErrorKind::NotFound => {}
        result => {
            result.context("Failed to remove the previously generated NodeSet file")?;
        }
    }

    // Compile the Model Design.
    let mut compile_args: Vec<OsString> = vec![
        "tool".into(),
        "run".into(),
        "Opc.Ua.ModelCompiler".into(),
        "compile".into(),
        // Path to the ModelDesign file.
        "-d2".into(),
        model_design_file_path.into(),
    ];
    // Paths to the NodeSet files of the dependencies, which must follow the ModelDesign
    // file, as the first file is the one to compile.
    for nodeset_path in dependencies_nodeset_paths {
        compile_args.extend(["-d2".into(), nodeset_path.into()]);
    }
    compile_args.extend([
        // Output directory.
        "-o2".into(),
        target_dir.into(),
        // Path to the identifier file (will be created if needed).
        "-cg".into(),
        identifier_file_path.into(),
        // The first node ID identifier to use.
        "-id".into(),
        "1000".into(),
        // OPC-UA v1.04.
        "-version".into(),
        "v104".into(),
        // Suppress unwanted generated output.
        "-suppress".into(),
        "PredefinedNodes,Constants,JsonSchema,Classes,DataTypes".into(),
    ]);
    let compile_command = duct::cmd("dotnet", compile_args);
    compile_command
        .run()
        .context("Failed to compile OPC-UA Model Design")?;

    // Check if the model compiler has done its job.
    if !generated_nodeset_path.exists() {
        return Err(anyhow!("ModelCompiler did not produce the NodeSet file"));
    }

    Ok(())
}
