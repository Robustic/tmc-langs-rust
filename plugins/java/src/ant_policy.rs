//! Ant student file policy

use std::path::{Path, PathBuf};
use tmc_langs_framework::policy::StudentFilePolicy;

pub struct AntStudentFilePolicy {
    config_file_parent_path: PathBuf,
}

impl AntStudentFilePolicy {
    pub fn new(config_file_parent_path: PathBuf) -> Self {
        Self {
            config_file_parent_path,
        }
    }
}

impl StudentFilePolicy for AntStudentFilePolicy {
    fn is_student_source_file(&self, path: &Path) -> bool {
        path.starts_with("src")
    }

    fn get_config_file_parent_path(&self) -> &Path {
        &self.config_file_parent_path
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_student_source_file() {
        let policy = AntStudentFilePolicy::new(PathBuf::from(""));
        assert!(policy.is_student_source_file(Path::new("src/file")));
        assert!(policy.is_student_source_file(Path::new("src/dir/file")));
    }

    #[test]
    fn is_not_student_source_file() {
        let policy = AntStudentFilePolicy::new(PathBuf::from(""));
        assert!(!policy.is_student_source_file(Path::new("file")));
        assert!(!policy.is_student_source_file(Path::new("dir/src/file")));
        assert!(!policy.is_student_source_file(Path::new("srca/file")));
    }
}
