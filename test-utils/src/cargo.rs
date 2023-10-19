use std::path::PathBuf;

pub fn get_dependency_dir(repo_name: &str, commit_hash: &str) -> Option<PathBuf> {
    assert_eq!(7, commit_hash.len(), "Commit hash should be 7 chars!");
    let git_dir = add_dir(home::cargo_home().unwrap(), "git/checkouts");
    let option = std::fs::read_dir(git_dir).ok();
    let mut commit_dir: Option<PathBuf> = None;
    for entry in option.unwrap() {
        let path = entry.ok()?.path();
        if path.is_dir() && path.iter().last().unwrap().to_str().unwrap().starts_with(repo_name) {
            commit_dir = Some(add_dir(path.clone(), commit_hash));
        }
    }
    assert!(commit_dir.is_some(), "Can't find a repository '{:?}' or commit '{:?}' in Cargo cache!", repo_name, commit_hash);
    return commit_dir;
}

pub fn get_repo_sub_dir(repo_name: &str, commit_hash: &str, dir: &str) -> PathBuf {
    let repo_dir = get_dependency_dir(repo_name, commit_hash).unwrap();
    return add_dir(repo_dir, dir);
}

fn add_dir(p: PathBuf, dir: &str) -> PathBuf {
    let mut p = p.into_os_string();
    p.push("/");
    p.push(dir);
    return p.into();
}

