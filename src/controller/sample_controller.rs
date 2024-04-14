use actix_web::{HttpResponse};

pub struct SampleController {}

impl SampleController {
    pub async fn sample() -> HttpResponse {
        HttpResponse::Ok().body("This is a sample response")
    }
}
