mod posts;

pub use self::connector::*;

#[get("/")]
pub fn index() -> &'static str {
    "
        Botcket

            This project is an experiment made to test both Rocket, a web frameword
            and Diesel, an ORM (used with postgres here)
    "
}