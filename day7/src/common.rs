use std::collections::HashMap;
use std::path::PathBuf;

pub fn calculate_sizes(input: &str) -> HashMap<PathBuf, usize> {
    let mut current_dir = PathBuf::from("/");
    let mut dirs = HashMap::new();
    for line in input.lines() {
        if line.starts_with('$') {
            match &line[2..4] {
                "cd" => match &line[5..] {
                    "/" => current_dir = PathBuf::from("/"),
                    ".." => {
                        current_dir.pop();
                    }
                    p => current_dir.push(p),
                },
                "ls" => (),
                c => panic!("unknown command {}", c),
            }
        } else if !line.starts_with("dir ") {
            let size: usize = line
                .split(' ')
                .next()
                .unwrap()
                .parse()
                .expect("should be a file size");

            for path in current_dir.ancestors() {
                *dirs.entry(path.into()).or_insert(0) += size;
            }
        }
    }

    dirs
}
