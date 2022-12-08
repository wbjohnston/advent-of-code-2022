use std::collections::BTreeMap as Map;
mod input;

fn main() {
    let mut root = Shell { fs: Dir {
        children: Map::new(),
    } };

    // insert

    let commands = input::INPUT.split("$").skip(1);

    let mut current_path: Vec<&str> = vec![];

    let mut get_current_path = || -> &mut Dir {
        let mut dir = &mut root.fs;
        for dirName in current_path.iter() {
            let s = dirName.to_string();
            dir = match dir.children.get_mut(&s).unwrap() {
                Entry::Dir(d) => d,
                _ => unreachable!(),
            }
        }

        dir
    };

    let mut current_dir = get_current_path();

    for command in commands {
        print!("{}", command);
        let lines: Vec<_> = command.trim().split("\n").collect();
        let command_line = dbg!(lines[0]);
        let split = command_line.split(" ").collect::<Vec<_>>();

        match split[0] {
            "cd" => {
                let directory_name = &command_line[3..];
                if directory_name == ".." {
                    current_path.pop();
                    current_dir = get_current_path();
                } else {
                    current_path.push(directory_name);
                }
            }
            "ls" => {
                for output_line in lines.iter().skip(1) {
                    let split = output_line.split(" ").collect::<Vec<_>>();

                    match split[0] {
                        "dir" => {
                            let dir_name = split[1];
                            current_dir.children.insert(
                                dir_name.to_string(),
                                Entry::Dir(Dir {
                                    children: Map::new(),
                                }),
                            );
                        }
                        x if x.chars().all(char::is_numeric) => {
                            current_dir.children.insert(
                                split[1].to_string(),
                                Entry::File {
                                    size: x.parse().unwrap(),
                                },
                            );
                        }
                        x => unimplemented!("{}", x),
                    }
                }
            }
            _ => unimplemented!(),
        }
    }

    dbg!(&root);

    // let s: usize = root.dirs().map(|d| d.size()).filter(|&x| x < 100000).sum();

    // println!("{}", s);
}

fn parse_output_line(s: &str) -> Entry {
    let elements: Vec<&str> = s.split(" ").collect();

    todo!()
}

#[derive(Debug, Clone)]
enum Entry {
    File { size: usize },
    Dir(Dir),
}

#[derive(Debug, Clone)]
struct Shell {
    fs: Dir
}

impl Shell {
    pub fn dirs<'a>(&'a self) -> DirIter<'a> {
        DirIter {
            root: &self.fs,
            current_path: vec![],
        }
    }
}

#[derive(Debug, Clone)]
struct Dir {
    children: Map<String, Entry>,
}

impl Dir {
    fn size(&self) -> usize {
        self.children.iter().map(|(_, x)| x.size()).sum()
    }
}

impl Entry {
    fn size(&self) -> usize {
        match self {
            Entry::File { size } => *size,
            Entry::Dir(d) => d.size(),
        }
    }
}

struct DirIter<'a> {
    root: &'a Dir,
    current_path: Vec<usize>,
}

impl<'a> Iterator for DirIter<'a> {
    type Item = &'a Dir;

    fn next(&mut self) -> Option<Self::Item> {
        // increase current path
        {
            let mut dirs = vec![];
            for i in self.current_path.iter() {
                let d = match self.root.children.values().nth(*i).unwrap() {
                    Entry::Dir(d) => d,
                    _ => unreachable!(),
                };

                dirs.push(d);
            }

            for i in (0..self.current_path.len()).rev() {
                let dir = dirs[i];
                if self.current_path[i] < dir.children.len() {
                    self.current_path[i] += 1;
                    break;
                } else {
                    self.current_path.pop();
                }
            }
        }

        if self.current_path.len() == 0 {
            return None;
        }

        let mut dir = self.root;
        for idx in self.current_path.iter() {
            dir = match dir.children.values().nth(*idx).unwrap() {
                Entry::Dir(d) => d,
                _ => unreachable!(),
            }
        }

        Some(dir)
    }
}
