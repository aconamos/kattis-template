use crate::{Scaffold, backends::C, scaffold::GraphDir};

impl Scaffold for C {
    fn new_contest(&self, _contest_info: crate::ContestInfo) -> anyhow::Result<GraphDir> {
        todo!()
    }

    fn new_problem(&self, _problem_info: crate::ProblemInfo) -> anyhow::Result<GraphDir> {
        todo!()
    }
}
