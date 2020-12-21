use std::process::Command;

fn main() {
    let hash = get_commit_hash().unwrap_or_else(|| "???????".to_string());
    println!("cargo:rustc-env=GIT_HASH={}", hash);
}

fn get_commit_hash() -> Option<String> {
    let args = &["rev-parse", "HEAD"];
    let output = Command::new("git").args(args).output().ok()?;
    let stdout = String::from_utf8(output.stdout).ok()?;
    let trimmed_hash = stdout.get(0..7)?;
    Some(trimmed_hash.to_string())
}
