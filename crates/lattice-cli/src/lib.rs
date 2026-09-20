//! Command-line entry points for stage validation.

/// Runs the stage 0 validator and returns a process exit code.
pub fn run<I>(arguments: I) -> i32
where
    I: IntoIterator<Item = String>,
{
    let mut arguments = arguments.into_iter();
    match arguments.next().as_deref() {
        None | Some("help") | Some("--help") => {
            println!("usage: lattice validate-stage-0");
            0
        }
        Some("validate-stage-0") => match validate_stage_zero() {
            Ok(summary) => {
                println!("stage_0=pass");
                println!("spec_version={}", summary.spec_version);
                println!("fault_cases={}", summary.fault_cases);
                println!("transition_rules={}", summary.transition_rules);
                println!("spec_hash={}", summary.spec_hash);
                0
            }
            Err(error) => {
                eprintln!("stage_0=fail: {error}");
                1
            }
        },
        Some(command) => {
            eprintln!("unknown command: {command}");
            2
        }
    }
}

/// Runs all stage 0 boundary checks before any runtime implementation begins.
pub fn validate_stage_zero() -> Result<lattice_format::StageZeroSummary, String> {
    let summary =
        lattice_format::validate_stage_zero_documents().map_err(|error| error.to_string())?;
    lattice_core::validate_stage_zero().map_err(|error| error.to_string())?;
    lattice_sync::validate_stage_zero()
        .map_err(|error| format!("sync validation failed: {error:?}"))?;
    lattice_transport::validate_stage_zero()
        .map_err(|error| format!("transport validation failed: {error:?}"))?;
    Ok(summary)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_stage_zero() {
        assert_eq!(run([String::from("validate-stage-0")]), 0);
        assert!(validate_stage_zero().is_ok());
    }

    #[test]
    fn help_is_successful() {
        assert_eq!(run([String::from("--help")]), 0);
        assert_eq!(run(std::iter::empty::<String>()), 0);
    }

    #[test]
    fn unknown_command_is_rejected() {
        assert_eq!(run([String::from("unknown")]), 2);
    }
}
