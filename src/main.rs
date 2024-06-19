use config::ensure_data_dir_exists;

mod config;

fn main() {
    ensure_data_dir_exists();
}
