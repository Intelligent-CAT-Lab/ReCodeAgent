use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
pub const MAX_UNAME: usize = 50;
pub const MAX_MAIL: usize = 100;
pub const CONFIG_FILE: &str = "/config";
pub const REPOSITORY_FILE: &str = "/repository";
pub const LOG_FILE: &str = "/log";
pub const MAX_REPOSITORY_FILE_INIT: usize = 1024;
pub const MAX_CWD: usize = 4096;
pub const MAX_REP_NAME: usize = 80;
#[derive(Clone)]
pub struct RepoValue {
    pub raw_path: String,
    pub hash: String,
}
#[derive(Clone)]
pub struct Author {
    pub username: String,
    pub mail: String,
}
pub const TRACKED_DIR: &str = "tracked";
pub const HASH_LEN: usize = 2 * 32 + 1;
#[derive(Clone)]
pub struct Commit {
    pub hash: String,
    pub author: Author,
    pub parent_hash: String,
}
#[derive(Clone)]
pub struct Repository {
    pub name: String,
    pub author: Author,
    pub dir: String,
    pub last_commit: Option<Commit>,
}
pub fn init_repo(author: &Author) -> Result<Repository, String> {
    unimplemented!()
}
pub fn write_repository_file(repo: &Repository) -> Result<(), String> {
    unimplemented!()
}
pub fn load_repository() -> Repository {
    unimplemented!()
}
pub fn load_author() -> Author {
    unimplemented!()
}
// commit.h
pub fn make_init_map_repo(
    repo: &Repository,
    map: &mut HashMap<String, RepoValue>,
    path: &str,
) -> Result<(), String> {
    unimplemented!()
}
pub fn add_changes(repo: &Repository, files: &Vec<&Path>) -> Result<(), String> {
    unimplemented!()
}
pub fn make_changes(repo: &Repository, map: &mut HashMap<String, RepoValue>) -> Result<(), String> {
    unimplemented!()
}
pub fn create_commit(
    repo: &Repository,
    commit: &Commit,
    map: &HashMap<String, RepoValue>,
) -> Result<(), String> {
    unimplemented!()
}
pub fn make_commit(repo: &Repository, author: &Author, commit: &Commit) -> Result<(), String> {
    unimplemented!()
}
pub fn delete_tracked_dir(repo: &Repository) -> Result<(), String> {
    unimplemented!()
}
pub fn rollback(repo: &Repository, commit_hash: &str) -> Result<(), String> {
    unimplemented!()
}
pub fn reset_repo_dir(path: &str, root_path: &str) -> Result<(), String> {
    unimplemented!()
}
pub fn make_rollback_commit(map: &mut HashMap<String, RepoValue>) -> Result<(), String> {
    unimplemented!()
}
pub fn load_commit(repo: &Repository, commit_hash: &str) -> Result<Commit, String> {
    unimplemented!()
}