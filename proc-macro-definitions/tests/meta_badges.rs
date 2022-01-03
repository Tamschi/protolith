#![cfg(not(miri))]

#[path = "meta_constants_.rs"]
mod constants;
use constants::*;

#[test]
fn weak_assert_branch() {
	let info = git_info::get();

	if let Some(branch) = info.current_branch {
		if branch.contains("HEAD detached") {
			eprintln!("Branch assert ignored: HEAD detached")
		} else if branch == "(no branch)" {
			// Most likely a release tag.
			eprintln!(r#"Branch assert ignored: "(no branch)""#)
		} else if branch == "template" {
			eprintln!("Branch assert ignored: Currently on template branch")
		} else if branch.contains('-') || branch.contains('/') {
			eprintln!("Branch assert ignored: Probably a feature branch")
		} else {
			assert_eq!(BRANCH, branch);
		}
	} else {
		eprintln!("Branch assert ignored: No branch information available")
	}
}
