use std::process::Command;

fn main() {
    // 1. CREATING AND SPAWNING A PROCESS
    // Describes the command and starts it as a separate step without waiting for it to finish.
    let child = Command::new("ls")
        .arg("-l") // Adds arguments before spawning
        .spawn() 
        .expect("failed to start ls");
    // Output: "started process with pid <some_number>"
    // Why: .spawn() returns a Child handle that contains the OS-assigned PID, allowing it to run concurrently.
    println!("started process with pid {}", child.id());

    // 2. GETTING OUTPUT ALL AT ONCE
    // .output() runs the command, waits for it to finish, and collects stdout, stderr, and exit status.
    let output = Command::new("ls")
        .arg("-l")
        .output()
        .expect("failed to run ls");
    // Output: The directory listing of the current folder.
    // Why: output.stdout is returned as raw bytes; String::from_utf8_lossy converts it to printable text.
    println!("{}", String::from_utf8_lossy(&output.stdout));

    // 3. WAITING ON A CHILD PROCESS
    let mut child_sleep = Command::new("sleep")
        .arg("2")
        .spawn()
        .expect("failed to start sleep");
    // .wait() blocks the current Rust program until the spawned child process finishes.
    let status = child_sleep.wait()
        .expect("failed to wait on child");
    // Output: "child finished successfully"
    // Why: `sleep 2` naturally completes successfully, returning an ExitStatus where .success() is true.
    if status.success() {
        println!("child finished successfully");
    } else {
        println!("exit code: {:?}", status.code());
    }

    // 4. ENVIRONMENT VARIABLES
    let env_child = Command::new("printenv")
        .arg("MY_VAR")
        // .env() sets an environment variable exclusively for this child process[cite: 1].
        .env("MY_VAR", "hello from parent") 
        .spawn()
        .expect("failed to start printenv");
    // Output: "hello from parent"
    // Why: The child process inherits this specific environment variable and prints it when executed[cite: 1].

    // 5. KILLING A PROCESS[cite: 1]
    // (Assuming `mut target_child` is a spawned process)
    // target_child.kill().expect("failed to kill child");
    // Output: No direct printed output, but the target process terminates immediately.
    // Why: Child::kill() unconditionally sends a SIGKILL signal to the OS, terminating the child without cleanup[cite: 1].
}