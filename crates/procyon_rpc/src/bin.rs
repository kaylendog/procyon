use std::{env, path::PathBuf};

use procyon_rpc::router;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = PathBuf::from(args.get(1).expect("expected output path"));
    let path = env::current_dir().unwrap().join(path);
    router().export_ts(path).expect("failed to write bindings");
}
