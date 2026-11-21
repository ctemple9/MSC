//! Essential deletion-boundary checks. All inputs are in-memory, with no
//! services, real directories, network calls, sleeps, or destructive execution.
use msc_infrastructure::fs::FakeFileSystem;
use msc_infrastructure::service::{ServiceInstallRequest, ServiceStatusReport};
use msc_infrastructure::uninstall::{
    self, EntryKind, EntryState, InstallerCandidate, Overrides, Platform, Request,
};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

fn request() -> Request {
    Request {
        computer: "fixture-host".into(),
        platform: Platform::Linux,
        home: PathBuf::from("/home/player"),
        overrides: Overrides::default(),
        services: vec![ServiceStatusReport::not_installed("com.ctemple.msc2.agent")],
        protected_paths: vec![PathBuf::from("/work/msc2")],
        client_paths: vec![],
        headless_roots: vec![],
        installer_candidates: vec![],
        findings: vec![],
    }
}

#[test]
fn rejects_shared_roots_traversal_source_trees_and_symlink_targets() {
    let fs = FakeFileSystem::new()
        .with_symlink("/home/player/.local/share/msc2", "/home/player/Documents")
        .with_dir("/work/other/.git");
    let req = request();
    for path in [
        "/",
        "/home/player",
        "/home/player/Documents",
        "/home/player/Downloads",
        "/home/player/.local/share",
        "/home/player/a/../Documents",
        "/work/msc2",
        "/work/msc2/crates",
        "/home/player/.local/share/msc2",
        "/work/other",
        "relative/server",
    ] {
        assert!(
            uninstall::validate_target(&fs, &req, Path::new(path)).is_err(),
            "accepted {path}"
        );
    }
    assert!(uninstall::validate_target(&fs, &req, Path::new("/home/player/servers/paper")).is_ok());
}

#[test]
fn corrupt_config_blocks_its_parent_data_deletion() {
    let fs = FakeFileSystem::new()
        .with_dir("/home/player/.local/share/msc2")
        .with_file(
            "/home/player/.local/share/msc2/server_config_swift.json",
            b"{broken".to_vec(),
            false,
        );
    let result = uninstall::inventory(&fs, &request());
    assert!(result.is_blocked());
    assert!(
        result
            .entries
            .iter()
            .any(|entry| entry.kind == EntryKind::DataTree
                && entry.path.as_deref() == Some(Path::new("/home/player/.local/share/msc2"))
                && entry.state == EntryState::Blocked)
    );
}

#[test]
fn macos_default_data_override_without_engine_config_does_not_block_uninstall() {
    let root = "/home/player/Library/Application Support/MSC 2";
    let fs = FakeFileSystem::new().with_dir(root);
    let mut req = request();
    req.platform = Platform::Macos;
    req.overrides.data_dir = Some(PathBuf::from(root));
    let result = uninstall::inventory(&fs, &req);
    let entries: Vec<_> = result
        .entries
        .iter()
        .filter(|entry| entry.path.as_deref() == Some(Path::new(root)))
        .collect();
    assert!(!entries.is_empty());
    assert!(
        entries
            .iter()
            .all(|entry| entry.state != EntryState::Blocked)
    );
}

#[test]
fn custom_missing_config_does_not_authorize_unrelated_directory_deletion() {
    let fs = FakeFileSystem::new().with_dir("/home/player/other");
    let mut req = request();
    req.overrides.data_dir = Some(PathBuf::from("/home/player/other"));
    let result = uninstall::inventory(&fs, &req);
    assert!(result.entries.iter().any(|entry| entry.path.as_deref()
        == Some(Path::new("/home/player/other"))
        && entry.state == EntryState::Blocked));
}

#[test]
fn uses_local_service_overrides_without_disclosing_other_environment_values() {
    let fs = FakeFileSystem::new();
    let mut req = request();
    let definition = ServiceInstallRequest::new(
        "com.ctemple.msc2.agent",
        "/opt/msc2/msc",
        "/opt/msc2/data",
        "/opt/msc2/logs/agent.log",
        48001,
    )
    .env("MSC2_DATA_DIR", "/srv/msc/data")
    .env("MSC2_APP_CONFIG_PATH", "/srv/msc/config.json")
    .env("MSC2_AGENT_SERVERS_ROOT", "/srv/msc/servers")
    .env("PASSWORD", "private-fixture-value");
    req.services = vec![ServiceStatusReport::stopped(definition)];
    let result = uninstall::inventory(&fs, &req);
    let encoded = serde_json::to_string(&result).unwrap();
    assert!(encoded.contains("/srv/msc/data"));
    assert!(encoded.contains("/srv/msc/config.json"));
    assert!(!encoded.contains("private-fixture-value"));
    let overrides = Overrides::from_environment(&BTreeMap::from([
        ("MSC2_DATA_DIR".into(), "/srv/msc/data".into()),
        ("SECRET".into(), "private-fixture-value".into()),
    ]));
    assert_eq!(
        overrides.data_dir.as_deref(),
        Some(Path::new("/srv/msc/data"))
    );
}

