use anyhow::Error;
use vergen::{vergen, Config};
use vergen::{SemverKind, ShaKind};

fn main() -> Result<(), Error> {
    let mut config = Config::default();

    *config.git_mut().sha_kind_mut() = ShaKind::Short;
    *config.git_mut().semver_kind_mut() = SemverKind::Lightweight;
    *config.git_mut().semver_dirty_mut() = Some("-dirty");

    vergen(config)
}
