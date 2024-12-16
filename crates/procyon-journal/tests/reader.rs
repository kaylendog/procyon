use std::fs::File;

use procyon_journal::JournalReader;

#[test]
fn read_example_data() {
    let reader =
        JournalReader::new(File::open("assets/data/Journal.2024-12-08T234325.01.log").unwrap());

    let entries: Vec<_> = reader.collect();

    println!("{:?}", entries);
}
