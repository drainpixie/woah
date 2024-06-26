use directories::ProjectDirs;
use once_cell::sync::Lazy;

pub static PROJECT_DIRS: Lazy<ProjectDirs> =
    Lazy::new(|| ProjectDirs::from("com", "drainpixie", "woah").unwrap());

pub fn create_data_directory() {
    let data_dir = PROJECT_DIRS.data_dir();
    if !data_dir.exists() {
        std::fs::create_dir_all(data_dir).unwrap();
    }
}
