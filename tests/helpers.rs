#[cfg(test)]
use crate::support::TestCandidate;
use sdkman_cli_native::helpers::{closest_candidate, known_candidates};
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
        config: None,
    };

    let sdkman_dir = support::virtual_env(env);
    let candidates =
        known_candidates(sdkman_dir.keep()).expect("the candidates file should be readable");
    let expected_candidates = vec!["scala".to_string()];

    assert_eq!(candidates, expected_candidates);

    Ok(())
}

#[test]
#[serial]
fn should_fail_if_candidate_file_is_missing() {
    let sdkman_dir = prepare_sdkman_dir();
    let error = known_candidates(sdkman_dir.keep()).expect_err("a missing file must be an error");
    assert!(
        error.message.contains("cannot read"),
        "message was: {}",
        error.message
    );
    assert!(error.message.contains("var/candidates"));
}

#[test]
fn should_suggest_java_for_jav() {
    let all_candidates = vec![
        "java".to_string(),
        "kotlin".to_string(),
        "scala".to_string(),
    ];
    assert_eq!(
        closest_candidate(&all_candidates, "jav"),
        Some("java".to_string())
    );
}

#[test]
fn should_point_to_sdk_list_for_xyz() {
    let all_candidates = vec![
        "java".to_string(),
        "kotlin".to_string(),
        "scala".to_string(),
    ];
    assert_eq!(closest_candidate(&all_candidates, "xyz"), None);
}
