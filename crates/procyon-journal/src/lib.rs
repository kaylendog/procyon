//! A library for parsing and reading Elite Dangerous journal files.

#[cfg(test)]
mod tests {
    use serde_json::Value;

    #[test]
    fn sample_shapes() {
        let value: Vec<Value> = include_str!("../samples/Journal.2024-12-08T234325.01.log")
            .lines()
            .filter_map(|line| serde_json::from_str(line).ok())
            .collect();

        let shape: schemagen::Shape = value.into();

        println!("{:#?}", shape);
    }
}
