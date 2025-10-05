mod config;
mod utils;

use config::CourseConfig;
use time::OffsetDateTime;
use utils::helpers::{greet, show_progress};

fn main() {

    let course = CourseConfig::default();

    greet();

    println!("Сегодня: {}", OffsetDateTime::now_utc().date());

    println!("Я прохожу курс: {}", course);
    
    println!("Мой прогресс в текущем модуле:");
    show_progress(9, 14);
}
