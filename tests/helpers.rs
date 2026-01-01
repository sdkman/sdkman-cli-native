#[cfg(test)]
use crate::support::TestCandidate;
use sdkman_cli_native::helpers::known_candidates;
use serial_test::serial;
use support::{prepare_sdkman_dir, VirtualEnv};

mod support;

#[test]
#[serial]
fn should_fail_if_candidate_is_unknown() -> Result<(), Box<dyn std::error::Error>> {
    let env = VirtualEnv {
        cli_version: "0.0.1".to_string(),
        native_version: "0.0.1".to_string(),
        candidates: vec![TestCandidate {
            name: "scala",
            versions: vec!["0.0.1"],
            current_version: "0.0.1",
        }],
    };

    let sdkman_dir = support::virtual_env(env);
    let candidates = known_candidates(sdkman_dir.keep());
    let expected_candidate = vec!["scala"];

    assert_eq!(candidates, expected_candidate);

    Ok(())
}

#[test]
#[serial]
#[should_panic]
fn should_fail_if_candidate_file_is_missing() {
    let sdkman_dir = prepare_sdkman_dir();
    known_candidates(sdkman_dir.keep());
}
