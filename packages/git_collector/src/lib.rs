pub use git2::*;
use bucket;
use std::fs;
use walkdir::WalkDir;
use std::io::Read;

const COMMIT_NAME: &str = "COMMIT-NAME";
const IS_HEAD: &str = "IS-HEAD";
const COMMIT_MESSAGE: &str = "COMMIT-MESSAGE";
const COMMIT_DESCRIPTION: &str = "COMMIT-DESCRIPTION";
const FILES: &str = "FILES";
const REMOTE_URL: &str = "REMOTE-URL";

pub fn create_bucket_from_head(repository: &Repository) -> Result<bucket::Bucket, git2::Error> {
    repository.checkout_head(None)?;

    let remote_name = repository.remotes().unwrap();
    let remote_name = remote_name.get(0).unwrap_or("").to_owned();
    let remote_url = if remote_name == "" {
        "".to_string()
    } else {
        let remote = repository.find_remote(&remote_name).unwrap();
        remote.url().unwrap().to_string()
    };

    let head = repository.head()?;
    let commit = head.peel_to_commit()?;
    let commit_name = commit.id().to_string();
    let commit_message = commit.message().unwrap_or("").to_string();
    let commit_description = commit.summary().unwrap_or("").to_string();
    let git_dir_root = repository.path().parent().unwrap();

    let mut bucket = bucket::Bucket::new();

    // set general info
    bucket.set(COMMIT_NAME, bucket::Value::String(commit_name));
    bucket.set(COMMIT_MESSAGE, bucket::Value::String(commit_message));
    bucket.set(COMMIT_DESCRIPTION, bucket::Value::String(commit_description));
    bucket.set(IS_HEAD, bucket::Value::Bool(true));
    bucket.set(REMOTE_URL, bucket::Value::String(remote_url));

    // set files
    let mut files_bucket = bucket::Bucket::new();
    let git_dir_root_as_str = String::from(git_dir_root.to_str().unwrap())
        .replace("\\", "/") + "/";
    for entry in WalkDir::new(git_dir_root)
        .into_iter()
        .filter_map(|x| x.ok())
        // filter only files and ignore /.git folder
        .filter(|x| x.path().is_file() && !x.path().to_str().unwrap().contains(".git")) {
        // create a relative path for the key
        let relative_path = String::from(entry.path().to_str().unwrap())
            .replace("\\", "/") // support both win and linux
            .replace(&git_dir_root_as_str, "");

        let mut file = fs::File::open(entry.path()).unwrap();
        // ignore files larger than 1mb
        // TODO: add this to options
        if file.metadata().unwrap().len() > 1_000_000 {
            continue;
        }
        let mut content = String::new();
        let result = file.read_to_string(&mut content);
        if result.is_ok() {
            files_bucket.set(&relative_path, bucket::Value::String(content));
        }
    }
    bucket.set(FILES, bucket::Value::Bucket(files_bucket));

    Ok(bucket)
}

#[cfg(test)]
mod tests {
    use crate::*;
    use bucket;
    use zip;
    use std::path;
    use serial_test::serial;

    fn get_test_repo() -> Repository {
        let file = fs::File::open("./test_repo.zip").expect("test_repo.zip archive is missing!");
        let mut archive = zip::ZipArchive::new(file).unwrap();
        fs::remove_dir("./test_artifacts/test_repo").unwrap_or_default();
        fs::create_dir_all("./test_artifacts/test_repo").unwrap();
        let repo_path = path::Path::new("./test_artifacts/test_repo");
        archive.extract(repo_path).unwrap();
        Repository::open(repo_path).unwrap()
    }

    #[test]
    #[serial(TestRepo)]
    fn basic_head_read() -> Result<(), git2::Error> {
        let repo = get_test_repo();

        let result = create_bucket_from_head(&repo)?;

        let mut bucket = bucket::Bucket::new();
        bucket.set(COMMIT_NAME, bucket::Value::String("501628ba7b2a3cedb39eaab767c4ead9991ff8ae".to_string()));
        bucket.set(COMMIT_MESSAGE, bucket::Value::String("Update file in folder\n".to_string()));
        bucket.set(COMMIT_DESCRIPTION, bucket::Value::String("Update file in folder".to_string()));
        bucket.set(IS_HEAD, bucket::Value::Bool(true));
        bucket.set(REMOTE_URL, bucket::Value::String("".to_string()));
        let mut files_bucket = bucket::Bucket::new();
        files_bucket.set("file.txt", bucket::Value::String("test string file".to_string()));
        files_bucket.set("folder/file", bucket::Value::String("file2 content".to_string()));
        bucket.set(FILES, bucket::Value::Bucket(files_bucket));
        assert_eq!(result, bucket);
        Ok(())
    }

    #[allow(dead_code)]
    fn check_if_ignores_big_files() {
        todo!();
    }
}
