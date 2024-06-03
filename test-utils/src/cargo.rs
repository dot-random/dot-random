use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use toml::{Table, Value};

pub fn get_dependency_dir(repo_name: &str, commit_hash: &str) -> Option<PathBuf> {
    assert_eq!(7, commit_hash.len(), "Commit hash should be 7 chars!");
    let git_dir = add_dir(home::cargo_home().unwrap(), "git/checkouts");
    let option = std::fs::read_dir(git_dir).ok();
    let mut commit_dir: Option<PathBuf> = None;
    for entry in option.unwrap() {
        let path = entry.ok()?.path();
        if path.is_dir() && path.iter().last().unwrap().to_str().unwrap().starts_with(repo_name) {
            let path_buf = add_dir(path.clone(), commit_hash);
            if path_buf.is_dir() {
                commit_dir = Some(path_buf);
            }
        }
    }
    assert!(commit_dir.is_some(), "Can't find a repository '{:?}' or commit '{:?}' in Cargo cache!", repo_name, commit_hash);
    return commit_dir;
}

pub fn get_repo_sub_dir(repo_name: &str, commit_hash: &str, dir: &str) -> PathBuf {
    let repo_dir = get_dependency_dir(repo_name, commit_hash).unwrap();
    return add_dir(repo_dir, dir);
}


pub fn get_rev_for_tag(tag: &str) -> Option<String> {
    let git_dir = add_dir(home::cargo_home().unwrap(), "git/db");
    let option = std::fs::read_dir(git_dir).ok();
    for entry in option.unwrap() {
        let path = entry.ok()?.path();
        if path.is_dir() && path.iter().last().unwrap().to_str().unwrap().starts_with("dot-random-") {
            let mut full_tag = String::new();
            full_tag.push_str("origin/tags/");
            full_tag.push_str(tag);
            let output = Command::new("git")
                .env_clear()
                .current_dir::<&PathBuf>(&path)
                .args([
                    "rev-list",
                    "-1",
                    &*full_tag,
                ])
                .output()
                .unwrap();

            if output.status.success() {
                let stdout_utf8 = String::from_utf8(output.stdout);
                return Some(stdout_utf8.unwrap()[0..7].to_string());
            }
        }
    }
    return None;
}

pub fn get_dependency_rev<P: AsRef<Path>>(cargo_file_dir: P) -> Option<String> {
    let p = Path::new(cargo_file_dir.as_ref());
    let mut result = read_toml(p);
    if result.is_none() {
        result = read_toml(p.parent().unwrap());
    }
    let toml_v = result.unwrap();
    let (tag, rev) = get_tag_and_rev(&toml_v);

    if rev.is_none() && tag.is_some() {
        return get_rev_for_tag(tag.unwrap());
    }

    return rev.and_then(|f| Some(f[0..7].to_string()));
}

fn read_toml<P: AsRef<Path>>(path: P) -> Option<Value> {
    let cargo_toml = path.as_ref().join("Cargo.toml");
    let f = File::open(cargo_toml);
    if f.is_err() {
        println!("err: {}", f.err().unwrap().kind());
        return None;
    } else {
        let mut file = f.ok().unwrap();

        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Couldn't parse Cargo.toml");

        return toml::from_str(&mut contents).unwrap();
    }
}

fn get_tag_and_rev(toml_v: &Value) -> (Option<&str>, Option<&str>) {
    let dep_filter: fn(&Table) -> Option<&Value> = |x: &Table| -> Option<&Value> {
        let git_repo: Value = Value::String("https://github.com/dot-random/dot-random".to_string());
        let dep_package: Value = Value::String("test-utils".to_string());
        for (_key, value) in x {
            if let Value::Table(r) = value {
                let git = r.get("git");
                let package = r.get("package");
                if git == Some(&git_repo) && package == Some(&dep_package) {
                    return Some(value);
                }
            }
        }
        return None;
    };

    // find the line with `dependency` declaration
    let dep_value: &Value = toml_v
        .get("dependencies")
        .and_then(Value::as_table)
        .and_then(dep_filter)
        .or_else(|| toml_v
            .get("dev-dependencies")
            .and_then(Value::as_table)
            .and_then(dep_filter))
        .expect("Can't read dependency from Cargo.toml");

    println!("{:?}", dep_value);
    // find either tag or rev
    let tag = dep_value.get("tag").and_then(Value::as_str);
    let rev = dep_value.get("rev").and_then(Value::as_str);
    println!("tag: {:?} rev: {:?}", tag, rev);

    return (tag, rev);
}


fn add_dir(p: PathBuf, dir: &str) -> PathBuf {
    let mut p = p.into_os_string();
    p.push("/");
    p.push(dir);
    return p.into();
}

