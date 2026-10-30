# Namespace

Source description of an OPC-UA namespace for ModelDesign XML generation.

### Type: `object`

| Property | Type | Required | Possible values | Default | Description |
| -------- | ---- | -------- | --------------- | ------- | ----------- |
| namespace_url | `string` | ✅ | string |  | Namespace URL. |
| publication_date | `string` | ✅ | Format: [`date-time`](https://json-schema.org/understanding-json-schema/reference/string#built-in-formats) |  | Namespace publication date and time. |
| root_elements | `array` | ✅ | [ObjectInstance](#objectinstance) or [ObjectType](#objecttype) |  | Root elements of this namespace. |
| version | `string` | ✅ | string |  | Version to set on the generated nodeset. |
| dependencies | `object` |  | object | `{}` | Namespaces this namespace depends on, mapped by prefix.<br /><br />Each value is the path of the dependency description file, relative to the<br />directory of this description file. The prefix is used to reference the elements<br />of the dependency (e.g. `prefix:MotorType`). The dependency must have been<br />generated beforehand, as its NodeSet file is used. |


---

# Definitions

## AccessLevel

The access level of the variable.

This is a subset of [available levels].

[available levels]: https://reference.opcfoundation.org/specs/OPC-10000-3/8.57

#### Type: `string`

**Possible Values:** `Read` or `Write` or `ReadWrite`

## ModellingRule

Modelling rule of an object component.

For placeholders, the browse name of the object is its name enclosed in angle
brackets (e.g. `<Motor>`).

#### Type: `string`

**Possible Values:** `Mandatory` or `Optional` or `MandatoryPlaceholder` or `OptionalPlaceholder`

## ObjectComponent

Source description of an object component of an ObjectType.

#### Type: `object`

| Property | Type | Required | Possible values | Description |
| -------- | ---- | -------- | --------------- | ----------- |
| description | `string` | ✅ | string | Description of the object. |
| name | `string` | ✅ | string | Name of the object. |
| object_type | `string` | ✅ | string | Name of the ObjectType of the object, prefixed for an ObjectType of a dependency<br />(e.g. `prefix:MotorType`). |
| modelling_rule | `string` or `null` |  | [ModellingRule](#modellingrule) | [Modelling rule] of the object, `Mandatory` if omitted.<br /><br />[Modelling rule]: https://reference.opcfoundation.org/specs/OPC-10000-3/6.4.4 |

## ObjectInstance

Source description of an instance of an ObjectType.

An instance at the root of the namespace is organized in the `Objects` folder,
other instances are components of their parent instance.

#### Type: `object`

| Property | Type | Required | Possible values | Description |
| -------- | ---- | -------- | --------------- | ----------- |
| name | `string` | ✅ | string | Name of the object instance. |
| object_type | `string` | ✅ | string | Name of the ObjectType of this instance, prefixed for an ObjectType of a<br />dependency (e.g. `prefix:MotorType`). |
| objects | `array` |  | [ObjectInstance](#objectinstance) | Object instances to add to this instance, as components. |

## ObjectType

Source description of an OPC-UA ObjectType for ModelDesign XML generation.

#### Type: `object`

| Property | Type | Required | Possible values | Default | Description |
| -------- | ---- | -------- | --------------- | ------- | ----------- |
| description | `string` | ✅ | string |  | Description of the ObjectType. |
| name | `string` | ✅ | string |  | Name of the ObjectType (e.g. `MotorType`). |
| base_type | `string` or `null` |  | string |  | Name of the ObjectType this one derives from, prefixed for an ObjectType of a<br />dependency (e.g. `prefix:DeviceType`). Defaults to `BaseObjectType`. |
| children | `array` |  | [ObjectComponent](#objectcomponent) or [Variable](#variable) |  | Children of the ObjectType, in declaration order. |
| is_abstract | `boolean` |  | boolean | `false` | Whether the ObjectType is abstract, i.e. cannot be instantiated. |

## ScalarDataType

OPC-UA scalar data types that can be used for variables.

The set of types has been chosen arbitrarily.

#### Type: `string`

**Possible Values:** `Boolean` or `SByte` or `Byte` or `Int16` or `UInt16` or `Int32` or `UInt32` or `Int64` or `UInt64` or `Float` or `Double` or `String` or `DateTime` or `Guid` or `ByteString` or `LocalizedText`

## Variable

Source description of a variable of an ObjectType.

#### Type: `object`

| Property | Type | Required | Possible values | Description |
| -------- | ---- | -------- | --------------- | ----------- |
| access_level | `string` | ✅ | [AccessLevel](#accesslevel) | Access level of the variable. |
| data_type | `string` | ✅ | [ScalarDataType](#scalardatatype) | OPC-UA data type of the variable. |
| description | `string` | ✅ | string | Description of the variable. |
| name | `string` | ✅ | string | Name of the variable. |
| array_dimensions | `array` or `null` |  | integer | List of [array dimensions] for the variable, which is a scalar if omitted.<br /><br />[array dimensions]: https://reference.opcfoundation.org/specs/OPC-10000-6/5.2.5 |


---

Markdown generated with [jsonschema-markdown](https://github.com/elisiariocouto/jsonschema-markdown).
