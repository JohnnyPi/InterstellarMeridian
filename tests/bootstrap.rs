use std::process::Command;

#[test]
fn package_metadata_matches_accepted_bootstrap() {
    assert_eq!(env!("CARGO_PKG_NAME"), "interstellar_meridian");
    assert_eq!(env!("CARGO_PKG_LICENSE"), "MIT OR Apache-2.0");
    assert_eq!(env!("CARGO_PKG_RUST_VERSION"), "1.100.0");
}

#[test]
fn native_bootstrap_launches_and_exits_successfully() {
    let output = Command::new(env!("CARGO_BIN_EXE_interstellar_meridian"))
        .output()
        .expect("production executable should launch");
    assert!(
        output.status.success(),
        "bootstrap exit status: {:?}; stderr: {}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        output.stderr.is_empty(),
        "unexpected bootstrap stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).expect("bootstrap stdout should be UTF-8");
    assert_eq!(
        stdout.trim_end_matches(['\r', '\n']),
        interstellar_meridian::BOOTSTRAP_MESSAGE
    );
}
