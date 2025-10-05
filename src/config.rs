use std::fmt::{self, Display};

pub const DEFAULT_COURSE_NAME: &str = "Rust для действующих разработчиков";

#[derive(Default)]
pub enum CourseCohort {
    #[default]
    Start,
    Base,
    Blockchain,
}

impl Display for CourseCohort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CourseCohort::Start => write!(f, "Start"),
            CourseCohort::Base => write!(f, "Base"),
            CourseCohort::Blockchain => write!(f, "Blockchain"),
        }
    }
}

#[derive(Default)]
pub struct CourseConfig {
    pub cohort: CourseCohort,
}

impl CourseConfig {
    pub fn new(cohort: CourseCohort) -> Self {
        Self { cohort }
    }

    pub fn duration(&self) -> u8 {
        match self.cohort {
            CourseCohort::Start => 16,
            CourseCohort::Base => 12,
            CourseCohort::Blockchain => 20,
        }
    }

    pub fn upgrade(&mut self) -> bool {
        match self.cohort {
            CourseCohort::Blockchain => false,
            _ => {
                self.cohort = CourseCohort::Blockchain;

                true
            }
        }
    }
}

