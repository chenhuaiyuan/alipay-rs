use crate::error::{AlipayError, AlipayResult};
pub use alipay_derive::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum FieldValue {
    Null,
    Boolean(bool),
    Integer(i64),
    Double(f64),
    String(String),
    Array(Vec<FieldValue>),
    Map(HashMap<String, FieldValue>),
}
impl ToString for FieldValue {
    fn to_string(&self) -> String {
        match self {
            FieldValue::Null => "".to_owned(),
            FieldValue::Boolean(value) => value.to_string(),
            FieldValue::Integer(value) => value.to_string(),
            FieldValue::Double(value) => value.to_string(),
            FieldValue::String(value) => value.to_string(),
            FieldValue::Array(value) => {
                let mut res = String::new();
                for val in value.iter() {
                    let v = val.to_string();
                    res.push_str(v.as_str());
                }
                res
            }
            FieldValue::Map(value) => {
                let mut res = String::new();
                for (key, val) in value {
                    res.push_str(key.as_str());
                    let v = val.to_string();
                    res.push_str(v.as_str());
                }
                res
            }
        }
    }
}

pub trait AlipayParam {
    fn to_map(&self) -> HashMap<String, FieldValue>;
    fn from_map(map: HashMap<String, FieldValue>) -> AlipayResult<Self>
    where
        Self: std::marker::Sized;
}

pub trait Converter: Sized {
    fn to_field_value(&self) -> FieldValue;
    fn to_primitive(fv: FieldValue) -> AlipayResult<Self>;
}

impl Converter for String {
    fn to_field_value(&self) -> FieldValue {
        FieldValue::String(self.to_string())
    }
    fn to_primitive(fv: FieldValue) -> AlipayResult<Self> {
        match fv {
            FieldValue::String(value) => Ok(value),
            _ => Err(AlipayError::ConvertError("invalid type: String".to_owned())),
        }
    }
}

impl Converter for char {
    fn to_field_value(&self) -> FieldValue {
        FieldValue::String(self.to_string())
    }
    fn to_primitive(fv: FieldValue) -> AlipayResult<Self> {
        match fv {
            FieldValue::String(value) => {
                let chars: Vec<char> = value.chars().collect();
                if chars.len() != 1 {
                    return Err(AlipayError::ConvertError("invalid type: char".to_owned()));
                }
                return Ok(chars[0]);
            }
            _ => Err(AlipayError::ConvertError("invalid type: char".to_owned())),
        }
    }
}
impl Converter for bool {
    fn to_field_value(&self) -> FieldValue {
        FieldValue::Boolean(*self)
    }

    fn to_primitive(fv: FieldValue) -> AlipayResult<Self> {
        match fv {
            FieldValue::Boolean(value) => Ok(value),
            _ => Err(AlipayError::ConvertError("invalid type: bool".to_owned())),
        }
    }
}

macro_rules! integer_impls {
    ($ ($type:ty) +) => {
        $(
            impl Converter for $type {
                #[inline]
                fn to_field_value(&self) -> FieldValue {
                    FieldValue::Integer(*self as i64)
                }

                #[inline]
                fn to_primitive(fv: FieldValue) ->AlipayResult<Self>{
                    match fv {
                        FieldValue::Integer(value)=>{
                            if let Ok(value) = <$type>::try_from(value) {
                                return Ok(value);
                            }
                            return Err(AlipayError::ConvertError(format!("invalid type: {}",stringify!($type).to_owned())))
                        },
                        _=> Err(AlipayError::ConvertError(format!("invalid type: {}",stringify!($type).to_owned()))),
                    }
                }
            }
        )+
    }
}

integer_impls!(i8 i16 i32 i64 isize u8 u16 u32);

impl Converter for f32 {
    fn to_field_value(&self) -> FieldValue {
        FieldValue::Double(*self as f64)
    }
    fn to_primitive(fv: FieldValue) -> AlipayResult<Self> {
        match fv {
            FieldValue::Double(value) => Ok(value as f32),
            _ => Err(AlipayError::ConvertError("invalid type: f32".to_owned())),
        }
    }
}

impl Converter for f64 {
    fn to_field_value(&self) -> FieldValue {
        FieldValue::Double(*self)
    }
    fn to_primitive(fv: FieldValue) -> AlipayResult<Self> {
        match fv {
            FieldValue::Double(value) => Ok(value),
            _ => Err(AlipayError::ConvertError("invalid type: f64".to_owned())),
        }
    }
}

impl<T> Converter for Option<T>
where
    T: Converter,
{
    fn to_field_value(&self) -> FieldValue {
        match self {
            Some(some) => some.to_field_value(),
            None => FieldValue::Null,
        }
    }

    fn to_primitive(fv: FieldValue) -> AlipayResult<Self> {
        match fv {
            FieldValue::Null => Ok(None),
            _ => Ok(Some(T::to_primitive(fv)?)),
        }
    }
}

impl<T> Converter for Vec<T>
where
    T: Converter,
{
    fn to_field_value(&self) -> FieldValue {
        FieldValue::Array(self.iter().map(|v| v.to_field_value()).collect())
    }

    fn to_primitive(fv: FieldValue) -> AlipayResult<Self> {
        match fv {
            FieldValue::Array(value) => value
                .into_iter()
                .map(|v| T::to_primitive(v))
                .collect::<AlipayResult<Vec<T>>>(),
            _ => Err(AlipayError::ConvertError("invalid type: Vec<T>".to_owned())),
        }
    }
}

impl<K, V> Converter for HashMap<K, V>
where
    K: ToString + From<String> + std::cmp::Eq + std::hash::Hash,
    V: Converter,
{
    fn to_field_value(&self) -> FieldValue {
        FieldValue::Map(
            self.iter()
                .map(|(key, value)| (key.to_string(), value.to_field_value()))
                .collect(),
        )
    }

    fn to_primitive(fv: FieldValue) -> AlipayResult<Self> {
        match fv {
            FieldValue::Map(value) => {
                let mut result = HashMap::with_capacity(value.len());
                for (k, v) in value {
                    if let Ok(k) = K::try_from(k) {
                        result.insert(k, V::to_primitive(v)?);
                    } else {
                        return Err(AlipayError::ConvertError(
                            "invalid type: HashMap<K, V>".to_owned(),
                        ));
                    }
                }
                return Ok(result);
            }
            _ => Err(AlipayError::ConvertError(
                "invalid type: HashMap<K, V>".to_owned(),
            )),
        }
    }
}

impl<T> Converter for T
where
    T: AlipayParam,
{
    fn to_field_value(&self) -> FieldValue {
        FieldValue::Map(self.to_map())
    }

    fn to_primitive(fv: FieldValue) -> AlipayResult<Self> {
        match fv {
            FieldValue::Map(value) => Ok(T::from_map(value)?),
            _ => Err(AlipayError::ConvertError("invalid type: Mapper".to_owned())),
        }
    }
}
