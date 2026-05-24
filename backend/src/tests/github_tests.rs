use blocksmith::github::verification::{
    verify_commit,
};

use blocksmith::github::commits::{
    CommitAuthor,
    CommitInfo,
    GithubCommit,
};

#[test]
fn test_commit_verification() {
    let commit = GithubCommit {
        sha: "abc123".to_string(),

        commit: CommitInfo {
            author: CommitAuthor {
                name:
                    "nirvanjain"
                        .to_string(),
            },

            message:
                "Initial commit"
                    .to_string(),
        },
    };

    assert!(verify_commit(
        &commit
    ));
}