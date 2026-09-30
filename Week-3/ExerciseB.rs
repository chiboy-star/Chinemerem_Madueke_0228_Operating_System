use std::fs::File;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};
use std::thread::sleep;

fn run_with_timeout_and_log(command: &str, timeout_secs: u64) {
    // Create the log files
    let out = File::create("stdout.log").expect("couldn't create stdout log file");
    let err = File::create("stderr.log").expect("couldn't create stderr log file");

    // Spawn the command with custom stdout and stderr routing
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::from(out))
        .stderr(Stdio::from(err))
        .spawn()
        .expect("failed to execute command");

    let start_time = Instant::now();
    let timeout = Duration::from_secs(timeout_secs);

    loop {
        let status = child.try_wait().expect("failed to wait on child");

        if status.is_some() {
            println!("Process exited naturally with code: {:?}", status.unwrap().code());
            println!("Elapsed time: {:?}", start_time.elapsed());
            break;
        }

        if start_time.elapsed() >= timeout {
            println!("Timeout reached. Killing process...");
            child.kill().expect("failed to kill process");
            child.wait().expect("failed to clean up killed process");
            println!("Process was killed after {:?}", start_time.elapsed());
            break;
        }

        sleep(Duration::from_millis(200));
    }
}

fn main() {
    // Example usage:
    // run_with_timeout_and_log("echo 'This goes to a file' && sleep 10", 3);
}