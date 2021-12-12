// use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
// use std::convert::AsRef;
// use std::fmt;
// use std::ops::Deref;

#[derive(Clone, Debug)]
pub struct RequestParam(HashMap<String, String>);

impl RequestParam {
    pub fn new(t: HashMap<String, String>) -> RequestParam {
        RequestParam(t)
    }
}

impl Default for RequestParam {
    #[inline]
    fn default() -> RequestParam {
        RequestParam::new(HashMap::new())
    }
}

impl RequestParam {
    // pub fn borrow(&self) -> Ref<HashMap<String, String>> {
    //     self.0.borrow()
    // }

    // pub fn borrow_mut(&self) -> RefMut<HashMap<String, String>> {
    //     self.0.borrow_mut()
    // }

    // pub fn as_ptr(&self) -> *mut HashMap<String, String> {
    //     self.0.as_ptr()
    // }

    // pub fn to_str(self) -> String {
    //     let map = self.0;
    //     let mut p = map.iter().collect::<Vec<_>>();
    //     p.sort_by(|a, b| a.0.cmp(&b.0));
    //     let mut temp: String = String::new();
    //     for (key, val) in p.iter() {
    //         temp.push_str(key);
    //         temp.push_str("=");
    //         temp.push_str(val);
    //         temp.push_str("&");
    //     }
    //     temp.pop();
    //     temp
    // }
    pub fn save(&mut self, k: String, v: String) {
        if let Some(value) = self.0.get_mut(&k) {
            *value = v;
        } else {
            self.0.insert(k, v);
        }
    }
    pub fn set(&mut self, k: String, v: String) {
        if let Some(value) = self.0.get_mut(&k) {
            *value = v;
        }
    }
    pub fn add(&mut self, k: String, v: String) {
        self.0.insert(k, v);
    }
    pub fn clear(&mut self) {
        self.0.clear();
    }
    pub fn replace(&mut self, args: HashMap<String, String>) {
        self.0 = args;
    }
    pub fn inner(self) -> HashMap<String, String> {
        self.0
    }
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&String, &mut String) -> bool,
    {
        self.0.retain(f);
    }
}

// impl fmt::Debug for RequestParam {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:?}", self.deref())
//     }
// }

// impl Deref for RequestParam {
//     type Target = HashMap<String, String>;

//     #[inline]
//     fn deref(&self) -> &HashMap<String, String> {
//         unsafe { self.as_ptr().as_ref().unwrap() }
//     }
// }

// impl AsRef<HashMap<String, String>> for RequestParam {
//     fn as_ref(&self) -> &HashMap<String, String> {
//         unsafe { self.as_ptr().as_ref().unwrap() }
//     }
// }
