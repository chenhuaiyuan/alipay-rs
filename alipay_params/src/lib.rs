pub use alipay_macros::*;
use serde_json::{Map, Number, Value};
use std::collections::HashMap;

#[derive(Clone)]
pub enum AlipayValue {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Tuple((String, Value)),
    TupleArray(Vec<(String, Value)>),
    Array(Vec<AlipayValue>),
    Object(HashMap<String, AlipayValue>),
}

impl From<HashMap<String, AlipayValue>> for AlipayValue {
    fn from(value: HashMap<String, AlipayValue>) -> Self {
        AlipayValue::Object(value)
    }
}

fn json_value_to_alipay_value(val: Value) -> AlipayValue {
    match val {
        Value::Null => AlipayValue::Null,
        Value::Bool(v) => AlipayValue::Bool(v),
        Value::Number(v) => AlipayValue::Number(v),
        Value::String(v) => AlipayValue::String(v),
        Value::Array(v) => {
            let mut array = Vec::new();
            for val in v {
                array.push(json_value_to_alipay_value(val))
            }
            AlipayValue::Array(array)
        }
        Value::Object(v) => {
            let mut map = HashMap::new();
            for (key, val) in v {
                map.insert(key, json_value_to_alipay_value(val));
            }
            AlipayValue::Object(map)
        }
    }
}

impl AlipayValue {
    pub fn is_null(&self) -> bool {
        matches!(self, AlipayValue::Null)
    }
    pub fn is_bool(&self) -> bool {
        matches!(self, AlipayValue::Bool(_))
    }
    pub fn is_number(&self) -> bool {
        matches!(self, AlipayValue::Number(_))
    }
    pub fn is_string(&self) -> bool {
        matches!(self, AlipayValue::String(_))
    }
    pub fn is_tuple(&self) -> bool {
        matches!(self, AlipayValue::Tuple(_))
    }
    pub fn is_tuple_array(&self) -> bool {
        matches!(self, AlipayValue::TupleArray(_))
    }
    pub fn is_array(&self) -> bool {
        matches!(self, AlipayValue::Array(_))
    }
    pub fn is_object(&self) -> bool {
        matches!(self, AlipayValue::Object(_))
    }
    pub fn to_json_value(self) -> Value {
        match self {
            AlipayValue::Null => Value::Null,
            AlipayValue::Bool(v) => Value::Bool(v),
            AlipayValue::Number(v) => Value::Number(v),
            AlipayValue::String(v) => Value::String(v),
            AlipayValue::Tuple((key, val)) => {
                let mut map = Map::new();
                map.insert(key, val);
                Value::Object(map)
            }
            AlipayValue::TupleArray(v) => {
                let mut map = Map::new();
                for (key, val) in v {
                    map.insert(key, val);
                }
                Value::Object(map)
            }
            AlipayValue::Array(v) => {
                let mut array = Vec::new();
                for val in v {
                    array.push(val.to_json_value())
                }
                Value::Array(array)
            }
            AlipayValue::Object(v) => {
                let mut map = Map::new();
                for (key, val) in v {
                    map.insert(key, val.to_json_value());
                }
                Value::Object(map)
            }
        }
    }
}

pub trait AlipayParams {
    fn to_alipay_value(self) -> AlipayValue;
}

macro_rules! impl_number_alipay_params {
    ($($ty: ty),*) => {
        $(
            impl AlipayParams for $ty {
                fn to_alipay_value(self) -> AlipayValue {
                    AlipayValue::Number(Number::from(self))
                }
            }
        )*
    };
}

impl_number_alipay_params!(u8, u16, u32, u64);
impl_number_alipay_params!(i8, i16, i32, i64);
impl_number_alipay_params!(usize, isize);

impl AlipayParams for f32 {
    fn to_alipay_value(self) -> AlipayValue {
        let data = Number::from_f64(self as f64);
        if let Some(val) = data {
            AlipayValue::Number(val)
        } else {
            AlipayValue::Null
        }
    }
}

