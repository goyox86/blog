
pub mod api_v1;
pub mod pagination;

pub mod helpers {
    use rocket::http::Status;
    use rocket::Response;
    use rocket_contrib::{JSON, Value};
    use std::io::Cursor;

    pub fn empty_response_with_status<'r>(status: Status) -> Response<'r> {
        Response::build().status(status).finalize()
    }

    pub fn json_response_with_status<'r>(status: Status, json: Value) -> Response<'r> {
        let mut response = empty_response_with_status(status);
        response.set_sized_body(Cursor::new(JSON(json).to_string()));
        response
    }

    pub fn not_found_json_response<'r>() -> Response<'r> {
        json_response_with_status(Status::NotFound, json!({"status": "not_found"}))
    }

    pub fn ise_json_response<'r>() -> Response<'r> {
        json_response_with_status(Status::InternalServerError,
                                  json!({"status": "an internal error has occured"}))
    }

    pub fn ok_json_response<'r>(json: Value) -> Response<'r> {
        json_response_with_status(Status::Ok, json)
    }
}
