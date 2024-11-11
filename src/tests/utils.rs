use std::path::Path;

pub fn cleanup_test_dir(path: &Path) {
    if path.exists() {
        std::fs::remove_dir_all(path).unwrap();
    }
}

pub fn assert_project_structure(project_path: &Path) {
    // Backend
    assert!(project_path.join("backend/Cargo.toml").exists());
    assert!(project_path.join("backend/src/main.rs").exists());
    assert!(project_path.join("backend/src/graphql").exists());
    assert!(project_path.join("backend/src/db").exists());

    // Frontend
    assert!(project_path.join("frontend/package.json").exists());

    // Docker
    assert!(project_path.join("docker-compose.yml").exists());
}
