pub use alipay_macros::*;
use serde_json::{Map, Number, Value};
use std::collections::HashMap;

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

impl From<bool> for AlipayValue {
    fn from(value: bool) -> Self {
        AlipayValue::Bool(value)
    }
}

macro_rules! alipay_value_from_number {
    ($($ty: ty),*) => {
        $(
            impl From<$ty> for AlipayValue {
                fn from(value: $ty) -> Self {
                    AlipayValue::Number(Number::from(value))
                }
            }
        )*
    };
}

impl From<()> for AlipayValue {
    fn from((): ()) -> Self {
        AlipayValue::Null
    }
}

alipay_value_from_number!(u8, u16, u32, u64);
alipay_value_from_number!(i8, i16, i32, i64);
alipay_value_from_number!(usize, isize);

impl From<f32> for AlipayValue {
    fn from(value: f32) -> Self {
        let val = Number::from_f64(value.into());
        if let Some(v) = val {
            AlipayValue::Number(v)
        } else {
            AlipayValue::Null
        }
    }
}

impl From<f64> for AlipayValue {
    fn from(value: f64) -> Self {
        let val = Number::from_f64(value);
        if let Some(v) = val {
            AlipayValue::Number(v)
        } else {
            AlipayValue::Null
        }
    }
}

impl From<String> for AlipayValue {
    fn from(value: String) -> Self {
        AlipayValue::String(value)
    }
}

impl<'a> From<&'a str> for AlipayValue {
    fn from(value: &'a str) -> Self {
        AlipayValue::String(value.to_string())
    }
}

impl<T: Clone + Into<Value>> From<(String, T)> for AlipayValue {
    fn from(value: (String, T)) -> Self {
        AlipayValue::Tuple((value.0, value.1.into()))
    }
}

impl<T: Clone + Into<Value>> From<Vec<(String, T)>> for AlipayValue {
    fn from(value: Vec<(String, T)>) -> Self {
        let mut data: Vec<(String, Value)> = Vec::new();
        for (key, val) in value {
            data.push((key, val.into()));
        }
        AlipayValue::TupleArray(data)
    }
}

impl From<Vec<AlipayValue>> for AlipayValue {
    fn from(value: Vec<AlipayValue>) -> Self {
        AlipayValue::Array(value)
    }
}

impl<T: Into<AlipayValue>> From<HashMap<String, T>> for AlipayValue {
    fn from(value: HashMap<String, T>) -> Self {
        let mut data: HashMap<String, AlipayValue> = HashMap::new();
        for (key, val) in value {
            data.insert(key, val.into());
        }
        AlipayValue::Object(data)
    }
}

impl<'a, T: Into<AlipayValue>> From<HashMap<&'a str, T>> for AlipayValue {
    fn from(value: HashMap<&'a str, T>) -> Self {
        let mut data: HashMap<String, AlipayValue> = HashMap::new();
        for (key, val) in value {
            data.insert(key.to_string(), val.into());
        }
        AlipayValue::Object(data)
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

macro_rules! alipay_params_implement_number {
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

alipay_params_implement_number!(u8, u16, u32, u64);
alipay_params_implement_number!(i8, i16, i32, i64);
alipay_params_implement_number!(usize, isize);

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
    T: AlipayParams,
{
    fn to_alipay_value(mut self) -> AlipayValue {
        if !self.is_empty() {
            let mut i = 0;
            let len = self.len();
            let temp = self.remove(0).to_alipay_value();
            if temp.is_tuple() {
                i += 1;
                let mut array = Vec::new();
                if let AlipayValue::Tuple((key, val)) = temp {
                    array.push((key, val));
                }
                while len > i {
                    let temp = self.remove(0).to_alipay_value();
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
                    let temp = self.remove(0).to_alipay_value();
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
