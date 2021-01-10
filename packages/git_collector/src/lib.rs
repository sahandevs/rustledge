use git2::*;
use bucket;
use std::fs;

const COMMIT_NAME: &str = "COMMIT-NAME";
const IS_HEAD: &str = "IS-HEAD";
const COMMIT_MESSAGE: &str = "COMMIT-MESSAGE";
const COMMIT_DESCRIPTION: &str = "COMMIT-DESCRIPTION";
const FILES: &str = "FILES";

fn test() -> Result<(), git2::Error> {
    let repo = Repository::init("./test_artifacts")?;
    repo.head().unwrap();
    let desc = repo.describe(&DescribeOptions::new())?;
    println!("{}", desc.format(None)?);
    Ok(())
}

pub fn create_bucket_from_head(repository: &Repository) -> Result<bucket::Bucket, git2::Error> {
    repository.checkout_head(None)?;
    let head = repository.head()?;
    let commit = head.peel_to_commit()?;
    let commit_name = commit.id().to_string();
    let commit_message = commit.message().unwrap_or("").to_string();
    let commit_description = commit.summary().unwrap_or("").to_string();
    println!("{:?}", repository.path());

    let mut bucket = bucket::Bucket::new();
    bucket.set(COMMIT_NAME, bucket::Value::String(commit_name));
    bucket.set(COMMIT_MESSAGE, bucket::Value::String(commit_message));
    bucket.set(COMMIT_DESCRIPTION, bucket::Value::String(commit_description));
    bucket.set(IS_HEAD, bucket::Value::Bool(true));
    let mut files_bucket = bucket::Bucket::new();
    files_bucket.set("file.txt", bucket::Value::String("test string file".to_string()));
    files_bucket.set("folder/file", bucket::Value::String("file2 content".to_string()));
    bucket.set(FILES, bucket::Value::Bucket(files_bucket));
    todo!()
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
    fn it_works() -> Result<(), git2::Error> {
        let repo = get_test_repo();

        let result = create_bucket_from_head(&repo)?;

        let mut bucket = bucket::Bucket::new();
        bucket.set(COMMIT_NAME, bucket::Value::String("placeholder".to_string()));
        bucket.set(COMMIT_MESSAGE, bucket::Value::String("placeholder".to_string()));
        bucket.set(COMMIT_DESCRIPTION, bucket::Value::String("placeholder".to_string()));
        bucket.set(IS_HEAD, bucket::Value::Bool(true));
        let mut files_bucket = bucket::Bucket::new();
        files_bucket.set("file.txt", bucket::Value::String("test string file".to_string()));
        files_bucket.set("folder/file", bucket::Value::String("file2 content".to_string()));
        bucket.set(FILES, bucket::Value::Bucket(files_bucket));
        assert_eq!(result, bucket);
        Ok(())
    }
}
