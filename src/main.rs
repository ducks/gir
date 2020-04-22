#[macro_use]
extern crate structopt;
extern crate ini;

mod lib;

use lib::repo::GitRepository;

fn main() {
    let repo = GitRepository::new();
    println!("{:#?}", repo);
}
