use std::process::Command;

//Abstraction over jps
struct Jps {}

impl Jps {
    //show all java processes except for jps
    fn show_java_processes(&self) {
        let process = Command::new("jps")
            .output()
            .expect("jps command failed to start");
        let output = String::from_utf8_lossy(&process.stdout);
        let parts: Vec<&str> = output.split("\n").collect();
        //if only jps process itself
        if parts.len() == 1 {
            println!("No java processes");
        } else {
            for (pos, element) in parts.iter().enumerate() {
                if pos != 0 {
                    let parts: Vec<&str> = element.split(" ").collect();
                    println!("{}", parts[0]);
                }
            }
        }
    }

    fn new() -> Jps {
        return Jps {};
    }
}

fn main() {
    let jps = Jps::new();
    jps.show_java_processes();
}