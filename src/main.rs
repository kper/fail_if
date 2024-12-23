use std::io::BufRead;
use std::io::BufReader;
use std::process::Command;
use std::process::Stdio;
use std::thread;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let mut child = Command::new(args.get(2).expect("Argument must exist"))
        .args(&args[3..])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let child_stdout = child
        .stdout
        .take()
        .expect("Internal error, could not take stdout");
    let child_stderr = child
        .stderr
        .take()
        .expect("Internal error, could not take stderr");

    let (stdout_tx, stdout_rx) = std::sync::mpsc::channel();
    let (stderr_tx, stderr_rx) = std::sync::mpsc::channel();

    let stdout_thread = thread::spawn(move || {
        let stdout_lines = BufReader::new(child_stdout).lines();
        for line in stdout_lines {
            let line = line.unwrap();
            println!("{}", line);
            stdout_tx.send(line).unwrap();
        }
    });

    let stderr_thread = thread::spawn(move || {
        let stderr_lines = BufReader::new(child_stderr).lines();
        for line in stderr_lines {
            let line = line.unwrap();
            eprintln!("{}", line);
            stderr_tx.send(line).unwrap();
        }
    });

    let status = child
        .wait()
        .expect("Internal error, failed to wait on child");

    stdout_thread.join().unwrap();
    stderr_thread.join().unwrap();

    let stdout = stdout_rx.into_iter().collect::<Vec<String>>().join("");
    let stderr = stderr_rx.into_iter().collect::<Vec<String>>().join("");

    if (stdout.contains(&args[1]) || stderr.contains(&args[1])) {
        std::process::exit(1);
    }
}
