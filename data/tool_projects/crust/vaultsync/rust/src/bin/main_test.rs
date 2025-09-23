use std::path::Path;

use VaultSync::vm::{add_changes, init_repo, make_commit, Author, Commit, Repository};

fn init_repository() {
    let author = Box::new(Author {
        mail: "john@example.com".to_string(),
        username: "john".to_string(),
    });

    let repo = init_repo(&author);
    if repo.is_ok() {
        println!("Init done");
    }
    assert!(repo.is_ok());
}

fn adding_changes() {
    let author = Author {
        mail: "john@example.com".to_string(),
        username: "john".to_string(),
    };

    let repo = Repository {
        name: "foo".to_string(),
        author: author,
        last_commit: None,
        dir: "/home/elhalili/workspace/VaultSync/build/mypr".to_string(),
    };

    let res = add_changes(
        &repo,
        &vec![Path::new(
            "/home/elhalili/workspace/VaultSync/build/mypr/hello.txt",
        )],
    );
    if res.is_ok() {
        println!("Changes added");
    }
    assert!(res.is_ok());
}

fn made_commit() {
    let author = Author {
        mail: "john@example.com".to_string(),
        username: "john".to_string(),
    };

    let mut repo = Repository {
        author: author.clone(),
        dir: "/home/elhalili/workspace/VaultSync/build/mypr".to_string(),
        name: "foo".to_string(),
        last_commit: Some(Commit {
            author: author.clone(),
            hash: "0".to_string(),
            parent_hash: "-".to_string(),
        }),
    };

    let mut commit = Commit {
        hash: "".to_string(),
        author: author.clone(),
        parent_hash: "".to_string(),
    };
    let res = make_commit(&repo, &author, &commit);
    if res.is_ok() {
        println!("Commit done");
    }
    assert!(res.is_ok());
}

#[test]
pub fn test() {
    init_repository();
    adding_changes();
    made_commit();
}
fn main(){}
