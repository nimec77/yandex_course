use std::fmt::{self};

pub const DEFAULT_COURSE_NAME: &str = "Rust для действующих разработчиков";

#[derive(Default)]
pub enum CourseCohort {
    #[default]
    Start,
    Base,
    Blockchain,
}

impl fmt::Display for CourseCohort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            CourseCohort::Start => "Переход с C/С++/Python",
            CourseCohort::Base => "Базовый курс",
            CourseCohort::Blockchain => "Погружение в блокчейн",
        };
        write!(f, "{}", name)
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

impl fmt::Display for CourseConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{DEFAULT_COURSE_NAME}: {}", self.cohort)
    }
}
