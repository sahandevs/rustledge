use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Value {
    String(String),
    Bool(bool),
    List(Vec<String>),
    Bucket(Bucket),
}

impl Clone for Value {
    fn clone(&self) -> Self {
        match self {
            Value::String(value) => Value::String(value.clone()),
            Value::Bool(value) => Value::Bool(value.clone()),
            Value::List(value) => Value::List(value.clone()),
            Value::Bucket(value) => Value::Bucket(value.clone()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Bucket {
    values: HashMap<String, Value>,
}

impl Bucket {
    pub fn new() -> Bucket {
        Bucket {
            values: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.values.get(key)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
        self.values.get_mut(key)
    }

    pub fn set(&mut self, key: &str, value: Value) {
        self.values.insert(String::from(key), value);
    }
}

impl Clone for Bucket {
    fn clone(&self) -> Self {
        Bucket {
            values: self.values.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn new_creates_empty_bucket() {
        let _ = Bucket::new();
    }

    #[test]
    fn it_can_hold_strings() {
        let mut bucket = Bucket::new();
        bucket.set("test", Value::String("value".to_string()));
        assert_eq!(
            *bucket.get("test").unwrap(),
            Value::String("value".to_string())
        );
    }

    #[test]
    fn it_can_hold_bools() {
        let mut bucket = Bucket::new();
        bucket.set("f", Value::Bool(false));
        bucket.set("t", Value::Bool(true));
        assert_eq!(
            *bucket.get("f").unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            *bucket.get("t").unwrap(),
            Value::Bool(true)
        );
    }

    #[test]
    fn it_can_hold_list_of_strings() {
        let mut bucket = Bucket::new();
        bucket.set("test", Value::List(vec!["a".to_string(), "b".to_string()]));
        assert_eq!(
            *bucket.get("test").unwrap(),
            Value::List(vec!["a".to_string(), "b".to_string(),]),
        );
    }

    #[test]
    fn it_can_hold_another_bucket() {
        let mut bucket = Bucket::new();
        let mut value_bucket = Bucket::new();
        value_bucket.set("val1", Value::String("tesst".to_string()));
        let copy_value_bucket = value_bucket.clone();
        
        bucket.set("test", Value::Bucket(value_bucket));
        assert_eq!(
            *bucket.values.get("test").unwrap(),
            Value::Bucket(copy_value_bucket)
        );
    }
}
