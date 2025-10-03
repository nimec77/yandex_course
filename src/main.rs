mod config;
mod utils;

use config::DEFAULT_COURSE_NAME;
use time::OffsetDateTime;
use utils::helpers::greet;

fn main() {
    greet();

    println!("Сегодня: {}", OffsetDateTime::now_utc().date());

    println!("Я прохожу курс: {}!", DEFAULT_COURSE_NAME);
}
