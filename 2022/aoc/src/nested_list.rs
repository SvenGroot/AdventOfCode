use std::str;
use std::str::FromStr;

#[derive(Debug)]
pub enum Item<T> {
    Value(T),
    List(Vec<Item<T>>),
}

impl<T> Item<T> {
    pub fn value(&self) -> Option<&T> {
        if let Item::Value(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn list(&self) -> Option<&Vec<Item<T>>> {
        if let Item::List(list) = self {
            Some(list)
        } else {
            None
        }
    }
}

impl<T: FromStr> Item<T> {
    fn from_str(mut value: &[u8]) -> Option<(Vec<Item<T>>, &[u8])> {
        let mut result = Vec::new();
        while !value.is_empty() && value[0] != b']' {
            if !value.is_empty() && value[0] == b',' {
                value = &value[1..];
            }

            if value[0] == b'[' {
                let (item, new_value) = Self::from_str(&value[1..])?;
                value = new_value;
                result.push(Item::List(item));
            } else {
                let end = value
                    .iter()
                    .position(|v| *v == b',' || *v == b']')
                    .unwrap_or(value.len());

                let (first, rest) = value.split_at(end);

                result.push(Item::Value(str::from_utf8(first).unwrap().parse().ok()?));
                value = rest;
            }
        }

        if !value.is_empty() && value[0] == b']' {
            value = &value[1..];
        }

        Some((result, value))
    }
}

pub struct NestedList<T>(Vec<Item<T>>);

impl<T> NestedList<T> {
    pub fn items(&self) -> &Vec<Item<T>> {
        &self.0
    }
}

impl<T: FromStr> FromStr for NestedList<T> {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.as_bytes();
        if s[0] != b'0' && *s.last().unwrap() != b']' {
            return Err("Invalid input.");
        }

        let s = &s[1..s.len() - 1];
        let (list, remain) = Item::from_str(s).ok_or("Invalid input.")?;
        if !remain.is_empty() {
            return Err("Invalid input.");
        }

        Ok(Self(list))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let result = NestedList::<u32>::from_str("[[1],[2,3,4]]").unwrap();
        assert_eq!(1, *result.0[0].list().unwrap()[0].value().unwrap());
        let list = result.0[1].list().unwrap();
        assert_eq!(2, *list[0].value().unwrap());
        assert_eq!(3, *list[1].value().unwrap());
        assert_eq!(4, *list[2].value().unwrap());
    }
}
