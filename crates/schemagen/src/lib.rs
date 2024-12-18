//! A library for generating Rust types from JSON data.

use std::collections::{HashMap, HashSet};

/// A shape is a description of a type that can be used to generate a schema.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Shape {
    Unknown,
    Null,
    Number,
    Bool,
    String,
    Optional(Box<Shape>),
    Union(Vec<Shape>),
    Array(Box<Shape>),
    Object(HashMap<String, Shape>),
}

impl Shape {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    /// Unify two shapes, returning the most general shape that can represent both.
    pub fn unify(&self, other: &Self) -> Shape {
        match (self, other) {
            // unknown unifies with anything
            (Shape::Unknown, x) | (x, Shape::Unknown) => x.clone(),
            // ground types unify if they are the same
            (Shape::Null, Shape::Null) => Shape::Null,
            (Shape::Number, Shape::Number) => Shape::Number,
            (Shape::String, Shape::String) => Shape::String,
            (Shape::Bool, Shape::Bool) => Shape::Bool,
            // constructors unify if their arguments unify
            (Shape::Array(a), Shape::Array(b)) => Shape::Array(Box::new(a.unify(b))),
            (Shape::Object(lhs), Shape::Object(rhs)) => {
                let mut map = HashMap::new();
                for key in lhs.keys().chain(rhs.keys()).collect::<HashSet<_>>().into_iter() {
                    let shape = match (lhs.get(key), rhs.get(key)) {
                        // common keys unify
                        (Some(lhs), Some(rhs)) => lhs.unify(rhs),
                        // non-common keys are optional
                        (Some(x), None) | (None, Some(x)) => Shape::Optional(x.clone().boxed()),
                        (None, None) => unreachable!(),
                    };
                    map.insert(key.clone(), shape);
                }
                Shape::Object(map)
            }
            // differing types unify under union
            (a, b) if a != b => Shape::Union(vec![a.clone(), b.clone()]),
            // unions unify by merging their elements
            (Shape::Union(lhs), Shape::Union(rhs)) => {
                let mut union = lhs.clone();
                for shape in rhs {
                    if !union.contains(&shape) {
                        union.push(shape.clone());
                    }
                }
                Shape::Union(union)
            }
            // a union unifies with a type by adding the type to the union
            (Shape::Union(union), x) | (x, Shape::Union(union)) => {
                let mut union = union.clone();
                if !union.contains(&x) {
                    union.push(x.clone());
                }
                Shape::Union(union)
            }
            // optional unifies with a type by making the type optional
            (Shape::Optional(a), Shape::Optional(b)) => Shape::Optional(Box::new(a.unify(b))),
            // otherwise, just union the two shapes
            (a, b) => Shape::Union(vec![a.clone(), b.clone()]),
        }
    }
}

impl From<&Vec<serde_json::Value>> for Shape {
    fn from(value: &Vec<serde_json::Value>) -> Self {
        value.iter().fold(Shape::Unknown, |acc, value| acc.unify(&value.into()))
    }
}

impl From<Vec<serde_json::Value>> for Shape {
    fn from(value: Vec<serde_json::Value>) -> Self {
        (&value).into()
    }
}

impl From<&serde_json::Value> for Shape {
    fn from(value: &serde_json::Value) -> Self {
        match value {
            serde_json::Value::Null => Shape::Null,
            serde_json::Value::Bool(_) => Shape::Bool,
            serde_json::Value::Number(_) => Shape::Number,
            serde_json::Value::String(_) => Shape::String,
            serde_json::Value::Array(vec) => Shape::Array(Box::new(vec.into())),
            serde_json::Value::Object(map) => {
                Shape::Object(map.into_iter().map(|(k, v)| (k.clone(), v.into())).collect())
            }
        }
    }
}

impl From<serde_json::Value> for Shape {
    fn from(value: serde_json::Value) -> Self {
        (&value).into()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::Shape;

    #[test]
    fn unify_constants() {
        assert_eq!(Shape::Number.unify(&Shape::Number), Shape::Number);
        assert_eq!(Shape::String.unify(&Shape::String), Shape::String);
    }

    #[test]
    fn unify_arrays() {
        assert_eq!(
            Shape::Array(Box::new(Shape::Number)).unify(&Shape::Array(Box::new(Shape::Number))),
            Shape::Array(Box::new(Shape::Number))
        );

        // unification of number[] and string[] results in (number | string)[]
        assert_eq!(
            Shape::Array(Box::new(Shape::Number)).unify(&Shape::Array(Box::new(Shape::String))),
            Shape::Array(Box::new(Shape::Union(vec![Shape::Number, Shape::String])))
        );
    }

    #[test]
    fn unify_maps() {
        let mut map = HashMap::new();
        map.insert("a".to_string(), Shape::Number);
        map.insert("b".to_string(), Shape::String);

        let lhs = Shape::Object(map.clone());
        map.insert("c".to_string(), Shape::Bool);

        let rhs = Shape::Object(map);

        let expected = Shape::Object(
            vec![
                ("a".to_string(), Shape::Number),
                ("b".to_string(), Shape::String),
                ("c".to_string(), Shape::Optional(Shape::Bool.boxed())),
            ]
            .into_iter()
            .collect(),
        );

        assert_eq!(expected, lhs.unify(&rhs));
    }

    #[test]
    fn unify_values() {
        let lhs: Shape = serde_json::json!({
            "a": 1,
            "b": "hello",
        })
        .into();

        let rhs: Shape = serde_json::json!({
            "a": 2,
            "c": true,
        })
        .into();

        let expected = Shape::Object(
            vec![
                ("a".to_string(), Shape::Number),
                ("b".to_string(), Shape::Optional(Shape::String.boxed())),
                ("c".to_string(), Shape::Optional(Shape::Bool.boxed())),
            ]
            .into_iter()
            .collect(),
        );

        assert_eq!(expected, lhs.unify(&rhs));
    }
}
