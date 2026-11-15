#![forbid(unsafe_code)]

// The files under generated/ are owned by flags-2-env, so validate the
// generated precedence contract instead of manually modifying generated code.
#[path = "../generated/rust/runtime.rs"]
mod generated_runtime;

use std::collections::BTreeMap;

fn map(entries: &[(&str, &str)]) -> BTreeMap<String, String> {
    entries
        .iter()
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect()
}

#[test]
fn flags_override_shell_and_dotenv_without_reading_os_secrets() {
    let shell = map(&[("APME_SIDECAR_BIND", "127.0.0.1:9201")]);
    let dotenv = map(&[("APME_SIDECAR_BIND", "127.0.0.1:9202")]);
    let flags = map(&[("APME_SIDECAR_BIND", "127.0.0.1:9203")]);

    let resolved = generated_runtime::load_env_map(&shell, &dotenv, &flags)
        .expect("all inputs are synthetic and valid");
    assert_eq!(resolved.get("APME_SIDECAR_BIND").map(String::as_str), Some("127.0.0.1:9203"));
}

#[test]
fn empty_higher_priority_values_fall_through_without_leaking_state() {
    let shell = map(&[("APME_SIDECAR_BIND", "127.0.0.1:9201")]);
    let dotenv = map(&[("APME_SIDECAR_BIND", "127.0.0.1:9202")]);
    let flags = map(&[("APME_SIDECAR_BIND", "  ")]);

    let resolved = generated_runtime::load_env_map(&shell, &dotenv, &flags)
        .expect("resolution succeeds");
    assert_eq!(resolved.get("APME_SIDECAR_BIND").map(String::as_str), Some("127.0.0.1:9201"));

    let default = generated_runtime::load_env_map(&map(&[]), &map(&[]), &map(&[]))
        .expect("default exists");
    assert_eq!(default.get("APME_SIDECAR_BIND").map(String::as_str), Some("127.0.0.1:9090"));
}

#[test]
fn pure_override_lookup_is_deterministic() {
    let values = generated_runtime::load_from(|name| {
        (name == "APME_SIDECAR_BIND").then(|| "127.0.0.1:9400".to_string())
    });
    assert_eq!(values.bind, "127.0.0.1:9400");
}