#[test]
fn installer_filename_is_not_ownership_and_changed_checksum_is_blocked() {
    let fs = FakeFileSystem::new().with_file(
        "/home/player/Downloads/MSC_2.dmg",
        b"unrelated-file".to_vec(),
        false,
    );
    for checksum in [None, Some("0".repeat(64))] {
        let mut req = request();
        req.installer_candidates.push(InstallerCandidate {
            path: PathBuf::from("/home/player/Downloads/MSC_2.dmg"),
            verified_sha256: checksum,
        });
        let result = uninstall::inventory(&fs, &req);
        assert!(
            result
                .entries
                .iter()
                .any(|entry| entry.kind == EntryKind::Installer
                    && entry.state == EntryState::Blocked)
        );
    }
}

#[test]
fn preview_has_no_remote_targets_and_does_not_modify_input_files() {
    let fs = FakeFileSystem::new().with_file(
        "/home/player/.local/share/msc2/keep",
        b"data".to_vec(),
        false,
    );
    let result = uninstall::inventory(&fs, &request());
    assert!(
        result
            .entries
            .iter()
            .filter_map(|entry| entry.path.as_ref())
            .all(|path| path.is_absolute())
    );
    assert_eq!(
        msc_infrastructure::fs::FileSystem::read(
            &fs,
            Path::new("/home/player/.local/share/msc2/keep")
        )
        .unwrap(),
        b"data"
    );
    assert!(!result.fingerprint.is_empty());
}

#[test]
fn uninstall_choices_remove_the_app_and_respect_kept_or_confirmed_uncertain_data() {
    use msc_infrastructure::uninstall::native::{self, Installation, LocalServices, Preview};
    use std::cell::RefCell;
    struct Operations(RefCell<Vec<PathBuf>>, RefCell<Vec<Installation>>);
    impl LocalServices for Operations {
        fn inspect(&self) -> Result<Vec<ServiceStatusReport>, String> {
            Ok(vec![ServiceStatusReport::not_installed(
                "com.ctemple.msc2.agent",
            )])
        }
        fn remove(&self) -> Result<(), String> {
            Ok(())
        }
        fn clear_credentials(&self) -> Result<(), String> {
            Ok(())
        }
        fn remove_data(&self, path: &Path) -> Result<(), String> {
            self.0.borrow_mut().push(path.into());
            Ok(())
        }
        fn remove_installation(&self, installation: &Installation) -> Result<(), String> {
            self.1.borrow_mut().push(installation.clone());
            Ok(())
        }
    }
    let root = PathBuf::from("/home/player/uncertain-msc-data");
    for keep in [false, true] {
        let fs = FakeFileSystem::new().with_dir(root.clone());
        let mut req = request();
        req.overrides.data_dir = Some(root.clone());
        let inventory = uninstall::inventory(&fs, &req);
        assert!(inventory.is_blocked());
        let app = Installation::MacBundle {
            path: PathBuf::from("/Applications/MSC fixture.app"),
        };
        let preview = Preview {
            inventory,
            request: req,
            installations: vec![app.clone()],
            engine_data_dir: root.clone(),
            engine_running: false,
        };
        let operations = Operations(RefCell::new(vec![]), RefCell::new(vec![]));
        let selected = vec![root.clone()];
        let (remove_anyway, keep_paths) = if keep {
            (&[][..], selected.as_slice())
        } else {
            (selected.as_slice(), &[][..])
        };
        let result = native::remove_confirmed(
            &operations,
            preview,
            uninstall::CONFIRMATION,
            remove_anyway,
            keep_paths,
        )
        .unwrap();
        assert_eq!(*operations.1.borrow(), vec![app]);
        assert_eq!(operations.0.borrow().contains(&root), !keep);
        assert_eq!(result.state, "uninstalled");
    }
}
