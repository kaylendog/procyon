use std::fs::File;

use procyon_journal::prelude::JournalReader;

const DATA_FILES: &[&'static str] = &[
    "assets/data/Journal.2024-12-08T234325.01.log",
    "assets/data/Journal.2024-12-09T141341.01.log",
    "assets/data/Journal.2024-12-10T203313.01.log",
    "assets/data/Journal.2024-12-10T231620.01.log",
    "assets/data/Journal.2024-12-10T232345.01.log",
    "assets/data/Journal.2024-12-11T001147.01.log",
    "assets/data/Journal.2024-12-12T151402.01.log",
];

#[test]
fn parse_example_data() {
    for file in DATA_FILES {
        let reader = JournalReader::new(File::open(file).unwrap());
        let errors = reader.enumerate().filter(|(_, r)| r.is_err()).collect::<Vec<_>>();
        if errors.len() > 0 {
            println!("{:#?}", errors);
        }
        assert_eq!(0, errors.len(), "{}", file);
    }
}