impl AlipayParams for f64 {
    fn to_alipay_value(self) -> AlipayValue {
        let data = Number::from_f64(self);
        if let Some(val) = data {
            AlipayValue::Number(val)
        } else {
            AlipayValue::Null
        }
    }
}

impl AlipayParams for bool {
    fn to_alipay_value(self) -> AlipayValue {
        AlipayValue::Bool(self)
    }
}

impl AlipayParams for String {
    fn to_alipay_value(self) -> AlipayValue {
        AlipayValue::String(self)
    }
}

impl<'a> AlipayParams for &'a str {
    fn to_alipay_value(self) -> AlipayValue {
        AlipayValue::String(self.to_owned())
    }
}

impl<T> AlipayParams for Option<T>
where
    T: AlipayParams,
{
    fn to_alipay_value(self) -> AlipayValue {
        if let Some(v) = self {
            v.to_alipay_value()
        } else {
            AlipayValue::Null
        }
    }
}

impl<T1, T2> AlipayParams for (T1, T2)
where
    T1: Into<String>,
    T2: AlipayParams,
{
    fn to_alipay_value(self) -> AlipayValue {
        AlipayValue::Tuple((self.0.into(), self.1.to_alipay_value().to_json_value()))
    }
}

impl<T, const N: usize> AlipayParams for [T; N]
where
    T: AlipayParams + Clone,
{
    fn to_alipay_value(self) -> AlipayValue {
        if !self.is_empty() {
            let mut i = 0;
            let len = self.len();
            let temp = self[i].clone().to_alipay_value();
            if temp.is_tuple() {
                i += 1;
                let mut array = Vec::new();
                if let AlipayValue::Tuple((key, val)) = temp {
                    array.push((key, val));
                }
                while len > i {
                    let temp = self[i].clone().to_alipay_value();
                    if temp.is_tuple() {
                        if let AlipayValue::Tuple((key, val)) = temp {
                            array.push((key, val));
                        }
                    }
                    i += 1;
                }
                AlipayValue::TupleArray(array)
            } else {
                i += 1;
                let mut array = Vec::new();
                array.push(temp);
                while len > i {
                    let temp = self[i].clone().to_alipay_value();
                    array.push(temp);
                    i += 1;
                }
                AlipayValue::Array(array)
            }
        } else {
            AlipayValue::Null
        }
    }
}

impl<T1, T2> AlipayParams for HashMap<T1, T2>
where
    T1: Into<String>,
    T2: AlipayParams,
{
    fn to_alipay_value(self) -> AlipayValue {
        let mut map = HashMap::new();
        for (key, val) in self {
            map.insert(key.into(), val.to_alipay_value());
        }
        AlipayValue::Object(map)
    }
}

impl AlipayParams for Value {
    fn to_alipay_value(self) -> AlipayValue {
        json_value_to_alipay_value(self)
    }
}

impl<T> AlipayParams for Vec<T>
where
    T: AlipayParams + Clone,
{
    fn to_alipay_value(self) -> AlipayValue {
        if !self.is_empty() {
            let mut i = 0;
            let len = self.len();
            let temp = self[i].clone().to_alipay_value();
            if temp.is_tuple() {
                i += 1;
                let mut array = Vec::new();
                if let AlipayValue::Tuple((key, val)) = temp {
                    array.push((key, val));
                }
                while len > i {
                    let temp = self[i].clone().to_alipay_value();
                    if temp.is_tuple() {
                        if let AlipayValue::Tuple((key, val)) = temp {
                            array.push((key, val));
                        }
                    }
                    i += 1;
                }
                AlipayValue::TupleArray(array)
            } else {
                i += 1;
                let mut array = Vec::new();
                array.push(temp);
                while len > i {
                    let temp = self[i].clone().to_alipay_value();
                    array.push(temp);
                    i += 1;
                }
                AlipayValue::Array(array)
            }
        } else {
            AlipayValue::Null
        }
    }
}

impl AlipayParams for () {
    fn to_alipay_value(self) -> AlipayValue {
        AlipayValue::Null
    }
}

impl AlipayParams for AlipayValue {
    fn to_alipay_value(self) -> AlipayValue {
        self
    }
}
