#![cfg(not(miri))]

#[path = "meta_constants_.rs"]
mod constants;
use constants::*;

#[test]
fn html_root_url() {
	version_sync::assert_contains_regex!(
		"src/lib.rs",
		r#"^#!\[doc\(html_root_url = "https://docs\.rs/TODO_CRATE_NAME_proc-macro-definitions/{version}"\)\]$"#
	);
}

#[test]
fn homepage() {
	version_sync::assert_contains_regex!(
		"Cargo.toml",
		&format!(
			r#"^homepage = "https://github\.com/{0}/TODO_CRATE_NAME/tree/v{{version}}"$"#,
			USER,
		)
	);
}

#[test]
fn documentation() {
	version_sync::assert_contains_regex!(
		"Cargo.toml",
		r#"^documentation = "https://docs\.rs/TODO_CRATE_NAME/{version}"$"#
	);
}
