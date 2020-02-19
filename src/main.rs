#[macro_use]
extern crate structopt;
extern crate ini;

use ini::Ini;
use std::env;
use std::path::PathBuf;

mod lib;

#[derive(StructOpt, Debug)]
enum GitCommands {
    Add {

    },
    Init {

    },
    Status {

    }
}

#[derive(Debug)]
struct GitRepository {
    worktree: PathBuf,
    gitdir: PathBuf,
}

impl GitRepository {
    pub fn new() -> GitRepository {
        let worktree = env::current_dir().unwrap();
        let gitdir = worktree.push(".git");

        if gitdir.exists() == false {
          panic!("Not a git repository!");
        }

        let config = Ini::load_from_file("config").unwrap();

        GitRepository {
            worktree: worktree,
            gitdir: gitdir,
            config: config,
        }
    }
}

fn main() {
    let repo = GitRepository::new();
    println!("{:?}", repo);
}
