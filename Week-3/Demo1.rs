use std::process::Command;

fn main() {
    let output = Command::new("echo")
        .arg("hello from a child process")
        .output()
        .expect("failed to run echo");
        
    let text = String::from_utf8_lossy(&output.stdout);
    println!("child said: {}", text.trim());
    println!("exit code: {:?}", output.status.code());
}