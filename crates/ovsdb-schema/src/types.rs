use ovsdb_common::common::{deserialize_set, serialize_set, AtomicType, Set};
use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all_fields(deserialize = "camelCase", serialize = "camelCase"))]
pub enum ConstrainedBaseType {
    #[serde(rename = "integer")]
    BaseTypeInt {
        min_integer: Option<i64>,
        max_integer: Option<i64>,
        #[serde(rename = "enum")]
        #[serde(default)]
        #[serde(deserialize_with = "deserialize_set")]
        #[serde(serialize_with = "serialize_set")]
        enum_: Option<Set<i64>>,
    },
    #[serde(rename = "real")]
    BaseTypeReal {
        min_real: Option<f64>,
        max_real: Option<f64>,
        #[serde(rename = "enum")]
        #[serde(default)]
        #[serde(deserialize_with = "deserialize_set")]
        #[serde(serialize_with = "serialize_set")]
        enum_: Option<Set<f64>>,
    },
    #[serde(rename = "string")]
    BaseTypeString {
        min_length: Option<i64>,
        max_length: Option<i64>,
        #[serde(rename = "enum")]
        #[serde(default)]
        #[serde(deserialize_with = "deserialize_set")]
        #[serde(serialize_with = "serialize_set")]
        enum_: Option<Set<String>>,
    },
    #[serde(rename = "uuid")]
    BaseTypeUUID {
        ref_table: String,
        #[serde(deserialize_with = "deserialize_ref_type")]
        #[serde(default = "ref_type_strong")]
        ref_type: RefType,
    },
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum BaseType {
    Atomic(AtomicType),
    Constrained(ConstrainedBaseType),
}

fn ref_type_strong() -> RefType {
    RefType::Strong
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RefType {
    Strong,
    Weak,
}

fn deserialize_ref_type<'de, D>(deserializer: D) -> Result<RefType, D::Error>
where
    D: Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    match value {
        serde_json::Value::String(s) => match s.as_str() {
            "strong" => Ok(RefType::Strong),
            "weak" => Ok(RefType::Weak),
            _ => Err(de::Error::custom("expected 'strong' or 'weak'")),
        },
        _ => Err(de::Error::custom("expected string")),
    }
}
#[derive(Debug)]
pub enum MaxOrUnlimited {
    Max(i64),
    Unlimited,
}

impl Default for MaxOrUnlimited {
    fn default() -> Self {
        MaxOrUnlimited::Max(1)
    }
}

fn deserialize_max_or_unlimited<'de, D>(deserializer: D) -> Result<MaxOrUnlimited, D::Error>
where
    D: Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    match value {
        // serde_json::Value::Null => Ok(MaxOrUnlimited::default()),
        serde_json::Value::Number(n) => {
            if n.is_i64() {
                Ok(MaxOrUnlimited::Max(n.as_i64().unwrap()))
            } else {
                Err(de::Error::custom("expected an i64 integer"))
            }
        }
        serde_json::Value::String(s) => {
            if s == "unlimited" {
                Ok(MaxOrUnlimited::Unlimited)
            } else {
                Err(de::Error::custom("expected the string 'unlimited'"))
            }
        }
        _ => Err(de::Error::custom("expected null, integer or 'unlimited'")),
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ColumnType {
    Atomic(AtomicType),
    Complex(ColumnComplexType),
}

#[derive(Debug, Deserialize)]
pub struct ColumnComplexType {
    pub key: BaseType,
    pub value: Option<BaseType>,

    // min and max are optional, but default to 1 if not specified
    // min is either 0 or 1, max must be at least 1 and greater than min
    // max can also be "unlimited"
    // If "min" and "max" are both 1 and "value" is not specified, the type is the scalar type specified by "key".
    // If min and max are both 1 and "value" is specified, the type is a map from the scalar type specified by "key" to the scalar type specified by "value".
    // If min is not 1 or max is not 1, or both, the type is a set of the scalar type specified by "key".
    // If value is specified, the type is a map from key to value.
    #[serde(default = "default_to_one")]
    pub min: i64,

    #[serde(default)]
    #[serde(deserialize_with = "deserialize_max_or_unlimited")]
    pub max: MaxOrUnlimited,
}

fn default_to_one() -> i64 {
    1
}

#[cfg(test)]
mod tests {
    use crate::types::*;
    // use serde::de::value::Error;
    use serde_json::json;

    #[test]
    fn test_base_type_int_many_enums() {
        let json = json!({
            "type": "integer",
            "minInteger": -1,
            "maxInteger": 1,
            "enum": ["set", [-1, 0, 1]]
        });
        let base_type: ConstrainedBaseType = serde_json::from_value(json).unwrap();
        match base_type {
            ConstrainedBaseType::BaseTypeInt {
                min_integer,
                max_integer,
                enum_,
            } => {
                assert_eq!(min_integer, Some(-1));
                assert_eq!(max_integer, Some(1));
                match enum_.unwrap() {
                    Set::Many(values) => assert_eq!(values, vec![-1, 0, 1]),
                    _ => panic!("Expected Set::Many"),
                }
            }
            _ => panic!("Expected BaseTypeInt"),
        }
    }

    #[test]
    fn test_base_type_int_single_enum() {
        let json = json!({
            "type": "integer",
            "minInteger": -1,
            "maxInteger": 1,
            "enum": 0
        });
        let base_type: ConstrainedBaseType = serde_json::from_value(json).unwrap();
        match base_type {
            ConstrainedBaseType::BaseTypeInt {
                min_integer,
                max_integer,
                enum_,
            } => {
                assert_eq!(min_integer, Some(-1));
                assert_eq!(max_integer, Some(1));
                match enum_.unwrap() {
                    Set::One(value) => assert_eq!(value, 0),
                    _ => panic!("Expected Set::One"),
                }
            }
            _ => panic!("Expected BaseTypeInt"),
        }
    }
    #[test]
    fn test_base_type_int_serialize_many_enums() {
        let base_type = ConstrainedBaseType::BaseTypeInt {
            min_integer: Some(-1),
            max_integer: Some(1),
            enum_: Some(Set::Many(vec![-1, 0, 1])),
        };
        let json = serde_json::to_value(base_type).unwrap();
        assert_eq!(
            json,
            json!({
                "type": "integer",
                "minInteger": -1,
                "maxInteger": 1,
                "enum": ["set", [-1, 0, 1]]
            })
        );
    }

    #[test]
    fn test_base_type_string_serialize_many_enums() {
        let base_type = ConstrainedBaseType::BaseTypeString {
            min_length: Some(0),
            max_length: Some(100),
            enum_: Some(Set::Many(vec![
                "tcp".to_string(),
                "udp".to_string(),
                "stcp".to_string(),
            ])),
        };
        let json = serde_json::to_value(base_type).unwrap();
        assert_eq!(
            json,
            json!({
                "type": "string",
                "minLength": 0,
                "maxLength": 100,
                "enum": ["set", ["tcp", "udp", "stcp"]]
            })
        );
    }
    // #[test]
    // fn test_base_type_int_error() {
    //     // give a wrong base type int data, and expect an error
    //     let json = json!({
    //         "type": "integer",
    //         "minInteger": -1,
    //         "maxInteger": 1,
    //         "enum": "set"
    //     });
    //     let e = serde_json::from_value(json).err().unwrap();
    //     match e {
    //         Error::Custom(_) => {},
    //         _ => panic!("Expected Error::Custom")
    //     }
    // }

    #[test]
    fn test_base_type_real() {
        let json = json!({
            "type": "real",
            "minReal": -1.5,
            "maxReal": 1.5
        });
        let base_type: ConstrainedBaseType = serde_json::from_value(json).unwrap();
        match base_type {
            ConstrainedBaseType::BaseTypeReal {
                min_real,
                max_real,
                enum_,
            } => {
                assert_eq!(min_real, Some(-1.5));
                assert_eq!(max_real, Some(1.5));
                assert_eq!(enum_, None);
            }
            _ => panic!("Expected BaseTypeReal"),
        }
    }

    #[test]
    fn test_base_type_string_with_many_enums() {
        let json = json!({
            "type": "string",
            "minLength": 0,
            "maxLength": 100,
            "enum": ["set", ["tcp", "udp", "stcp"]]
        });
        let base_type: ConstrainedBaseType = serde_json::from_value(json).unwrap();
        match base_type {
            ConstrainedBaseType::BaseTypeString {
                min_length,
                max_length,
                enum_,
            } => {
                assert_eq!(min_length, Some(0));
                assert_eq!(max_length, Some(100));
                match enum_.unwrap() {
                    Set::Many(values) => assert_eq!(values, vec!["tcp", "udp", "stcp"]),
                    _ => panic!("Expected Set::Many"),
                }
            }
            _ => panic!("Expected BaseTypeString"),
        }
    }

    #[test]
    fn test_base_type_string_with_single_enum() {
        let json = json!({
            "type": "string",
            "minLength": 0,
            "maxLength": 100,
            "enum": "tcp"
        });
        let base_type: ConstrainedBaseType = serde_json::from_value(json).unwrap();
        match base_type {
            ConstrainedBaseType::BaseTypeString {
                min_length,
                max_length,
                enum_,
            } => {
                assert_eq!(min_length, Some(0));
                assert_eq!(max_length, Some(100));
                match enum_.unwrap() {
                    Set::One(value) => assert_eq!(value, "tcp"),
                    _ => panic!("Expected Set::One"),
                }
            }
            _ => panic!("Expected BaseTypeString"),
        }
    }

    #[test]
    fn test_base_type_uuid() {
        let json = json!({
            "type": "uuid",
            "refTable": "DNS",
            "refType": "weak"
        });
        let base_type: ConstrainedBaseType = serde_json::from_value(json).unwrap();
        match base_type {
            ConstrainedBaseType::BaseTypeUUID {
                ref_table,
                ref_type,
            } => {
                assert_eq!(ref_table, "DNS");
                assert!(matches!(ref_type, RefType::Weak));
            }
            _ => panic!("Expected BaseTypeUUID"),
        }
    }

    #[test]
    fn test_base_type_uuid_default_value() {
        let json = json!({
            "type": "uuid",
            "refTable": "TestTable"
        });
        let base_type: ConstrainedBaseType = serde_json::from_value(json).unwrap();
        match base_type {
            ConstrainedBaseType::BaseTypeUUID {
                ref_table,
                ref_type,
            } => {
                assert_eq!(ref_table, "TestTable");
                assert!(matches!(ref_type, RefType::Strong));
            }
            _ => panic!("Expected BaseTypeUUID"),
        }
    }
    #[test]
    fn test_column_type_int() {
        let json = json!({
            "key": {
                "type": "integer",
                "minInteger": 0,
                "maxInteger": 4095
            },
            "min": 0,
            "max": 1
        });
        let column_type: ColumnType = serde_json::from_value(json).unwrap();
        match column_type {
            ColumnType::Complex(complex) => match complex.key {
                BaseType::Constrained(ConstrainedBaseType::BaseTypeInt {
                    min_integer,
                    max_integer,
                    enum_,
                }) => {
                    assert_eq!(min_integer, Some(0));
                    assert_eq!(max_integer, Some(4095));
                    assert_eq!(enum_, None);
                    assert!(complex.value.is_none());
                    assert_eq!(complex.min, 0);
                    assert!(matches!(complex.max, MaxOrUnlimited::Max(1)));
                }
                _ => panic!("Expected BaseTypeInt"),
            },
            _ => panic!("Expected ColumnComplexType"),
        }
    }
    #[test]
    fn test_column_type_uuid() {
        let json = json!({
                "key": {
                    "type": "uuid",
                    "refTable": "DNS",
                    "refType": "weak"
                },
                "min": 0,
                "max": "unlimited"
        });
        let column_type: ColumnType = serde_json::from_value(json).unwrap();
        match column_type {
            ColumnType::Complex(complex) => match complex.key {
                BaseType::Constrained(ConstrainedBaseType::BaseTypeUUID {
                    ref_table,
                    ref_type,
                }) => {
                    assert_eq!(ref_table, "DNS");
                    assert!(matches!(ref_type, RefType::Weak));
                    assert!(complex.value.is_none());
                    assert_eq!(complex.min, 0);
                    assert!(matches!(complex.max, MaxOrUnlimited::Unlimited));
                }
                _ => panic!("Expected BaseTypeUUID"),
            },
            _ => panic!("Expected ColumnComplexType"),
        }
    }
}
