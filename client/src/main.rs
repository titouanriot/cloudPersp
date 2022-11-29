use std::process::Command;

fn main() {

    let output = Command::new("echo")
        .arg("hello world")
        .status()
        .expect("failed to execute");

    let list_dir = Command::new("ls")
        // command arguments
        .args(["-l", "-a", "-R", "-t"])
        // directory we want to display the files
        .current_dir("src/")
        .status()
        .expect("ls command failed to start");

    let change_permissions = Command::new("chmod")
        .args(["-R", "777", "test/"])
        .status()
        .expect("chmod command failed to start");
        
}
