use git2::{Repository, Tag, Oid, Revwalk, Error};

fn main() {
    // start repo
    let repo = Repository::open("/path/to/repo").unwrap();
    let start_commit = String::from("start");
    let end_commit = String::from("end");

    get_merged_pull_requests(&repo, start_commit, end_commit);
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
