# OPC-UA nodeset generator

Generate OPC-UA nodeset from concise description, via Model Design.

This tool turns a description of an OPC-UA namespace into OPC-UA NodeSet2 files,
via an UA Model Design intermediate representation.

## Usage

### Model description file

The source format is a YAML file describing the namespace to be created,
including ObjectTypes, variables and Object instances. The JSON schema from
[this link](schema/config.schema.json) can be used to help editing this file.

The configuration structure is described in this [documentation](docs/configuration.md).

#### Schema generation

To generate the JSON schema, the following command can be used.

```ShellSession
cargo generate-schema > <SCHEMA_FILE>
```

### Output

The tool will generate a Model Design file, with a name prefixed after the final
component of the provided directory, and will then use UA Model Compiler tool to
generate NodeSet files, with the same name prefix as the Model Design.

### Invocation

The tool depends on the [UA Model Compiler](https://github.com/OPCFoundation/UA-ModelCompiler) tool,
used via the `dotnet` CLI. To ease its installation, it is recommended to use [mise](https://mise.jdx.dev/),
for which a configuration is included in this project.

```ShellSession
cargo run -- <DESCRIPTION_FILE>
```
