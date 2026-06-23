use crate::{Scaffold, backends::Rust};

impl Scaffold for Rust {
    fn new_contest(
        &self,
        _contest_info: crate::ContestInfo,
        _path: std::path::PathBuf,
    ) -> anyhow::Result<()> {
        todo!()
    }

    fn new_problem(
        &self,
        _problem_info: crate::ProblemInfo,
        _path: std::path::PathBuf,
    ) -> anyhow::Result<()> {
        todo!()
    }
}
