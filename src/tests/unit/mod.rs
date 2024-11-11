use crate::generators::{backend, docker, frontend};
use tempfile::tempdir;

#[test]
fn test_backend_generator() {
    let temp_dir = tempdir().unwrap();
    let project_name = "test_project";

    backend::create_structure(&temp_dir.path().join(project_name).to_str().unwrap());

    // Vérifier la structure backend
    assert!(temp_dir
        .path()
        .join(project_name)
        .join("backend/Cargo.toml")
        .exists());
    assert!(temp_dir
        .path()
        .join(project_name)
        .join("backend/src/main.rs")
        .exists());
}
