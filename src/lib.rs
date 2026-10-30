use std::collections::BTreeMap;
use std::fs::File;
use std::path::{Path, PathBuf};

use anyhow::Context as _;
use jiff::Timestamp;
use schemars::JsonSchema;
use serde::Deserialize;
use strum::Display;

/// Source description of an OPC-UA namespace for ModelDesign XML generation.
#[derive(Deserialize, JsonSchema)]
pub struct Namespace {
    /// Namespace URL.
    pub namespace_url: String,
    /// Version to set on the generated nodeset.
    pub version: String,
    /// Namespace publication date and time.
    pub publication_date: Timestamp,
    /// Namespaces this namespace depends on, mapped by prefix.
    ///
    /// Each value is the path of the dependency description file, relative to the
    /// directory of this description file. The prefix is used to reference the elements
    /// of the dependency (e.g. `prefix:MotorType`). The dependency must have been
    /// generated beforehand, as its NodeSet file is used.
    #[serde(default)]
    pub dependencies: BTreeMap<String, PathBuf>,
    /// Root elements of this namespace.
    pub root_elements: Vec<RootElement>,
}

impl Namespace {
    /// Create a [`Namespace`] from the provided path to a YAML file.
    pub fn from_yaml_file<P>(path: P) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        let file = File::open(path).context("Failed to open file")?;
        yaml_serde::from_reader(file).context("Failed to deserialize file contents")
    }
}

/// A namespace root element.
// The schema is inlined at its use site and each variant is given a title, otherwise the
// documentation generator renders this enum as an empty definition: it only documents the
// properties of a definition, never the variants of a `oneOf`.
#[derive(Deserialize, JsonSchema)]
#[schemars(inline)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum RootElement {
    /// ObjectType description.
    #[schemars(title = "ObjectTypeElement")]
    ObjectType(ObjectType),
    /// ObjectType instance description.
    #[schemars(title = "ObjectInstanceElement")]
    ObjectInstance(ObjectInstance),
}

impl RootElement {
    /// If the [`RootElement`] is an object type, return a reference to the associated
    /// [`ObjectType`]. Returns [`None`] otherwise.
    pub fn as_object_type(&self) -> Option<&ObjectType> {
        if let Self::ObjectType(t) = self {
            Some(t)
        } else {
            None
        }
    }

    /// If the [`RootElement`] is an object instance, return a reference to the associated
    /// [`ObjectInstance`]. Returns [`None`] otherwise.
    pub fn as_object_instance(&self) -> Option<&ObjectInstance> {
        if let Self::ObjectInstance(i) = self {
            Some(i)
        } else {
            None
        }
    }
}

/// Source description of an OPC-UA ObjectType for ModelDesign XML generation.
#[derive(Deserialize, JsonSchema)]
pub struct ObjectType {
    /// Name of the ObjectType (e.g. `MotorType`).
    pub name: String,
    /// Description of the ObjectType.
    pub description: String,
    /// Name of the ObjectType this one derives from, prefixed for an ObjectType of a
    /// dependency (e.g. `prefix:DeviceType`). Defaults to `BaseObjectType`.
    pub base_type: Option<String>,
    /// Whether the ObjectType is abstract, i.e. cannot be instantiated.
    #[serde(default)]
    pub is_abstract: bool,
    /// Children of the ObjectType, in declaration order.
    #[serde(default)]
    pub children: Vec<ObjectTypeChild>,
}

/// A child of an ObjectType.
// See `RootElement` for the reasons of the schema inlining and variants titles.
#[derive(Deserialize, JsonSchema)]
#[schemars(inline)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum ObjectTypeChild {
    /// Variable description, as a property.
    #[schemars(title = "VariableChild")]
    Variable(Variable),
    /// Object description, as a component.
    #[schemars(title = "ObjectChild")]
    Object(ObjectComponent),
}

impl ObjectTypeChild {
    /// If the [`ObjectTypeChild`] is a variable, return a reference to the associated
    /// [`Variable`]. Returns [`None`] otherwise.
    pub fn as_variable(&self) -> Option<&Variable> {
        if let Self::Variable(v) = self {
            Some(v)
        } else {
            None
        }
    }

