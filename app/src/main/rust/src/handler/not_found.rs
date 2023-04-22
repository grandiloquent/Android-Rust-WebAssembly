use rocket::http::Status;
use rocket::catch;

#[catch(404)]
pub fn not_found() -> Status {
    Status::NotFound
}

