# Namespace

Source description of an OPC-UA namespace for ModelDesign XML generation.

### Type: `object`

| Property | Type | Required | Possible values | Description |
| -------- | ---- | -------- | --------------- | ----------- |
| namespace_url | `string` | ✅ | string | Namespace URL. |
| publication_date | `string` | ✅ | Format: [`date-time`](https://json-schema.org/understanding-json-schema/reference/string#built-in-formats) | Namespace publication date and time. |
| root_elements | `array` | ✅ | [ObjectInstance](#objectinstance) or [ObjectType](#objecttype) | Root elements of this namespace. |
| version | `string` | ✅ | string | Version to set on the generated nodeset. |


---

# Definitions

## AccessLevel

The access level of the variable.

This is a subset of [available levels].

[available levels]: https://reference.opcfoundation.org/specs/OPC-10000-3/8.57

#### Type: `string`

**Possible Values:** `Read` or `Write` or `ReadWrite`

## ObjectInstance

An instance of ObjectType.

#### Type: `object`

| Property | Type | Required | Possible values | Description |
| -------- | ---- | -------- | --------------- | ----------- |
| name | `string` | ✅ | string | Name of the variable instance. |
| object_type | `string` | ✅ | string | Name of the object type for this instance. |

## ObjectType

Source description of an OPC-UA ObjectType for ModelDesign XML generation.

#### Type: `object`

| Property | Type | Required | Possible values | Description |
| -------- | ---- | -------- | --------------- | ----------- |
| description | `string` | ✅ | string | The description of the ObjectType. |
| name | `string` | ✅ | string | The name of the ObjectType (e.g. MotorType). |
| variables | `array` | ✅ | [Variable](#variable) | The list of variables found in the ObjectDesign modelization. |

## ScalarDataType

OPC-UA scalar datatypes that can be used for variables. The set of types has been chosen arbitrarily.

#### Type: `string`

**Possible Values:** `Boolean` or `SByte` or `Byte` or `Int16` or `UInt16` or `Int32` or `UInt32` or `Int64` or `UInt64` or `Float` or `Double` or `String` or `DateTime` or `Guid` or `ByteString` or `LocalizedText`

## Variable

Represents the modelization for a variable member of an ObjectType.

#### Type: `object`

| Property | Type | Required | Possible values | Description |
| -------- | ---- | -------- | --------------- | ----------- |
| access_level | `string` | ✅ | [AccessLevel](#accesslevel) | The access level of the variable. |
| data_type | `string` | ✅ | [ScalarDataType](#scalardatatype) | The OPC-UA data type of the variable. |
| description | `string` | ✅ | string | The description of the variable. |
| name | `string` | ✅ | string | The name of the variable. |
| array_dimensions | `array` or `null` |  | integer | A list of [array dimensions] for the variable.<br /><br />[array dimensions]: https://reference.opcfoundation.org/specs/OPC-10000-6/5.2.5 |


---

Markdown generated with [jsonschema-markdown](https://github.com/elisiariocouto/jsonschema-markdown).
