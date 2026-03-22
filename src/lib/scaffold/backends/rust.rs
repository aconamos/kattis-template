use crate::{Scaffold, backends::Rust};

impl Scaffold for Rust {
    fn new_contest(
        &self,
        contest_info: crate::ContestInfo,
        path: std::path::PathBuf,
    ) -> anyhow::Result<()> {
        todo!()
    }

    fn new_problem(
        &self,
        problem_info: crate::ProblemInfo,
        path: std::path::PathBuf,
    ) -> anyhow::Result<()> {
        todo!()
    }
}
