use serde::{Deserialize, Serialize};

use crate::project::Project;

#[derive(Serialize, Deserialize)]
pub struct Board {
    projects: Vec<Project>,
}

impl Default for Board {
    fn default() -> Self {
        Self::load_or_create()
    }
}

impl Board {
    pub fn load_or_create() -> Self {
        Self { projects: vec![] }
    }

    pub fn add_new_project(&mut self, name: &str) {
        self.projects.push(Project::with_name(name));
    }

    pub fn get_project_names(&self) -> Vec<String> {
        self.projects.iter().map(|p| p.name.clone()).collect()
    }
}
