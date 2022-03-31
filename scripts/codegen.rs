use std::process::Command;

fn main() {
    let mut child = Command::new("./scripts/codegen.sh")
        .arg("target/out.rs")
        .spawn()
        .unwrap();
    child.wait().unwrap();
}
