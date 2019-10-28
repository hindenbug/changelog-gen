use git2::Repository;
use docopt::Docopt;
use serde::Deserialize;

const USAGE: &str = "
Usage:
  changelog-gen [options] <git-dir>

Options:
    --start <start>     start commit
    --end <end>         end commit
    -h, --help          show this message`
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_git_dir: String,
    flag_start: Option<String>,
    flag_end: Option<String>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    match run(&args) {
        Ok(()) => {}
        Err(e) => println!("error: {}", e),
    }
}

fn run(args: &Args) -> Result<(), git2::Error> {
    // start repo
    let path = args.arg_git_dir.as_str();
    let repo = Repository::open(path)?;
    let start = args.flag_start.as_ref().unwrap();
    let end = args.flag_end.as_ref().unwrap();
    let start_commit = String::from(start);
    let end_commit = String::from(end);

    get_merged_pull_requests(&repo, start_commit, end_commit);
    Ok(())
}

fn get_merged_pull_requests(repo: &Repository, start_commit: String, end_commit: String) {
    let mut revwalk = repo.revwalk().unwrap();
    revwalk.set_sorting(git2::Sort::NONE);

    let commit_range: &str = &*format!("{}..{}", start_commit, end_commit);
    revwalk.push_range(commit_range).unwrap();
    revwalk.simplify_first_parent();

    for id in revwalk {
        let id_str: &str = &*id.unwrap().to_string();
        let commit_obj = repo.revparse_single(id_str).unwrap();

        println!("{}", commit_obj.as_commit().unwrap().message().unwrap());
    }
}
