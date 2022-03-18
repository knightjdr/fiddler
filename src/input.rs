use std::collections::HashMap;
use std::path::Path;

#[derive(Eq)]
#[derive(Hash)]
#[derive(PartialEq)]
enum Project {
  Node,
  Rust,
}

fn identify_project() -> Vec<Project> {
  let mut projects = Vec::new();
  if Path::new("./Cargo.toml").exists() {
    projects.push(Project::Rust);
  } if Path::new("./package.json").exists() {
    projects.push(Project::Node);
  }
  return projects;
}


fn suggest(input: &str, projects: &Vec<Project>) {
  let commands = HashMap::from([
    (
      Project::Rust, 
      [
        "cargo run",
        "cargo build",
      ],
    ),
  ]);

  if projects.contains(&Project::Rust) {
    let index = commands.get(&Project::Rust).unwrap().iter().position(|&x| x.starts_with(input));
    match index {
      Some(index) => println!("{}", commands.get(&Project::Rust).unwrap()[index]),
      None => println!("No command found"),
    }
  }
}

pub fn parse_input(input: &str) {
  let projects = identify_project();
  suggest(input, &projects);
}