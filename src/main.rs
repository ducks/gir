#[macro_use]
extern crate structopt;
extern crate ini;

mod commands;
mod repo;

use repo::GitRepository;

fn main() {
    let repo = GitRepository::init();
    println!("{:#?}", repo);
}
