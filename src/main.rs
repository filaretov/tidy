use git2::Repository;
use walkdir::{DirEntry, WalkDir};

fn is_dir(entry: &DirEntry) -> bool {
    entry.file_type().is_dir()
}
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut walker = WalkDir::new("/home/hgf/dev")
        .into_iter()
        .filter_entry(is_dir);
    loop {
        let entry = match walker.next() {
            None => break,
            Some(Err(err)) => panic!("Error: {}", err),
            Some(Ok(entry)) => entry,
        };

        if let Ok(repo) = Repository::open(entry.path()) {
            git_is_dirty(&repo)?;
            walker.skip_current_dir();
        }
    }
    Ok(())
}

fn git_is_dirty(repo: &Repository) -> Result<bool> {
    let head = match repo.head() {
        Ok(head) => Some(head),
        Err(_err) => None,
    };
    let tree = head.and_then(|head| head.peel_to_tree().ok());
    let changes = repo
        .diff_tree_to_workdir_with_index(tree.as_ref(), None)?
        .stats()?
        .files_changed();
    if changes > 0 {
        print!("{:?} ", repo.path());
        println!("dirty");
        return Ok(true);
    }

    Ok(false)
}
