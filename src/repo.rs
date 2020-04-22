#[derive(Debug)]
struct GitRepository {
    worktree: PathBuf,
    gitdir: PathBuf,
    config: Ini,
}

impl GitRepository {
    pub fn init() -> GitRepository {
        let current_dir = env::current_dir().unwrap();

        let worktree = &current_dir;
        let wt = worktree.to_path_buf();

        let mut gitdir = &current_dir;
        gitdir.push(".git");
        let gd = gitdir.to_path_buf();

        if gitdir.exists() == false {
          panic!("Not a git repository!");
        }

        let mut config_path = &current_dir;
        config_path.push(".git/config");

        let cf = config_path.as_os_str();

        let _config = Ini::load_from_file(cf).unwrap();

        GitRepository {
            worktree: wt,
            gitdir: gd,
            config: _config,
        }
    }
}

pub fn repo_path() -> PathBuf {

}

pub fn repo_file() -> PathBuf {

}
