//! CLI argument parsing.
//!
//! No external dependency: the CLI has a handful of flags, and a parser in
//! 100 lines is clearer than a dependency. `clap` is deliberately avoided
//! for now; if the CLI grows, it will be adopted with an ADR.

use std::error::Error;
use std::fmt;

/// The parsed command-line options.
///
/// The flags are independent: `--check --list` is valid, and the first
/// command wins in `main::run`. The `struct_excessive_bools` lint assumes
/// mutually exclusive flags, which is not the case here.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Args {
    /// The seed for the scenarios. Default: 42.
    pub seed: u64,
    /// If true, run the health check.
    pub check: bool,
    /// If true, list the registered scenarios and exit.
    pub list: bool,
    /// If true, print feature coverage and exit.
    pub coverage: bool,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            seed: 42,
            check: false,
            list: false,
            coverage: false,
        }
    }
}

/// Errors that can occur while parsing arguments.
#[derive(Debug)]
pub enum ArgError {
    /// Unknown flag.
    Unknown(String),
    /// Missing value for a flag that requires one.
    MissingValue(String),
    /// Invalid value for a flag.
    InvalidValue {
        /// The flag name.
        flag: String,
        /// The invalid value.
        value: String,
    },
    /// `--help` was requested.
    HelpRequested,
}

impl fmt::Display for ArgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown(s) => write!(f, "unknown argument: {s}"),
            Self::MissingValue(s) => write!(f, "missing value for {s}"),
            Self::InvalidValue { flag, value } => {
                write!(f, "invalid value for {flag}: {value}")
            }
            Self::HelpRequested => write!(f, "help requested"),
        }
    }
}

impl Error for ArgError {}

/// Parses command-line arguments.
///
/// # Errors
///
/// Returns [`ArgError`] if a flag is unknown, a value is missing, or a value
/// is invalid.
pub fn parse<I, S>(iter: I) -> Result<Args, ArgError>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let mut args = Args::default();
    let mut iter = iter.into_iter().map(Into::into);

    // The first item is the binary name. Skip it.
    let _ = iter.next();

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--help" | "-h" => return Err(ArgError::HelpRequested),
            "--check" => args.check = true,
            "--list" => args.list = true,
            "--coverage" => args.coverage = true,
            "--seed" => {
                let v = iter
                    .next()
                    .ok_or_else(|| ArgError::MissingValue(arg.clone()))?;
                args.seed = v.parse().map_err(|_| ArgError::InvalidValue {
                    flag: arg,
                    value: v,
                })?;
            }
            other => return Err(ArgError::Unknown(other.to_string())),
        }
    }
    Ok(args)
}

/// The help text.
pub const HELP: &str = "\
ds-test-runner — run PCFM test scenarios

USAGE:
    ds-test-runner [OPTIONS]

OPTIONS:
    --seed <U64>       Seed for the scenarios (default: 42)
    --check            Run the health check (default when no command is given)
    --list             List the registered test scenarios and exit
    --coverage         Print feature coverage of the test scenarios
    --help, -h         Print this help
";

#[cfg(test)]
mod tests {
    use super::*;

    fn argv(items: &[&str]) -> Vec<String> {
        std::iter::once("ds-test-runner")
            .chain(items.iter().copied())
            .map(String::from)
            .collect()
    }

    #[test]
    fn empty_args_gives_default() {
        let a = parse(argv(&[])).unwrap();
        assert_eq!(a, Args::default());
    }

    #[test]
    fn seed_is_parsed() {
        let a = parse(argv(&["--seed", "123"])).unwrap();
        assert_eq!(a.seed, 123);
    }

    #[test]
    fn check_flag_is_set() {
        let a = parse(argv(&["--check"])).unwrap();
        assert!(a.check);
    }

    #[test]
    fn list_flag_is_set() {
        let a = parse(argv(&["--list"])).unwrap();
        assert!(a.list);
    }

    #[test]
    fn coverage_flag_is_set() {
        let a = parse(argv(&["--coverage"])).unwrap();
        assert!(a.coverage);
    }

    #[test]
    fn unknown_flag_is_rejected() {
        assert!(matches!(parse(argv(&["--foo"])), Err(ArgError::Unknown(_))));
    }

    #[test]
    fn missing_value_is_rejected() {
        assert!(matches!(
            parse(argv(&["--seed"])),
            Err(ArgError::MissingValue(_))
        ));
    }

    #[test]
    fn invalid_seed_is_rejected() {
        assert!(matches!(
            parse(argv(&["--seed", "abc"])),
            Err(ArgError::InvalidValue { .. })
        ));
    }

    #[test]
    fn help_is_requested() {
        assert!(matches!(
            parse(argv(&["--help"])),
            Err(ArgError::HelpRequested)
        ));
    }
}
