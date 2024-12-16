use std::fs::File;

use procyon_journal::JournalReader;

#[test]
fn example_data() {
    // assets/data/Journal.2024-12-08T234325.01.log
    let reader =
        JournalReader::new(File::open("assets/data/Journal.2024-12-08T234325.01.log").unwrap());

    let events = reader.collect::<Vec<_>>();
    println!("{:#?}", events.iter().enumerate().filter(|(_, r)| r.is_err()).collect::<Vec<_>>());
    assert_eq!(
        0,
        events.into_iter().filter(|r| r.is_err()).count(),
        "assets/data/Journal.2024-12-08T234325.01.log"
    );

    // assets/data/Journal.2024-12-09T141341.01.log
    let reader =
        JournalReader::new(File::open("assets/data/Journal.2024-12-09T141341.01.log").unwrap());

    let events = reader.collect::<Vec<_>>();
    println!("{:#?}", events.iter().enumerate().filter(|(_, r)| r.is_err()).collect::<Vec<_>>());
    assert_eq!(
        0,
        events.into_iter().filter(|r| r.is_err()).count(),
        "assets/data/Journal.2024-12-09T141341.01.log"
    );
}
