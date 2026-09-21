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
    /// Root elements of this namespace.
    pub root_elements: Vec<RootElement>,
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
    /// The name of the ObjectType (e.g. MotorType).
    pub name: String,
    /// The description of the ObjectType.
    pub description: String,
    /// The list of variables found in the ObjectDesign modelization.
    pub variables: Vec<Variable>,
}

/// Represents the modelization for a variable member of an ObjectType.
#[derive(Deserialize, JsonSchema)]
pub struct Variable {
    /// The name of the variable.
    pub name: String,
    /// The description of the variable.
    pub description: String,
    /// The OPC-UA data type of the variable.
    pub data_type: ScalarDataType,
    /// The access level of the variable.
    pub access_level: AccessLevel,
    /// A list of [array dimensions] for the variable.
    ///
    /// [array dimensions]: https://reference.opcfoundation.org/specs/OPC-10000-6/5.2.5
    #[schemars(length(min = 1))]
    pub array_dimensions: Option<Vec<i32>>,
}

/// OPC-UA scalar datatypes that can be used for variables. The set of types has been chosen arbitrarily.
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

/// An instance of ObjectType.
#[derive(Deserialize, JsonSchema)]
pub struct ObjectInstance {
    /// Name of the variable instance.
    pub name: String,
    /// Name of the object type for this instance.
    pub object_type: String,
}