    /// If the [`ObjectTypeChild`] is an object, return a reference to the associated
    /// [`ObjectComponent`]. Returns [`None`] otherwise.
    pub fn as_object(&self) -> Option<&ObjectComponent> {
        if let Self::Object(o) = self {
            Some(o)
        } else {
            None
        }
    }
}

/// Source description of a variable of an ObjectType.
#[derive(Deserialize, JsonSchema)]
pub struct Variable {
    /// Name of the variable.
    pub name: String,
    /// Description of the variable.
    pub description: String,
    /// OPC-UA data type of the variable.
    pub data_type: ScalarDataType,
    /// Access level of the variable.
    pub access_level: AccessLevel,
    /// List of [array dimensions] for the variable, which is a scalar if omitted.
    ///
    /// [array dimensions]: https://reference.opcfoundation.org/specs/OPC-10000-6/5.2.5
    #[schemars(length(min = 1))]
    pub array_dimensions: Option<Vec<i32>>,
}

/// Source description of an object component of an ObjectType.
#[derive(Deserialize, JsonSchema)]
pub struct ObjectComponent {
    /// Name of the object.
    pub name: String,
    /// Description of the object.
    pub description: String,
    /// Name of the ObjectType of the object, prefixed for an ObjectType of a dependency
    /// (e.g. `prefix:MotorType`).
    pub object_type: String,
    /// [Modelling rule] of the object, `Mandatory` if omitted.
    ///
    /// [Modelling rule]: https://reference.opcfoundation.org/specs/OPC-10000-3/6.4.4
    pub modelling_rule: Option<ModellingRule>,
}

impl ObjectComponent {
    /// If the [`ObjectComponent`] is a placeholder, return its browse name, i.e. its
    /// name enclosed in angle brackets (e.g. `<Motor>`). Returns [`None`] otherwise, as
    /// the browse name is then the name.
    pub fn browse_name(&self) -> Option<String> {
        use ModellingRule::*;

        self.modelling_rule.as_ref().and_then(|m| {
            matches!(m, MandatoryPlaceholder | OptionalPlaceholder)
                .then(|| format!("<{}>", self.name))
        })
    }
}

/// OPC-UA scalar data types that can be used for variables.
///
/// The set of types has been chosen arbitrarily.
#[derive(Deserialize, Display, JsonSchema)]
pub enum ScalarDataType {
    Boolean,
    SByte,
    Byte,
    Int16,
    UInt16,
    Int32,
    UInt32,
    Int64,
    UInt64,
    Float,
    Double,
    String,
    DateTime,
    Guid,
    ByteString,
    LocalizedText,
}

/// The access level of the variable.
///
/// This is a subset of [available levels].
///
/// [available levels]: https://reference.opcfoundation.org/specs/OPC-10000-3/8.57
#[derive(Deserialize, Display, JsonSchema)]
pub enum AccessLevel {
    Read,
    Write,
    ReadWrite,
}

/// Modelling rule of an object component.
///
/// For placeholders, the browse name of the object is its name enclosed in angle
/// brackets (e.g. `<Motor>`).
#[derive(Deserialize, Display, JsonSchema)]
pub enum ModellingRule {
    Mandatory,
    Optional,
    MandatoryPlaceholder,
    OptionalPlaceholder,
}

/// Source description of an instance of an ObjectType.
///
/// An instance at the root of the namespace is organized in the `Objects` folder,
/// other instances are components of their parent instance.
#[derive(Deserialize, JsonSchema)]
pub struct ObjectInstance {
    /// Name of the object instance.
    pub name: String,
    /// Name of the ObjectType of this instance, prefixed for an ObjectType of a
    /// dependency (e.g. `prefix:MotorType`).
    pub object_type: String,
    /// Object instances to add to this instance, as components.
    #[serde(default)]
    pub objects: Vec<ObjectInstance>,
}
