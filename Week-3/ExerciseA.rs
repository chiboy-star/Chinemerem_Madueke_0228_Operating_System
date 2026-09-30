use std::process::Command;
use std::time::{Duration, Instant};
use std::thread::sleep;

fn run_with_timeout(command: &str, timeout_secs: u64) {
    // 1. Spawn the command. We use `sh -c` to easily handle shell commands.
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(command)
        .spawn()
        .expect("failed to execute command");

    let start_time = Instant::now();
    let timeout = Duration::from_secs(timeout_secs);

    // 2. Loop and poll with try_wait()
    loop {
        let status = child.try_wait().expect("failed to wait on child");

        // 5. On natural exit: report exit code and elapsed time
        if status.is_some() {
            println!("Process exited naturally with code: {:?}", status.unwrap().code());
            println!("Elapsed time: {:?}", start_time.elapsed());
            break;
        }

        // 3. Compare elapsed vs timeout
        if start_time.elapsed() >= timeout {
            // 4. On timeout: kill() + wait() to prevent zombie processes
            println!("Timeout reached. Killing process...");
            child.kill().expect("failed to kill process");
            child.wait().expect("failed to clean up killed process");
            println!("Process was killed after {:?}", start_time.elapsed());
            break;
        }

        // Sleep briefly between checks
        sleep(Duration::from_millis(200));
    }
}

fn main() {
    // Example usage: 
    // run_with_timeout("sleep 2", 5); // Will exit naturally
    // run_with_timeout("sleep 10", 3); // Will be killed
}