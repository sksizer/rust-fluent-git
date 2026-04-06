//! API specification tests for fluent-git.
//!
//! Each test section shows sync and async versions side-by-side.
//! The only difference is:
//!   - sync:  fluent_git::sync::git::*  / .run()
//!   - async: fluent_git::git::*        / .run_async().await
//!
//! Errors and result types are shared — they come from:
//!   - fluent_git::error::*
//!   - fluent_git::types::*
//!
//! Key conventions:
//!   - .run() returns Result<T, DomainError> (not GitError)
//!   - GitError only appears when combining multiple operations via ?
//!   - stash().push().run() — explicit sub-command
//!   - rev_parse("HEAD").run() — consistent builder pattern
//!   - Repo.path is private, accessed via .path()

#[cfg(test)]
mod tests {
    use fluent_git::error::*;
    use fluent_git::types::*;
    use std::env::temp_dir;
    use std::fs;
    use std::path::PathBuf;

    fn get_temp_dir(test_name: &str) -> PathBuf {
        let dir = temp_dir().join("fluent-git-tests").join(test_name);
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    /// Configure git identity in a repo so commits work on CI (no global config).
    #[cfg(feature = "blocking")]
    fn configure_test_identity(repo: &Repo) {
        repo.config().set("user.name", "Test").run().unwrap();
        repo.config().set("user.email", "test@test.com").run().unwrap();
    }

    /// Async version of configure_test_identity.
    #[cfg(feature = "tokio")]
    async fn configure_test_identity_async(repo: &Repo) {
        repo.config().set("user.name", "Test").run_async().await.unwrap();
        repo.config().set("user.email", "test@test.com").run_async().await.unwrap();
    }

    // ══════════════════════════════════════════════════════════════════════
    // Setup: System info
    // ══════════════════════════════════════════════════════════════════════

    #[test]
    fn sync_git_is_available() {
        assert!(fluent_git::info::available());
    }

    // async: info::available() is not async — it's a simple PATH check

    #[test]
    fn sync_git_info() {
        let info = fluent_git::info::get().unwrap();
        assert!(!info.version.is_empty());
        assert!(info.version.chars().next().unwrap().is_ascii_digit());
        assert!(info.path.exists());
    }

    // TODO: async_git_info — fluent_git::info::get_async().await

    #[test]
    fn setup_error_matching_is_exhaustive() {
        let err = SetupError::NotInstalled;
        let msg = match &err {
            SetupError::NotInstalled => "install git",
            SetupError::VersionTooOld { .. } => "upgrade git",
            SetupError::Command(_) => "command failed",
            SetupError::Io(_) => "io error",
        };
        assert_eq!(msg, "install git");
    }

    // ══════════════════════════════════════════════════════════════════════
    // Setup: Init
    // ══════════════════════════════════════════════════════════════════════

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_init_creates_repo() {
        let dir = get_temp_dir("sync_init_creates");
        let result: Result<InitResult, InitError> = fluent_git::sync::git::init(&dir).run();
        let result = result.unwrap();
        assert!(dir.join(".git").exists());
        assert_eq!(result.path, dir);
        assert!(!result.bare);
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_init_creates_repo() {
        let dir = get_temp_dir("async_init_creates");
        let result: Result<InitResult, InitError> = fluent_git::git::init(&dir).run().await;
        let result = result.unwrap();
        assert!(dir.join(".git").exists());
        assert_eq!(result.path, dir);
        assert!(!result.bare);
    }

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_init_bare() {
        let dir = get_temp_dir("sync_init_bare");
        let result = fluent_git::sync::git::init(&dir).bare().run().unwrap();
        assert!(dir.join("HEAD").exists());
        assert!(result.bare);
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_init_bare() {
        let dir = get_temp_dir("async_init_bare");
        let result = fluent_git::git::init(&dir).bare().run().await.unwrap();
        assert!(dir.join("HEAD").exists());
        assert!(result.bare);
    }

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_init_with_initial_branch() {
        let dir = get_temp_dir("sync_init_branch");
        let result = fluent_git::sync::git::init(&dir).initial_branch("main").run().unwrap();
        assert_eq!(result.branch, "main");
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_init_with_initial_branch() {
        let dir = get_temp_dir("async_init_branch");
        let result = fluent_git::git::init(&dir).initial_branch("main").run().await.unwrap();
        assert_eq!(result.branch, "main");
    }

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_init_into_repo() {
        let dir = get_temp_dir("sync_init_into_repo");
        let repo: Repo = fluent_git::sync::git::init(&dir).run().unwrap().into_repo();
        assert_eq!(repo.path(), dir.as_path());
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_init_into_repo() {
        let dir = get_temp_dir("async_init_into_repo");
        let repo: Repo = fluent_git::git::init(&dir).run().await.unwrap().into_repo();
        assert_eq!(repo.path(), dir.as_path());
    }

    #[test]
    fn init_error_matching_is_tight() {
        let err = InitError::AlreadyExists { path: "/tmp/x".into() };
        let msg = match &err {
            InitError::AlreadyExists { path } => format!("exists at {}", path.display()),
            InitError::PermissionDenied { path } => format!("denied at {}", path.display()),
            InitError::InvalidBranchName { name, .. } => format!("bad name: {name}"),
            InitError::Command(_) => "git failed".into(),
            InitError::Io(_) => "io error".into(),
        };
        assert!(msg.contains("/tmp/x"));
    }

    // ══════════════════════════════════════════════════════════════════════
    // Setup: Open
    // ══════════════════════════════════════════════════════════════════════

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_open_existing_repo() {
        let dir = get_temp_dir("sync_open_existing");
        fluent_git::sync::git::init(&dir).run().unwrap();
        let repo: Result<Repo, OpenError> = fluent_git::sync::git::open(&dir);
        assert_eq!(repo.unwrap().path(), dir.as_path());
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_open_existing_repo() {
        let dir = get_temp_dir("async_open_existing");
        fluent_git::git::init(&dir).run().await.unwrap();
        let repo: Result<Repo, OpenError> = fluent_git::git::open(&dir).await;
        assert_eq!(repo.unwrap().path(), dir.as_path());
    }

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_open_nonexistent_fails() {
        let dir = get_temp_dir("sync_open_noexist");
        let _ = fs::remove_dir_all(&dir);
        let err: OpenError = fluent_git::sync::git::open(&dir).unwrap_err();
        assert!(matches!(err, OpenError::NotAccessible { .. }));
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_open_nonexistent_fails() {
        let dir = get_temp_dir("async_open_noexist");
        let _ = fs::remove_dir_all(&dir);
        let err: OpenError = fluent_git::git::open(&dir).await.unwrap_err();
        assert!(matches!(err, OpenError::NotAccessible { .. }));
    }

    // ══════════════════════════════════════════════════════════════════════
    // Setup: Clone
    // ══════════════════════════════════════════════════════════════════════

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_clone_local_repo() {
        let origin = get_temp_dir("sync_clone_origin");
        fluent_git::sync::git::init(&origin).run().unwrap();
        let dest = get_temp_dir("sync_clone_dest");
        let result: Result<CloneResult, CloneError> = fluent_git::sync::git::clone(&origin).into(&dest).run();
        let result = result.unwrap();
        assert!(dest.join(".git").exists());
        assert!(!result.shallow);
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_clone_local_repo() {
        let origin = get_temp_dir("async_clone_origin");
        fluent_git::git::init(&origin).run().await.unwrap();
        let dest = get_temp_dir("async_clone_dest");
        let result: Result<CloneResult, CloneError> = fluent_git::git::clone(&origin).into(&dest).run().await;
        let result = result.unwrap();
        assert!(dest.join(".git").exists());
        assert!(!result.shallow);
    }

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_clone_with_depth() {
        let origin = get_temp_dir("sync_clone_depth_origin");
        fluent_git::sync::git::init(&origin).run().unwrap();
        let dest = get_temp_dir("sync_clone_depth_dest");
        let result = fluent_git::sync::git::clone(&origin).depth(1).into(&dest).run().unwrap();
        assert!(result.shallow);
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_clone_with_depth() {
        let origin = get_temp_dir("async_clone_depth_origin");
        fluent_git::git::init(&origin).run().await.unwrap();
        let dest = get_temp_dir("async_clone_depth_dest");
        let result = fluent_git::git::clone(&origin).depth(1).into(&dest).run().await.unwrap();
        assert!(result.shallow);
    }

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_clone_specific_branch() {
        let origin = get_temp_dir("sync_clone_branch_origin");
        let origin_repo = fluent_git::sync::git::init(&origin).initial_branch("main").run().unwrap().into_repo();
        configure_test_identity(&origin_repo);
        fs::write(origin.join("dummy.txt"), "x").unwrap();
        origin_repo.add().all().run().unwrap();
        origin_repo.commit().message("init").run().unwrap();
        let dest = get_temp_dir("sync_clone_branch_dest");
        let result = fluent_git::sync::git::clone(&origin).branch("main").into(&dest).run().unwrap();
        assert_eq!(result.branch, "main");
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_clone_specific_branch() {
        let origin = get_temp_dir("async_clone_branch_origin");
        let origin_repo = fluent_git::git::init(&origin).initial_branch("main").run().await.unwrap().into_repo();
        configure_test_identity_async(&origin_repo).await;
        fs::write(origin.join("dummy.txt"), "x").unwrap();
        origin_repo.add().all().run_async().await.unwrap();
        origin_repo.commit().message("init").run_async().await.unwrap();
        let dest = get_temp_dir("async_clone_branch_dest");
        let result = fluent_git::git::clone(&origin).branch("main").into(&dest).run().await.unwrap();
        assert_eq!(result.branch, "main");
    }

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_clone_with_remote_name() {
        let origin = get_temp_dir("sync_clone_remote_origin");
        fluent_git::sync::git::init(&origin).run().unwrap();
        let dest = get_temp_dir("sync_clone_remote_dest");
        let result = fluent_git::sync::git::clone(&origin).remote_name("upstream").into(&dest).run().unwrap();
        assert_eq!(result.remote, "upstream");
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_clone_with_remote_name() {
        let origin = get_temp_dir("async_clone_remote_origin");
        fluent_git::git::init(&origin).run().await.unwrap();
        let dest = get_temp_dir("async_clone_remote_dest");
        let result = fluent_git::git::clone(&origin).remote_name("upstream").into(&dest).run().await.unwrap();
        assert_eq!(result.remote, "upstream");
    }

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_clone_into_repo() {
        let origin = get_temp_dir("sync_clone_into_repo_origin");
        fluent_git::sync::git::init(&origin).run().unwrap();
        let dest = get_temp_dir("sync_clone_into_repo_dest");
        let repo = fluent_git::sync::git::clone(&origin).into(&dest).run().unwrap().into_repo();
        assert_eq!(repo.path(), dest.as_path());
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_clone_into_repo() {
        let origin = get_temp_dir("async_clone_into_repo_origin");
        fluent_git::git::init(&origin).run().await.unwrap();
        let dest = get_temp_dir("async_clone_into_repo_dest");
        let repo = fluent_git::git::clone(&origin).into(&dest).run().await.unwrap().into_repo();
        assert_eq!(repo.path(), dest.as_path());
    }

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_clone_with_mutate() {
        let origin = get_temp_dir("sync_clone_mutate_origin");
        fluent_git::sync::git::init(&origin).run().unwrap();
        let dest = get_temp_dir("sync_clone_mutate_dest");
        let shallow = true;
        let mut clone = fluent_git::sync::git::clone(&origin).into(&dest).mutate();
        if shallow {
            clone.depth(1);
        }
        clone.finish().run().unwrap();
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_clone_with_mutate() {
        let origin = get_temp_dir("async_clone_mutate_origin");
        fluent_git::git::init(&origin).run().await.unwrap();
        let dest = get_temp_dir("async_clone_mutate_dest");
        let shallow = true;
        let mut clone = fluent_git::git::clone(&origin).into(&dest).mutate();
        if shallow {
            clone.depth(1);
        }
        clone.finish().run().await.unwrap();
    }

    // ══════════════════════════════════════════════════════════════════════
    // Repo: Add
    // ══════════════════════════════════════════════════════════════════════

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_add_single_file() {
        let dir = get_temp_dir("sync_add_single");
        let repo = fluent_git::sync::git::init(&dir).run().unwrap().into_repo();
        fs::write(dir.join("hello.txt"), "hello").unwrap();
        let result: Result<AddResult, AddError> = repo.add().path("hello.txt").run();
        assert_eq!(result.unwrap().files.len(), 1);
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_add_single_file() {
        let dir = get_temp_dir("async_add_single");
        let repo = fluent_git::git::init(&dir).run().await.unwrap().into_repo();
        fs::write(dir.join("hello.txt"), "hello").unwrap();
        let result: Result<AddResult, AddError> = repo.add().path("hello.txt").run_async().await;
        assert_eq!(result.unwrap().files.len(), 1);
    }

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_add_multiple_files() {
        let dir = get_temp_dir("sync_add_multi");
        let repo = fluent_git::sync::git::init(&dir).run().unwrap().into_repo();
        fs::write(dir.join("a.txt"), "a").unwrap();
        fs::write(dir.join("b.txt"), "b").unwrap();
        let result = repo.add().path("a.txt").path("b.txt").run().unwrap();
        assert_eq!(result.files.len(), 2);
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_add_multiple_files() {
        let dir = get_temp_dir("async_add_multi");
        let repo = fluent_git::git::init(&dir).run().await.unwrap().into_repo();
        fs::write(dir.join("a.txt"), "a").unwrap();
        fs::write(dir.join("b.txt"), "b").unwrap();
        let result = repo.add().path("a.txt").path("b.txt").run_async().await.unwrap();
        assert_eq!(result.files.len(), 2);
    }

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_add_all() {
        let dir = get_temp_dir("sync_add_all");
        let repo = fluent_git::sync::git::init(&dir).run().unwrap().into_repo();
        fs::write(dir.join("a.txt"), "a").unwrap();
        repo.add().all().run().unwrap();
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_add_all() {
        let dir = get_temp_dir("async_add_all");
        let repo = fluent_git::git::init(&dir).run().await.unwrap().into_repo();
        fs::write(dir.join("a.txt"), "a").unwrap();
        repo.add().all().run_async().await.unwrap();
    }

    // ══════════════════════════════════════════════════════════════════════
    // Repo: Commit
    // ══════════════════════════════════════════════════════════════════════

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_commit_with_message() {
        let dir = get_temp_dir("sync_commit_msg");
        let repo = fluent_git::sync::git::init(&dir).run().unwrap().into_repo();
        configure_test_identity(&repo);
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().all().run().unwrap();
        let result: Result<CommitResult, CommitError> = repo.commit().message("initial").run();
        let result = result.unwrap();
        assert_eq!(result.sha.len(), 40);
        assert_eq!(result.short_sha.len(), 7);
        assert_eq!(result.message, "initial");
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_commit_with_message() {
        let dir = get_temp_dir("async_commit_msg");
        let repo = fluent_git::git::init(&dir).run().await.unwrap().into_repo();
        configure_test_identity_async(&repo).await;
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().all().run_async().await.unwrap();
        let result: Result<CommitResult, CommitError> = repo.commit().message("initial").run_async().await;
        let result = result.unwrap();
        assert_eq!(result.sha.len(), 40);
        assert_eq!(result.short_sha.len(), 7);
        assert_eq!(result.message, "initial");
    }

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_commit_with_author() {
        let dir = get_temp_dir("sync_commit_author");
        let repo = fluent_git::sync::git::init(&dir).run().unwrap().into_repo();
        configure_test_identity(&repo);
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().all().run().unwrap();
        let result = repo.commit().message("init").author("Test", "t@t.com").run().unwrap();
        assert_eq!(result.author.name, "Test");
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_commit_with_author() {
        let dir = get_temp_dir("async_commit_author");
        let repo = fluent_git::git::init(&dir).run().await.unwrap().into_repo();
        configure_test_identity_async(&repo).await;
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().all().run_async().await.unwrap();
        let result = repo.commit().message("init").author("Test", "t@t.com").run_async().await.unwrap();
        assert_eq!(result.author.name, "Test");
    }

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_commit_allow_empty() {
        let dir = get_temp_dir("sync_commit_empty");
        let repo = fluent_git::sync::git::init(&dir).run().unwrap().into_repo();
        configure_test_identity(&repo);
        let result = repo.commit().message("empty").allow_empty().run().unwrap();
        assert_eq!(result.files_changed, 0);
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_commit_allow_empty() {
        let dir = get_temp_dir("async_commit_empty");
        let repo = fluent_git::git::init(&dir).run().await.unwrap().into_repo();
        configure_test_identity_async(&repo).await;
        let result = repo.commit().message("empty").allow_empty().run_async().await.unwrap();
        assert_eq!(result.files_changed, 0);
    }

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_commit_amend() {
        let dir = get_temp_dir("sync_commit_amend");
        let repo = fluent_git::sync::git::init(&dir).run().unwrap().into_repo();
        configure_test_identity(&repo);
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().all().run().unwrap();
        repo.commit().message("initial").run().unwrap();
        let result = repo.commit().amend().message("amended").run().unwrap();
        assert_eq!(result.message, "amended");
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_commit_amend() {
        let dir = get_temp_dir("async_commit_amend");
        let repo = fluent_git::git::init(&dir).run().await.unwrap().into_repo();
        configure_test_identity_async(&repo).await;
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().all().run_async().await.unwrap();
        repo.commit().message("initial").run_async().await.unwrap();
        let result = repo.commit().amend().message("amended").run_async().await.unwrap();
        assert_eq!(result.message, "amended");
    }

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_commit_nothing_staged_fails() {
        let dir = get_temp_dir("sync_commit_no_staged");
        let repo = fluent_git::sync::git::init(&dir).run().unwrap().into_repo();
        configure_test_identity(&repo);
        let err: CommitError = repo.commit().message("nothing").run().unwrap_err();
        assert!(matches!(err, CommitError::NothingToCommit));
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_commit_nothing_staged_fails() {
        let dir = get_temp_dir("async_commit_no_staged");
        let repo = fluent_git::git::init(&dir).run().await.unwrap().into_repo();
        configure_test_identity_async(&repo).await;
        let err: CommitError = repo.commit().message("nothing").run_async().await.unwrap_err();
        assert!(matches!(err, CommitError::NothingToCommit));
    }

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_commit_with_closure() {
        let dir = get_temp_dir("sync_commit_with");
        let repo = fluent_git::sync::git::init(&dir).run().unwrap().into_repo();
        configure_test_identity(&repo);
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().all().run().unwrap();
        let no_verify = true;
        repo.commit()
            .message("fast")
            .with(|c| {
                if no_verify {
                    c.no_verify();
                }
            })
            .run()
            .unwrap();
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_commit_with_closure() {
        let dir = get_temp_dir("async_commit_with");
        let repo = fluent_git::git::init(&dir).run().await.unwrap().into_repo();
        configure_test_identity_async(&repo).await;
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().all().run_async().await.unwrap();
        let no_verify = true;
        repo.commit()
            .message("fast")
            .with(|c| {
                if no_verify {
                    c.no_verify();
                }
            })
            .run_async()
            .await
            .unwrap();
    }

    // ══════════════════════════════════════════════════════════════════════
    // Repo: Branch
    // ══════════════════════════════════════════════════════════════════════

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_branch_create_list_delete_rename() {
        let dir = get_temp_dir("sync_branch_lifecycle");
        let repo = fluent_git::sync::git::init(&dir).initial_branch("main").run().unwrap().into_repo();
        configure_test_identity(&repo);
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().all().run().unwrap();
        repo.commit().message("init").run().unwrap();

        repo.branch().create("feature").run().unwrap();
        repo.branch().create("to-rename").run().unwrap();

        let branches: Result<Vec<BranchInfo>, BranchError> = repo.branch().list().run();
        let branches = branches.unwrap();
        assert!(branches.len() >= 3);
        assert_eq!(branches.iter().find(|b| b.is_current).unwrap().name, "main");

        repo.branch().rename("to-rename", "renamed").run().unwrap();
        repo.branch().delete("feature").run().unwrap();

        let current: Result<String, BranchError> = repo.branch().current();
        assert_eq!(current.unwrap(), "main");
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_branch_create_list_delete_rename() {
        let dir = get_temp_dir("async_branch_lifecycle");
        let repo = fluent_git::git::init(&dir).initial_branch("main").run().await.unwrap().into_repo();
        configure_test_identity_async(&repo).await;
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().all().run_async().await.unwrap();
        repo.commit().message("init").run_async().await.unwrap();

        repo.branch().create("feature").run_async().await.unwrap();
        repo.branch().create("to-rename").run_async().await.unwrap();

        let branches: Result<Vec<BranchInfo>, BranchError> = repo.branch().list().run_async().await;
        let branches = branches.unwrap();
        assert!(branches.len() >= 3);
        assert_eq!(branches.iter().find(|b| b.is_current).unwrap().name, "main");

        repo.branch().rename("to-rename", "renamed").run_async().await.unwrap();
        repo.branch().delete("feature").run_async().await.unwrap();

        let current: Result<String, BranchError> = repo.branch().current_async().await;
        assert_eq!(current.unwrap(), "main");
    }

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_branch_duplicate_fails() {
        let dir = get_temp_dir("sync_dup_branch");
        let repo = fluent_git::sync::git::init(&dir).run().unwrap().into_repo();
        configure_test_identity(&repo);
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().all().run().unwrap();
        repo.commit().message("init").run().unwrap();
        repo.branch().create("x").run().unwrap();
        assert!(matches!(repo.branch().create("x").run().unwrap_err(), BranchError::AlreadyExists { .. }));
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_branch_duplicate_fails() {
        let dir = get_temp_dir("async_dup_branch");
        let repo = fluent_git::git::init(&dir).run().await.unwrap().into_repo();
        configure_test_identity_async(&repo).await;
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().all().run_async().await.unwrap();
        repo.commit().message("init").run_async().await.unwrap();
        repo.branch().create("x").run_async().await.unwrap();
        assert!(matches!(repo.branch().create("x").run_async().await.unwrap_err(), BranchError::AlreadyExists { .. }));
    }

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_delete_current_branch_fails() {
        let dir = get_temp_dir("sync_del_current");
        let repo = fluent_git::sync::git::init(&dir).initial_branch("main").run().unwrap().into_repo();
        configure_test_identity(&repo);
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().all().run().unwrap();
        repo.commit().message("init").run().unwrap();
        assert!(matches!(repo.branch().delete("main").run().unwrap_err(), BranchError::DeleteCurrent { .. }));
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_delete_current_branch_fails() {
        let dir = get_temp_dir("async_del_current");
        let repo = fluent_git::git::init(&dir).initial_branch("main").run().await.unwrap().into_repo();
        configure_test_identity_async(&repo).await;
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().all().run_async().await.unwrap();
        repo.commit().message("init").run_async().await.unwrap();
        assert!(matches!(
            repo.branch().delete("main").run_async().await.unwrap_err(),
            BranchError::DeleteCurrent { .. }
        ));
    }

    // ══════════════════════════════════════════════════════════════════════
    // Repo: Checkout
    // ══════════════════════════════════════════════════════════════════════

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_checkout_branch_and_new_branch() {
        let dir = get_temp_dir("sync_checkout");
        let repo = fluent_git::sync::git::init(&dir).run().unwrap().into_repo();
        configure_test_identity(&repo);
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().all().run().unwrap();
        repo.commit().message("init").run().unwrap();
        repo.branch().create("feature").run().unwrap();
        let result: Result<(), CheckoutError> = repo.checkout().branch("feature").run();
        result.unwrap();
        assert_eq!(repo.branch().current().unwrap(), "feature");
        repo.checkout().new_branch("feature2").run().unwrap();
        assert_eq!(repo.branch().current().unwrap(), "feature2");
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_checkout_branch_and_new_branch() {
        let dir = get_temp_dir("async_checkout");
        let repo = fluent_git::git::init(&dir).run().await.unwrap().into_repo();
        configure_test_identity_async(&repo).await;
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().all().run_async().await.unwrap();
        repo.commit().message("init").run_async().await.unwrap();
        repo.branch().create("feature").run_async().await.unwrap();
        let result: Result<(), CheckoutError> = repo.checkout().branch("feature").run_async().await;
        result.unwrap();
        assert_eq!(repo.branch().current_async().await.unwrap(), "feature");
        repo.checkout().new_branch("feature2").run_async().await.unwrap();
        assert_eq!(repo.branch().current_async().await.unwrap(), "feature2");
    }

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_checkout_nonexistent_fails() {
        let dir = get_temp_dir("sync_checkout_noexist");
        let repo = fluent_git::sync::git::init(&dir).run().unwrap().into_repo();
        configure_test_identity(&repo);
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().all().run().unwrap();
        repo.commit().message("init").run().unwrap();
        assert!(matches!(repo.checkout().branch("nope").run().unwrap_err(), CheckoutError::RefNotFound { .. }));
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_checkout_nonexistent_fails() {
        let dir = get_temp_dir("async_checkout_noexist");
        let repo = fluent_git::git::init(&dir).run().await.unwrap().into_repo();
        configure_test_identity_async(&repo).await;
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().all().run_async().await.unwrap();
        repo.commit().message("init").run_async().await.unwrap();
        assert!(matches!(
            repo.checkout().branch("nope").run_async().await.unwrap_err(),
            CheckoutError::RefNotFound { .. }
        ));
    }

    // ══════════════════════════════════════════════════════════════════════
    // Repo: Status
    // ══════════════════════════════════════════════════════════════════════

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_status() {
        let dir = get_temp_dir("sync_status");
        let repo = fluent_git::sync::git::init(&dir).run().unwrap().into_repo();
        configure_test_identity(&repo);
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().all().run().unwrap();
        repo.commit().message("init").run().unwrap();
        let status: Result<StatusResult, StatusError> = repo.status().run();
        assert!(status.unwrap().is_clean());

        fs::write(dir.join("h.txt"), "modified").unwrap();
        assert!(!repo.status().run().unwrap().is_clean());
        assert!(repo.status().run().unwrap().modified().contains(&"h.txt".into()));

        fs::write(dir.join("new.txt"), "new").unwrap();
        assert!(repo.status().run().unwrap().untracked.contains(&"new.txt".into()));
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_status() {
        let dir = get_temp_dir("async_status");
        let repo = fluent_git::git::init(&dir).run().await.unwrap().into_repo();
        configure_test_identity_async(&repo).await;
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().all().run_async().await.unwrap();
        repo.commit().message("init").run_async().await.unwrap();
        let status: Result<StatusResult, StatusError> = repo.status().run_async().await;
        assert!(status.unwrap().is_clean());

        fs::write(dir.join("h.txt"), "modified").unwrap();
        assert!(!repo.status().run_async().await.unwrap().is_clean());

        fs::write(dir.join("new.txt"), "new").unwrap();
        assert!(repo.status().run_async().await.unwrap().untracked.contains(&"new.txt".into()));
    }

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_status_short() {
        let dir = get_temp_dir("sync_status_short");
        let repo = fluent_git::sync::git::init(&dir).run().unwrap().into_repo();
        fs::write(dir.join("h.txt"), "h").unwrap();
        assert!(!repo.status().short().run().unwrap().is_clean());
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_status_short() {
        let dir = get_temp_dir("async_status_short");
        let repo = fluent_git::git::init(&dir).run().await.unwrap().into_repo();
        fs::write(dir.join("h.txt"), "h").unwrap();
        assert!(!repo.status().short().run_async().await.unwrap().is_clean());
    }

    // ══════════════════════════════════════════════════════════════════════
    // Repo: Log
    // ══════════════════════════════════════════════════════════════════════

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_log() {
        let dir = get_temp_dir("sync_log");
        let repo = fluent_git::sync::git::init(&dir).run().unwrap().into_repo();
        configure_test_identity(&repo);
        fs::write(dir.join("a.txt"), "a").unwrap();
        repo.add().all().run().unwrap();
        repo.commit().message("first").run().unwrap();
        fs::write(dir.join("b.txt"), "b").unwrap();
        repo.add().all().run().unwrap();
        repo.commit().message("second").run().unwrap();
        let commits: Result<Vec<LogEntry>, LogError> = repo.log().run();
        let commits = commits.unwrap();
        assert_eq!(commits.len(), 2);
        assert_eq!(commits[0].message, "second");
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_log() {
        let dir = get_temp_dir("async_log");
        let repo = fluent_git::git::init(&dir).run().await.unwrap().into_repo();
        configure_test_identity_async(&repo).await;
        fs::write(dir.join("a.txt"), "a").unwrap();
        repo.add().all().run_async().await.unwrap();
        repo.commit().message("first").run_async().await.unwrap();
        fs::write(dir.join("b.txt"), "b").unwrap();
        repo.add().all().run_async().await.unwrap();
        repo.commit().message("second").run_async().await.unwrap();
        let commits: Result<Vec<LogEntry>, LogError> = repo.log().run_async().await;
        let commits = commits.unwrap();
        assert_eq!(commits.len(), 2);
        assert_eq!(commits[0].message, "second");
    }

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_log_with_limit() {
        let dir = get_temp_dir("sync_log_limit");
        let repo = fluent_git::sync::git::init(&dir).run().unwrap().into_repo();
        configure_test_identity(&repo);
        for i in 0..5 {
            fs::write(dir.join(format!("{i}.txt")), format!("{i}")).unwrap();
            repo.add().all().run().unwrap();
            repo.commit().message(&format!("commit {i}")).run().unwrap();
        }
        assert_eq!(repo.log().max_count(2).run().unwrap().len(), 2);
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_log_with_limit() {
        let dir = get_temp_dir("async_log_limit");
        let repo = fluent_git::git::init(&dir).run().await.unwrap().into_repo();
        configure_test_identity_async(&repo).await;
        for i in 0..5 {
            fs::write(dir.join(format!("{i}.txt")), format!("{i}")).unwrap();
            repo.add().all().run_async().await.unwrap();
            repo.commit().message(&format!("commit {i}")).run_async().await.unwrap();
        }
        assert_eq!(repo.log().max_count(2).run_async().await.unwrap().len(), 2);
    }

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_log_on_empty_repo_fails() {
        let dir = get_temp_dir("sync_log_empty");
        let repo = fluent_git::sync::git::init(&dir).run().unwrap().into_repo();
        assert!(matches!(repo.log().run().unwrap_err(), LogError::NoCommits));
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_log_on_empty_repo_fails() {
        let dir = get_temp_dir("async_log_empty");
        let repo = fluent_git::git::init(&dir).run().await.unwrap().into_repo();
        assert!(matches!(repo.log().run_async().await.unwrap_err(), LogError::NoCommits));
    }

    // ══════════════════════════════════════════════════════════════════════
    // Repo: Diff
    // ══════════════════════════════════════════════════════════════════════

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_diff() {
        let dir = get_temp_dir("sync_diff");
        let repo = fluent_git::sync::git::init(&dir).run().unwrap().into_repo();
        configure_test_identity(&repo);
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().all().run().unwrap();
        repo.commit().message("init").run().unwrap();
        fs::write(dir.join("h.txt"), "mod").unwrap();
        let diff: Result<DiffResult, DiffError> = repo.diff().run();
        assert!(!diff.unwrap().is_empty());
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_diff() {
        let dir = get_temp_dir("async_diff");
        let repo = fluent_git::git::init(&dir).run().await.unwrap().into_repo();
        configure_test_identity_async(&repo).await;
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().all().run_async().await.unwrap();
        repo.commit().message("init").run_async().await.unwrap();
        fs::write(dir.join("h.txt"), "mod").unwrap();
        let diff: Result<DiffResult, DiffError> = repo.diff().run_async().await;
        assert!(!diff.unwrap().is_empty());
    }

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_diff_cached() {
        let dir = get_temp_dir("sync_diff_cached");
        let repo = fluent_git::sync::git::init(&dir).run().unwrap().into_repo();
        configure_test_identity(&repo);
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().all().run().unwrap();
        repo.commit().message("init").run().unwrap();
        fs::write(dir.join("h.txt"), "mod").unwrap();
        repo.add().all().run().unwrap();
        assert_eq!(repo.diff().cached().run().unwrap().files[0].path, "h.txt");
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_diff_cached() {
        let dir = get_temp_dir("async_diff_cached");
        let repo = fluent_git::git::init(&dir).run().await.unwrap().into_repo();
        configure_test_identity_async(&repo).await;
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().all().run_async().await.unwrap();
        repo.commit().message("init").run_async().await.unwrap();
        fs::write(dir.join("h.txt"), "mod").unwrap();
        repo.add().all().run_async().await.unwrap();
        assert_eq!(repo.diff().cached().run_async().await.unwrap().files[0].path, "h.txt");
    }

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_diff_between_refs() {
        let dir = get_temp_dir("sync_diff_refs");
        let repo = fluent_git::sync::git::init(&dir).initial_branch("main").run().unwrap().into_repo();
        configure_test_identity(&repo);
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().all().run().unwrap();
        repo.commit().message("first").run().unwrap();
        repo.checkout().new_branch("feature").run().unwrap();
        fs::write(dir.join("h.txt"), "changed").unwrap();
        repo.add().all().run().unwrap();
        repo.commit().message("second").run().unwrap();
        assert!(!repo.diff().between("main", "feature").run().unwrap().is_empty());
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_diff_between_refs() {
        let dir = get_temp_dir("async_diff_refs");
        let repo = fluent_git::git::init(&dir).initial_branch("main").run().await.unwrap().into_repo();
        configure_test_identity_async(&repo).await;
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().all().run_async().await.unwrap();
        repo.commit().message("first").run_async().await.unwrap();
        repo.checkout().new_branch("feature").run_async().await.unwrap();
        fs::write(dir.join("h.txt"), "changed").unwrap();
        repo.add().all().run_async().await.unwrap();
        repo.commit().message("second").run_async().await.unwrap();
        assert!(!repo.diff().between("main", "feature").run_async().await.unwrap().is_empty());
    }

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_diff_bad_ref_fails() {
        let dir = get_temp_dir("sync_diff_bad_ref");
        let repo = fluent_git::sync::git::init(&dir).run().unwrap().into_repo();
        configure_test_identity(&repo);
        fs::write(dir.join("a.txt"), "a").unwrap();
        repo.add().all().run().unwrap();
        repo.commit().message("init").run().unwrap();
        assert!(matches!(repo.diff().between("nope", "HEAD").run().unwrap_err(), DiffError::RefNotFound { .. }));
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_diff_bad_ref_fails() {
        let dir = get_temp_dir("async_diff_bad_ref");
        let repo = fluent_git::git::init(&dir).run().await.unwrap().into_repo();
        configure_test_identity_async(&repo).await;
        fs::write(dir.join("a.txt"), "a").unwrap();
        repo.add().all().run_async().await.unwrap();
        repo.commit().message("init").run_async().await.unwrap();
        assert!(matches!(
            repo.diff().between("nope", "HEAD").run_async().await.unwrap_err(),
            DiffError::RefNotFound { .. }
        ));
    }

    // ══════════════════════════════════════════════════════════════════════
    // Repo: Stash
    // ══════════════════════════════════════════════════════════════════════

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_stash_push_and_pop() {
        let dir = get_temp_dir("sync_stash");
        let repo = fluent_git::sync::git::init(&dir).run().unwrap().into_repo();
        configure_test_identity(&repo);
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().all().run().unwrap();
        repo.commit().message("init").run().unwrap();
        fs::write(dir.join("h.txt"), "mod").unwrap();
        repo.stash().push().run().unwrap();
        assert_eq!(fs::read_to_string(dir.join("h.txt")).unwrap(), "h");
        repo.stash().pop().run().unwrap();
        assert_eq!(fs::read_to_string(dir.join("h.txt")).unwrap(), "mod");
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_stash_push_and_pop() {
        let dir = get_temp_dir("async_stash");
        let repo = fluent_git::git::init(&dir).run().await.unwrap().into_repo();
        configure_test_identity_async(&repo).await;
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().all().run_async().await.unwrap();
        repo.commit().message("init").run_async().await.unwrap();
        fs::write(dir.join("h.txt"), "mod").unwrap();
        repo.stash().push().run_async().await.unwrap();
        assert_eq!(fs::read_to_string(dir.join("h.txt")).unwrap(), "h");
        repo.stash().pop().run_async().await.unwrap();
        assert_eq!(fs::read_to_string(dir.join("h.txt")).unwrap(), "mod");
    }

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_stash_list() {
        let dir = get_temp_dir("sync_stash_list");
        let repo = fluent_git::sync::git::init(&dir).run().unwrap().into_repo();
        configure_test_identity(&repo);
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().all().run().unwrap();
        repo.commit().message("init").run().unwrap();
        fs::write(dir.join("h.txt"), "c1").unwrap();
        repo.stash().push().message("first").run().unwrap();
        fs::write(dir.join("h.txt"), "c2").unwrap();
        repo.stash().push().message("second").run().unwrap();
        let stashes: Result<Vec<StashEntry>, StashError> = repo.stash().list().run();
        assert_eq!(stashes.unwrap().len(), 2);
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_stash_list() {
        let dir = get_temp_dir("async_stash_list");
        let repo = fluent_git::git::init(&dir).run().await.unwrap().into_repo();
        configure_test_identity_async(&repo).await;
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().all().run_async().await.unwrap();
        repo.commit().message("init").run_async().await.unwrap();
        fs::write(dir.join("h.txt"), "c1").unwrap();
        repo.stash().push().message("first").run_async().await.unwrap();
        fs::write(dir.join("h.txt"), "c2").unwrap();
        repo.stash().push().message("second").run_async().await.unwrap();
        let stashes: Result<Vec<StashEntry>, StashError> = repo.stash().list().run_async().await;
        assert_eq!(stashes.unwrap().len(), 2);
    }

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_stash_on_clean_fails() {
        let dir = get_temp_dir("sync_stash_clean");
        let repo = fluent_git::sync::git::init(&dir).run().unwrap().into_repo();
        configure_test_identity(&repo);
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().all().run().unwrap();
        repo.commit().message("init").run().unwrap();
        assert!(matches!(repo.stash().push().run().unwrap_err(), StashError::NothingToStash));
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_stash_on_clean_fails() {
        let dir = get_temp_dir("async_stash_clean");
        let repo = fluent_git::git::init(&dir).run().await.unwrap().into_repo();
        configure_test_identity_async(&repo).await;
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().all().run_async().await.unwrap();
        repo.commit().message("init").run_async().await.unwrap();
        assert!(matches!(repo.stash().push().run_async().await.unwrap_err(), StashError::NothingToStash));
    }

    // ══════════════════════════════════════════════════════════════════════
    // Repo: Remote, Tag, Reset, Merge, Rebase, Cherry-pick, Config, Clean, RevParse
    // ══════════════════════════════════════════════════════════════════════

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_remote_lifecycle() {
        let dir = get_temp_dir("sync_remote");
        let repo = fluent_git::sync::git::init(&dir).run().unwrap().into_repo();
        repo.remote().add("origin", "https://github.com/u/r.git").run().unwrap();
        assert!(repo.remote().list().run().unwrap().iter().any(|r| r.name == "origin"));
        assert!(matches!(
            repo.remote().add("origin", "https://b.com").run().unwrap_err(),
            RemoteError::AlreadyExists { .. }
        ));
        repo.remote().remove("origin").run().unwrap();
        assert!(matches!(repo.remote().remove("ghost").run().unwrap_err(), RemoteError::NotFound { .. }));
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_remote_lifecycle() {
        let dir = get_temp_dir("async_remote");
        let repo = fluent_git::git::init(&dir).run().await.unwrap().into_repo();
        repo.remote().add("origin", "https://github.com/u/r.git").run_async().await.unwrap();
        assert!(repo.remote().list().run_async().await.unwrap().iter().any(|r| r.name == "origin"));
        assert!(matches!(
            repo.remote().add("origin", "https://b.com").run_async().await.unwrap_err(),
            RemoteError::AlreadyExists { .. }
        ));
        repo.remote().remove("origin").run_async().await.unwrap();
        assert!(matches!(repo.remote().remove("ghost").run_async().await.unwrap_err(), RemoteError::NotFound { .. }));
    }

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_tag_lifecycle() {
        let dir = get_temp_dir("sync_tag");
        let repo = fluent_git::sync::git::init(&dir).run().unwrap().into_repo();
        configure_test_identity(&repo);
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().all().run().unwrap();
        repo.commit().message("init").run().unwrap();
        repo.tag().create("v0.1.0").run().unwrap();
        repo.tag().create("v0.2.0").message("annotated").run().unwrap();
        let tags = repo.tag().list().run().unwrap();
        assert!(tags.iter().any(|t| t.name == "v0.1.0" && !t.annotated));
        assert!(tags.iter().any(|t| t.name == "v0.2.0" && t.annotated));
        assert!(matches!(repo.tag().create("v0.1.0").run().unwrap_err(), TagError::AlreadyExists { .. }));
        repo.tag().delete("v0.1.0").run().unwrap();
        assert!(matches!(repo.tag().delete("v99").run().unwrap_err(), TagError::NotFound { .. }));
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_tag_lifecycle() {
        let dir = get_temp_dir("async_tag");
        let repo = fluent_git::git::init(&dir).run().await.unwrap().into_repo();
        configure_test_identity_async(&repo).await;
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().all().run_async().await.unwrap();
        repo.commit().message("init").run_async().await.unwrap();
        repo.tag().create("v0.1.0").run_async().await.unwrap();
        repo.tag().create("v0.2.0").message("annotated").run_async().await.unwrap();
        let tags = repo.tag().list().run_async().await.unwrap();
        assert!(tags.iter().any(|t| t.name == "v0.1.0" && !t.annotated));
        assert!(tags.iter().any(|t| t.name == "v0.2.0" && t.annotated));
        assert!(matches!(repo.tag().create("v0.1.0").run_async().await.unwrap_err(), TagError::AlreadyExists { .. }));
        repo.tag().delete("v0.1.0").run_async().await.unwrap();
        assert!(matches!(repo.tag().delete("v99").run_async().await.unwrap_err(), TagError::NotFound { .. }));
    }

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_reset() {
        let dir = get_temp_dir("sync_reset");
        let repo = fluent_git::sync::git::init(&dir).run().unwrap().into_repo();
        configure_test_identity(&repo);
        fs::write(dir.join("a.txt"), "a").unwrap();
        repo.add().all().run().unwrap();
        repo.commit().message("first").run().unwrap();
        fs::write(dir.join("b.txt"), "b").unwrap();
        repo.add().all().run().unwrap();
        repo.commit().message("second").run().unwrap();
        let r = repo.reset().soft().to("HEAD~1").run().unwrap();
        assert_eq!(r.mode, ResetMode::Soft);
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_reset() {
        let dir = get_temp_dir("async_reset");
        let repo = fluent_git::git::init(&dir).run().await.unwrap().into_repo();
        configure_test_identity_async(&repo).await;
        fs::write(dir.join("a.txt"), "a").unwrap();
        repo.add().all().run_async().await.unwrap();
        repo.commit().message("first").run_async().await.unwrap();
        fs::write(dir.join("b.txt"), "b").unwrap();
        repo.add().all().run_async().await.unwrap();
        repo.commit().message("second").run_async().await.unwrap();
        let r = repo.reset().soft().to("HEAD~1").run_async().await.unwrap();
        assert_eq!(r.mode, ResetMode::Soft);
    }

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_merge_no_ff() {
        let dir = get_temp_dir("sync_merge");
        let repo = fluent_git::sync::git::init(&dir).initial_branch("main").run().unwrap().into_repo();
        configure_test_identity(&repo);
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().all().run().unwrap();
        repo.commit().message("init").run().unwrap();
        repo.checkout().new_branch("feature").run().unwrap();
        fs::write(dir.join("f.txt"), "f").unwrap();
        repo.add().all().run().unwrap();
        repo.commit().message("feature").run().unwrap();
        repo.checkout().branch("main").run().unwrap();
        let result = repo.merge().branch("feature").no_ff().run().unwrap();
        assert!(!result.fast_forward);
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_merge_no_ff() {
        let dir = get_temp_dir("async_merge");
        let repo = fluent_git::git::init(&dir).initial_branch("main").run().await.unwrap().into_repo();
        configure_test_identity_async(&repo).await;
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().all().run_async().await.unwrap();
        repo.commit().message("init").run_async().await.unwrap();
        repo.checkout().new_branch("feature").run_async().await.unwrap();
        fs::write(dir.join("f.txt"), "f").unwrap();
        repo.add().all().run_async().await.unwrap();
        repo.commit().message("feature").run_async().await.unwrap();
        repo.checkout().branch("main").run_async().await.unwrap();
        let result = repo.merge().branch("feature").no_ff().run_async().await.unwrap();
        assert!(!result.fast_forward);
    }

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_merge_conflict() {
        let dir = get_temp_dir("sync_merge_conflict");
        let repo = fluent_git::sync::git::init(&dir).initial_branch("main").run().unwrap().into_repo();
        repo.config().set("user.name", "T").run().unwrap();
        repo.config().set("user.email", "t@t").run().unwrap();
        fs::write(dir.join("f.txt"), "original").unwrap();
        repo.add().all().run().unwrap();
        repo.commit().message("init").run().unwrap();
        repo.checkout().new_branch("a").run().unwrap();
        fs::write(dir.join("f.txt"), "from a").unwrap();
        repo.add().all().run().unwrap();
        repo.commit().message("a").run().unwrap();
        repo.checkout().branch("main").run().unwrap();
        fs::write(dir.join("f.txt"), "from main").unwrap();
        repo.add().all().run().unwrap();
        repo.commit().message("main").run().unwrap();
        let err: MergeError = repo.merge().branch("a").run().unwrap_err();
        assert!(matches!(err, MergeError::Conflict { .. }));
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_merge_conflict() {
        let dir = get_temp_dir("async_merge_conflict");
        let repo = fluent_git::git::init(&dir).initial_branch("main").run().await.unwrap().into_repo();
        repo.config().set("user.name", "T").run_async().await.unwrap();
        repo.config().set("user.email", "t@t").run_async().await.unwrap();
        fs::write(dir.join("f.txt"), "original").unwrap();
        repo.add().all().run_async().await.unwrap();
        repo.commit().message("init").run_async().await.unwrap();
        repo.checkout().new_branch("a").run_async().await.unwrap();
        fs::write(dir.join("f.txt"), "from a").unwrap();
        repo.add().all().run_async().await.unwrap();
        repo.commit().message("a").run_async().await.unwrap();
        repo.checkout().branch("main").run_async().await.unwrap();
        fs::write(dir.join("f.txt"), "from main").unwrap();
        repo.add().all().run_async().await.unwrap();
        repo.commit().message("main").run_async().await.unwrap();
        let err: MergeError = repo.merge().branch("a").run_async().await.unwrap_err();
        assert!(matches!(err, MergeError::Conflict { .. }));
    }

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_cherry_pick() {
        let dir = get_temp_dir("sync_cherry_pick");
        let repo = fluent_git::sync::git::init(&dir).initial_branch("main").run().unwrap().into_repo();
        configure_test_identity(&repo);
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().all().run().unwrap();
        repo.commit().message("init").run().unwrap();
        repo.checkout().new_branch("feature").run().unwrap();
        fs::write(dir.join("f.txt"), "cherry").unwrap();
        repo.add().all().run().unwrap();
        repo.commit().message("cherry this").run().unwrap();
        let sha = &repo.log().max_count(1).run().unwrap()[0].sha;
        repo.checkout().branch("main").run().unwrap();
        repo.cherry_pick().commit(sha).run().unwrap();
        assert!(dir.join("f.txt").exists());
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_cherry_pick() {
        let dir = get_temp_dir("async_cherry_pick");
        let repo = fluent_git::git::init(&dir).initial_branch("main").run().await.unwrap().into_repo();
        configure_test_identity_async(&repo).await;
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().all().run_async().await.unwrap();
        repo.commit().message("init").run_async().await.unwrap();
        repo.checkout().new_branch("feature").run_async().await.unwrap();
        fs::write(dir.join("f.txt"), "cherry").unwrap();
        repo.add().all().run_async().await.unwrap();
        repo.commit().message("cherry this").run_async().await.unwrap();
        let sha = &repo.log().max_count(1).run_async().await.unwrap()[0].sha;
        repo.checkout().branch("main").run_async().await.unwrap();
        repo.cherry_pick().commit(sha).run_async().await.unwrap();
        assert!(dir.join("f.txt").exists());
    }

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_config_set_get_unset() {
        let dir = get_temp_dir("sync_config");
        let repo = fluent_git::sync::git::init(&dir).run().unwrap().into_repo();
        repo.config().set("user.name", "Test").run().unwrap();
        assert_eq!(repo.config().get("user.name").run().unwrap(), "Test");
        repo.config().unset("user.name").run().unwrap();
        assert!(matches!(repo.config().get("user.name").run().unwrap_err(), ConfigError::KeyNotFound { .. }));
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_config_set_get_unset() {
        let dir = get_temp_dir("async_config");
        let repo = fluent_git::git::init(&dir).run().await.unwrap().into_repo();
        repo.config().set("user.name", "Test").run_async().await.unwrap();
        assert_eq!(repo.config().get("user.name").run_async().await.unwrap(), "Test");
        repo.config().unset("user.name").run_async().await.unwrap();
        assert!(matches!(
            repo.config().get("user.name").run_async().await.unwrap_err(),
            ConfigError::KeyNotFound { .. }
        ));
    }

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_clean() {
        let dir = get_temp_dir("sync_clean");
        let repo = fluent_git::sync::git::init(&dir).run().unwrap().into_repo();
        configure_test_identity(&repo);
        fs::write(dir.join("junk.txt"), "j").unwrap();
        assert!(matches!(repo.clean().run().unwrap_err(), CleanError::ForceRequired));
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().path("h.txt").run().unwrap();
        repo.commit().message("init").run().unwrap();
        let _result = repo.clean().force().directories().run().unwrap();
        assert!(!dir.join("junk.txt").exists());
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_clean() {
        let dir = get_temp_dir("async_clean");
        let repo = fluent_git::git::init(&dir).run().await.unwrap().into_repo();
        configure_test_identity_async(&repo).await;
        fs::write(dir.join("junk.txt"), "j").unwrap();
        assert!(matches!(repo.clean().run_async().await.unwrap_err(), CleanError::ForceRequired));
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().path("h.txt").run_async().await.unwrap();
        repo.commit().message("init").run_async().await.unwrap();
        let _result = repo.clean().force().directories().run_async().await.unwrap();
        assert!(!dir.join("junk.txt").exists());
    }

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_rev_parse() {
        let dir = get_temp_dir("sync_rev_parse");
        let repo = fluent_git::sync::git::init(&dir).run().unwrap().into_repo();
        configure_test_identity(&repo);
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().all().run().unwrap();
        repo.commit().message("init").run().unwrap();
        let sha: Result<String, RevParseError> = repo.rev_parse("HEAD").run();
        assert_eq!(sha.unwrap().len(), 40);
        assert!(matches!(repo.rev_parse("nonexistent").run().unwrap_err(), RevParseError::RefNotFound { .. }));
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_rev_parse() {
        let dir = get_temp_dir("async_rev_parse");
        let repo = fluent_git::git::init(&dir).run().await.unwrap().into_repo();
        configure_test_identity_async(&repo).await;
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().all().run_async().await.unwrap();
        repo.commit().message("init").run_async().await.unwrap();
        let sha: Result<String, RevParseError> = repo.rev_parse("HEAD").run_async().await;
        assert_eq!(sha.unwrap().len(), 40);
        assert!(matches!(
            repo.rev_parse("nonexistent").run_async().await.unwrap_err(),
            RevParseError::RefNotFound { .. }
        ));
    }

    // ══════════════════════════════════════════════════════════════════════
    // Repo: Worktree
    // ══════════════════════════════════════════════════════════════════════

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_worktree_lifecycle() {
        let dir = get_temp_dir("sync_wt");
        let repo = fluent_git::sync::git::init(&dir).initial_branch("main").run().unwrap().into_repo();
        configure_test_identity(&repo);
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().all().run().unwrap();
        repo.commit().message("init").run().unwrap();

        // Add — canonicalize paths since git returns canonical paths
        let wt_a = std::fs::canonicalize(get_temp_dir("sync_wt_a")).unwrap();
        let wt_b = std::fs::canonicalize(get_temp_dir("sync_wt_b")).unwrap();
        let result = repo.worktree().add(&wt_a, "feature-a").run().unwrap();
        assert!(result.created_branch);
        repo.worktree().add(&wt_b, "feature-b").run().unwrap();

        // Branch in use
        let wt_dup = get_temp_dir("sync_wt_dup");
        assert!(matches!(repo.worktree().add(&wt_dup, "main").run().unwrap_err(), WorktreeError::BranchInUse { .. }));

        // List & find
        let list = repo.worktree().list().run().unwrap();
        assert!(list.main.is_main);
        assert_eq!(list.linked().len(), 2);
        assert_eq!(list.find_by_branch("feature-a").unwrap().path, wt_a);
        assert_eq!(list.find_by_path(&wt_b).unwrap().branch, "feature-b");

        // Lock & unlock
        let lock = repo.worktree().lock(&wt_b).reason("external drive").run().unwrap();
        assert!(!lock.was_already_locked);
        assert!(repo.worktree().list().run().unwrap().find_by_branch("feature-b").unwrap().locked);
        repo.worktree().unlock(&wt_b).run().unwrap();

        // Move — target must not exist for git worktree move
        let wt_moved_parent = std::fs::canonicalize(get_temp_dir("sync_wt_moved_parent")).unwrap();
        let wt_moved = wt_moved_parent.join("wt_dest");
        let mv = repo.worktree().move_to(&wt_a, &wt_moved).run().unwrap();
        assert_eq!(mv.old_path, wt_a);
        assert_eq!(mv.new_path, wt_moved);

        // Remove
        repo.worktree().remove(&wt_moved).run().unwrap();
        repo.worktree().remove(&wt_b).run().unwrap();
        assert_eq!(repo.worktree().list().run().unwrap().linked().len(), 0);
        assert!(matches!(
            repo.worktree().remove(&PathBuf::from("/tmp/ghost")).run().unwrap_err(),
            WorktreeError::NotFound { .. }
        ));

        // Prune
        assert!(repo.worktree().prune().run().unwrap().pruned.is_empty());
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_worktree_lifecycle() {
        let dir = get_temp_dir("async_wt");
        let repo = fluent_git::git::init(&dir).initial_branch("main").run().await.unwrap().into_repo();
        configure_test_identity_async(&repo).await;
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().all().run_async().await.unwrap();
        repo.commit().message("init").run_async().await.unwrap();

        // Add — canonicalize paths since git returns canonical paths
        let wt_a = std::fs::canonicalize(get_temp_dir("async_wt_a")).unwrap();
        let wt_b = std::fs::canonicalize(get_temp_dir("async_wt_b")).unwrap();
        let result = repo.worktree().add(&wt_a, "feature-a").run_async().await.unwrap();
        assert!(result.created_branch);
        repo.worktree().add(&wt_b, "feature-b").run_async().await.unwrap();

        // Branch in use
        let wt_dup = get_temp_dir("async_wt_dup");
        assert!(matches!(
            repo.worktree().add(&wt_dup, "main").run_async().await.unwrap_err(),
            WorktreeError::BranchInUse { .. }
        ));

        // List & find
        let list = repo.worktree().list().run_async().await.unwrap();
        assert!(list.main.is_main);
        assert_eq!(list.linked().len(), 2);
        assert_eq!(list.find_by_branch("feature-a").unwrap().path, wt_a);
        assert_eq!(list.find_by_path(&wt_b).unwrap().branch, "feature-b");

        // Lock & unlock
        let lock = repo.worktree().lock(&wt_b).reason("external drive").run_async().await.unwrap();
        assert!(!lock.was_already_locked);
        assert!(repo.worktree().list().run_async().await.unwrap().find_by_branch("feature-b").unwrap().locked);
        repo.worktree().unlock(&wt_b).run_async().await.unwrap();

        // Move — target must not exist for git worktree move
        let wt_moved_parent = std::fs::canonicalize(get_temp_dir("async_wt_moved_parent")).unwrap();
        let wt_moved = wt_moved_parent.join("wt_dest");
        let mv = repo.worktree().move_to(&wt_a, &wt_moved).run_async().await.unwrap();
        assert_eq!(mv.old_path, wt_a);
        assert_eq!(mv.new_path, wt_moved);

        // Remove
        repo.worktree().remove(&wt_moved).run_async().await.unwrap();
        repo.worktree().remove(&wt_b).run_async().await.unwrap();
        assert_eq!(repo.worktree().list().run_async().await.unwrap().linked().len(), 0);
        assert!(matches!(
            repo.worktree().remove(&PathBuf::from("/tmp/ghost")).run_async().await.unwrap_err(),
            WorktreeError::NotFound { .. }
        ));

        // Prune
        assert!(repo.worktree().prune().run_async().await.unwrap().pruned.is_empty());
    }

    // ══════════════════════════════════════════════════════════════════════
    // GitError: umbrella for ? propagation
    // ══════════════════════════════════════════════════════════════════════

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_question_mark_propagation() {
        fn workflow(repo: &Repo) -> Result<CommitResult, GitError> {
            fs::write(repo.path().join("new.txt"), "data").unwrap();
            repo.add().all().run()?;
            repo.commit().message("add").run()?;
            repo.branch().create("feature").run()?;
            repo.checkout().branch("feature").run()?;
            fs::write(repo.path().join("f.txt"), "feature").unwrap();
            repo.add().all().run()?;
            let result = repo.commit().message("feature work").run()?;
            Ok(result)
        }

        let dir = get_temp_dir("sync_workflow");
        let repo = fluent_git::sync::git::init(&dir).run().unwrap().into_repo();
        configure_test_identity(&repo);
        assert_eq!(workflow(&repo).unwrap().message, "feature work");
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_question_mark_propagation() {
        async fn workflow(repo: &Repo) -> Result<CommitResult, GitError> {
            fs::write(repo.path().join("new.txt"), "data").unwrap();
            repo.add().all().run_async().await?;
            repo.commit().message("add").run_async().await?;
            repo.branch().create("feature").run_async().await?;
            repo.checkout().branch("feature").run_async().await?;
            fs::write(repo.path().join("f.txt"), "feature").unwrap();
            repo.add().all().run_async().await?;
            let result = repo.commit().message("feature work").run_async().await?;
            Ok(result)
        }

        let dir = get_temp_dir("async_workflow");
        let repo = fluent_git::git::init(&dir).run().await.unwrap().into_repo();
        configure_test_identity_async(&repo).await;
        assert_eq!(workflow(&repo).await.unwrap().message, "feature work");
    }

    #[cfg(feature = "blocking")]
    #[test]
    fn git_error_preserves_domain_info() {
        fn failing(repo: &Repo) -> Result<(), GitError> {
            repo.branch().delete("nonexistent").run()?;
            Ok(())
        }

        let dir = get_temp_dir("sync_git_error_domain");
        let repo = fluent_git::sync::git::init(&dir).run().unwrap().into_repo();
        configure_test_identity(&repo);
        fs::write(dir.join("h.txt"), "h").unwrap();
        repo.add().all().run().unwrap();
        repo.commit().message("init").run().unwrap();
        assert!(matches!(failing(&repo).unwrap_err(), GitError::Branch(BranchError::NotFound { .. })));
    }

    // ══════════════════════════════════════════════════════════════════════
    // Full workflows
    // ══════════════════════════════════════════════════════════════════════

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_full_workflow_init_to_merge() {
        let dir = get_temp_dir("sync_full");
        let repo = fluent_git::sync::git::init(&dir).initial_branch("main").run().unwrap().into_repo();
        repo.config().set("user.name", "Test").run().unwrap();
        repo.config().set("user.email", "t@t").run().unwrap();
        fs::write(dir.join("README.md"), "# Project").unwrap();
        repo.add().all().run().unwrap();
        assert_eq!(repo.commit().message("initial").run().unwrap().branch, "main");
        repo.checkout().new_branch("feature").run().unwrap();
        fs::write(dir.join("feature.rs"), "fn f() {}").unwrap();
        repo.add().all().run().unwrap();
        repo.commit().message("add feature").run().unwrap();
        repo.tag().create("v0.1.0-beta").message("beta").run().unwrap();
        repo.checkout().branch("main").run().unwrap();
        assert!(!repo.merge().branch("feature").no_ff().run().unwrap().fast_forward);
        repo.tag().create("v0.1.0").message("release").run().unwrap();
        assert!(dir.join("feature.rs").exists());
        assert_eq!(repo.branch().current().unwrap(), "main");
        assert!(repo.status().run().unwrap().is_clean());
        assert!(repo.log().run().unwrap().len() >= 3);
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_full_workflow_init_to_merge() {
        let dir = get_temp_dir("async_full");
        let repo = fluent_git::git::init(&dir).initial_branch("main").run().await.unwrap().into_repo();
        repo.config().set("user.name", "Test").run_async().await.unwrap();
        repo.config().set("user.email", "t@t").run_async().await.unwrap();
        fs::write(dir.join("README.md"), "# Project").unwrap();
        repo.add().all().run_async().await.unwrap();
        assert_eq!(repo.commit().message("initial").run_async().await.unwrap().branch, "main");
        repo.checkout().new_branch("feature").run_async().await.unwrap();
        fs::write(dir.join("feature.rs"), "fn f() {}").unwrap();
        repo.add().all().run_async().await.unwrap();
        repo.commit().message("add feature").run_async().await.unwrap();
        repo.tag().create("v0.1.0-beta").message("beta").run_async().await.unwrap();
        repo.checkout().branch("main").run_async().await.unwrap();
        assert!(!repo.merge().branch("feature").no_ff().run_async().await.unwrap().fast_forward);
        repo.tag().create("v0.1.0").message("release").run_async().await.unwrap();
        assert!(dir.join("feature.rs").exists());
        assert_eq!(repo.branch().current_async().await.unwrap(), "main");
        assert!(repo.status().run_async().await.unwrap().is_clean());
        assert!(repo.log().run_async().await.unwrap().len() >= 3);
    }

    #[cfg(feature = "blocking")]
    #[test]
    fn sync_full_workflow_clone_then_operate() {
        let origin = get_temp_dir("sync_full_clone_origin");
        let origin_repo = fluent_git::sync::git::init(&origin).initial_branch("main").run().unwrap().into_repo();
        origin_repo.config().set("user.name", "O").run().unwrap();
        origin_repo.config().set("user.email", "o@o").run().unwrap();
        fs::write(origin.join("README.md"), "# O").unwrap();
        origin_repo.add().all().run().unwrap();
        origin_repo.commit().message("init").run().unwrap();

        let dest = get_temp_dir("sync_full_clone_dest");
        let clone_result = fluent_git::sync::git::clone(&origin).into(&dest).run().unwrap();
        assert_eq!(clone_result.remote, "origin");
        let repo = clone_result.into_repo();
        repo.config().set("user.name", "C").run().unwrap();
        repo.config().set("user.email", "c@c").run().unwrap();
        fs::write(dest.join("f.rs"), "fn f() {}").unwrap();
        repo.add().all().run().unwrap();
        repo.commit().message("feature").run().unwrap();
        assert!(repo.remote().list().run().unwrap().iter().any(|r| r.name == "origin"));
    }

    #[cfg(feature = "tokio")]
    #[tokio::test]
    async fn async_full_workflow_clone_then_operate() {
        let origin = get_temp_dir("async_full_clone_origin");
        let origin_repo = fluent_git::git::init(&origin).initial_branch("main").run().await.unwrap().into_repo();
        origin_repo.config().set("user.name", "O").run_async().await.unwrap();
        origin_repo.config().set("user.email", "o@o").run_async().await.unwrap();
        fs::write(origin.join("README.md"), "# O").unwrap();
        origin_repo.add().all().run_async().await.unwrap();
        origin_repo.commit().message("init").run_async().await.unwrap();

        let dest = get_temp_dir("async_full_clone_dest");
        let clone_result = fluent_git::git::clone(&origin).into(&dest).run().await.unwrap();
        assert_eq!(clone_result.remote, "origin");
        let repo = clone_result.into_repo();
        repo.config().set("user.name", "C").run_async().await.unwrap();
        repo.config().set("user.email", "c@c").run_async().await.unwrap();
        fs::write(dest.join("f.rs"), "fn f() {}").unwrap();
        repo.add().all().run_async().await.unwrap();
        repo.commit().message("feature").run_async().await.unwrap();
        assert!(repo.remote().list().run_async().await.unwrap().iter().any(|r| r.name == "origin"));
    }

    // ══════════════════════════════════════════════════════════════════════
    // Error message validation
    // ══════════════════════════════════════════════════════════════════════

    #[test]
    fn all_error_messages_are_human_readable() {
        let errors: Vec<Box<dyn std::error::Error>> = vec![
            // Setup
            Box::new(SetupError::NotInstalled),
            Box::new(InitError::AlreadyExists { path: "/x".into() }),
            Box::new(InitError::PermissionDenied { path: "/x".into() }),
            Box::new(OpenError::NotARepo { path: "/x".into() }),
            Box::new(OpenError::CorruptRepo { path: "/x".into(), reason: "bad".into() }),
            Box::new(CloneError::AuthFailed { url: "x".into() }),
            Box::new(CloneError::Network { url: "x".into(), reason: "timeout".into() }),
            // Repo ops
            Box::new(AddError::PathNotFound { path: "x".into() }),
            Box::new(BranchError::NotFound { name: "x".into() }),
            Box::new(BranchError::DeleteCurrent { name: "x".into() }),
            Box::new(CheckoutError::RefNotFound { reference: "x".into() }),
            Box::new(CommitError::NothingToCommit),
            Box::new(CommitError::IdentityNotConfigured),
            Box::new(StatusError::IndexLocked),
            Box::new(LogError::NoCommits),
            Box::new(MergeError::Conflict { files: vec!["x".into()] }),
            Box::new(RebaseError::Conflict { files: vec!["x".into()] }),
            Box::new(CherryPickError::CommitNotFound { sha: "x".into() }),
            Box::new(RemoteError::PushRejected { name: "x".into(), reason: "y".into() }),
            Box::new(StashError::NothingToStash),
            Box::new(TagError::AlreadyExists { name: "x".into() }),
            Box::new(WorktreeError::BranchInUse { branch: "x".into(), path: "/y".into() }),
            Box::new(WorktreeError::CannotRemoveMain),
            Box::new(ConfigError::KeyNotFound { key: "x".into() }),
            Box::new(CleanError::ForceRequired),
            Box::new(ResetError::RefNotFound { reference: "x".into() }),
            Box::new(DiffError::InvalidRange { range: "x".into() }),
            Box::new(RevParseError::RefNotFound { reference: "x".into() }),
            Box::new(CommandError::Failed { args: "x".into(), code: 1, stdout: "".into(), stderr: "y".into() }),
            Box::new(CommandError::Timeout { args: "x".into(), timeout_secs: 30 }),
        ];

        for err in &errors {
            let msg = err.to_string();
            assert!(!msg.is_empty(), "empty message: {:?}", err);
        }
    }
}
