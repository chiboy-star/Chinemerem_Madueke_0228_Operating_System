use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut child = Command::new("sleep")
        .arg("30")
        .spawn()
        .expect("failed to start sleep");
        
    sleep(Duration::from_secs(3));
    
    let status = child.try_wait()
        .expect("failed to check child status");
        
    if status.is_none() {
        println!("still running — killing it");
        child.kill().expect("failed to kill child");
    } else {
        println!("child already exited on its own");
    }
}