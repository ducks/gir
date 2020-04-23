use ini::Ini;
use std::env;
use std::fmt;
use std::path::PathBuf;

pub struct GitRepository {
    worktree: PathBuf,
    git_dir: PathBuf,
    config: Ini,
}

impl fmt::Debug for GitRepository {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        println!("config:");
        for (sec, prop) in self.config.iter() {
			let section_name = sec.as_ref().unwrap();
			println!("-- Section: {:?} begins", section_name);
			for (k, v) in prop.iter() {
				println!("{}: {:?}", k, v);
			}
        }
        write!(f, "worktree:{:#?} git_dir:{:#?}", self.worktree, self.git_dir)
    }
}

impl GitRepository {
    pub fn init() -> GitRepository {
        let mut current_dir = env::current_dir().unwrap();

        let worktree = &current_dir;
        let wt = worktree.to_path_buf();

        let mut git_dir = &mut current_dir;
        git_dir.push(".git");
        let gd = git_dir.to_path_buf();

        if git_dir.exists() == false {
          panic!("Not a git repository!");
        }

        let mut config_path = &mut current_dir;
        config_path.push("config");

        let cf = config_path.as_os_str();

        let _config = Ini::load_from_file(cf).unwrap();

        GitRepository {
            worktree: wt,
            git_dir: gd,
            config: _config,
        }
    }

    pub fn repo_path(&self, path: PathBuf) -> PathBuf {
    }
}
