use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
enum InnerError {
    #[snafu(display("inner error"))]
    AnExample,
}

#[derive(Debug, Snafu)]
enum Error {
    NoDisplay { source: InnerError },
}

#[test]
fn default_error_display() {
    let err: Error = AnExampleSnafu
        .fail::<()>()
        .context(NoDisplaySnafu)
        .unwrap_err();
    assert_eq!(format!("{}", err), "NoDisplay: inner error",);
}
