use crate::{Scaffold, backends::PythonUv};

impl Scaffold for PythonUv {
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
