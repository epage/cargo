use cargo_test_support::compare::assert_ui;
use cargo_test_support::paths;
use cargo_test_support::prelude::*;
use cargo_test_support::ProjectBuilder;

use cargo_test_support::curr_dir;

#[cargo_test]
fn case() {
    let project = ProjectBuilder::new(paths::root().join("in"))
        .file(
            ".cargo/config.toml",
            r#"[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "./vendor"
"#,
        )
        .file("vendor/aa/src/lib.rs", "")
        .file("vendor/aa/.cargo-checksum.json", "{\"files\":{}}")
        .file(
            "vendor/aa/Cargo.toml",
            r#"[workspace]

[package]
name = "aa"
version = "0.0.0"
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
        .arg_line("cbindgen")
        .current_dir(cwd)
        .assert()
        .failure()
        .stdout_matches_path(curr_dir!().join("stdout.log"))
        .stderr_matches_path(curr_dir!().join("stderr.log"));

    assert_ui().subset_matches(curr_dir!().join("out"), &project_root);
}
