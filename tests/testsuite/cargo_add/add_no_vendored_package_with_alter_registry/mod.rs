use cargo_test_support::compare::assert_ui;
use cargo_test_support::paths;
use cargo_test_support::prelude::*;
use cargo_test_support::ProjectBuilder;

use cargo_test_support::curr_dir;

#[cargo_test]
fn case() {
    cargo_test_support::registry::alt_init();
    cargo_test_support::registry::Package::new("linked-hash-map", "0.5.4")
        .feature("clippy", &[])
        .feature("heapsize", &[])
        .feature("heapsize_impl", &[])
        .feature("nightly", &[])
        .feature("serde", &[])
        .feature("serde_impl", &[])
        .feature("serde_test", &[])
        .alternative(true)
        .publish();

    let project = ProjectBuilder::new(paths::root().join("in"))
        .file(
            ".cargo/config.toml",
            r#"[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "./vendor"
"#,
        )
        .file("src/lib.rs", "")
        .file(
            "Cargo.toml",
            r#"[workspace]

[package]
name = "cargo-list-test-fixture"
version = "0.0.0"
"#,
        )
        .build();
    let project_root = project.root();
    let cwd = &project_root;

    snapbox::cmd::Command::cargo_ui()
        .arg("add")
        .arg_line("linked_hash_map --registry alternative")
        .current_dir(cwd)
        .assert()
        .success()
        .stdout_matches_path(curr_dir!().join("stdout.log"))
        .stderr_matches_path(curr_dir!().join("stderr.log"));

    assert_ui().subset_matches(curr_dir!().join("out"), &project_root);
}
