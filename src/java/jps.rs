// Abstraction over jps
use std::{fmt::Display, process::Command};

//output line from jps
pub struct JpsLine {
    pid: u32,
    name: String,
}

impl JpsLine {
    fn from_vec(vec: Vec<&str>) -> JpsLine {
        return JpsLine {
            pid: vec[0].parse().unwrap(),
            name: vec[1].to_string(),
        };
    }
}

impl Display for JpsLine {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        write!(f, "{} {}", &self.pid, &self.name)
    }
}

pub fn list_java_processes() -> Vec<JpsLine> {
    //run jps
    let process = Command::new("jps")
        .output()
        .expect("jps command failed to start");
    let output =
        String::from_utf8_lossy(&process.stdout);
    let parts: Vec<&str> =
        output.split("\n").collect();
    //if there is no any java processes then jps will show only itself
    if parts.len() == 1 {
        return Vec::new();
    } else {
        let mut java_processes: Vec<JpsLine> =
            Vec::with_capacity(parts.len());
        for (pos, element) in
            parts.iter().enumerate()
        {
            //otherwise
            if pos != 0 && pos != parts.len() - 1
            {
                let parts: Vec<&str> =
                    element.split(" ").collect();
                java_processes.push(
                    JpsLine::from_vec(parts),
                );
            }
        }
        return java_processes;
    }
}
