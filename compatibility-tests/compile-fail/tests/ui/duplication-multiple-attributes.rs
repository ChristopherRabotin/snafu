use snafu::prelude::*;

#[derive(Debug, Snafu)]
enum EnumError {
    // Duplication on one line
    #[snafu(visibility(pub), visibility(pub))]
    AVariant,
}

fn main() {}
