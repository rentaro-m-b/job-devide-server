use actix_web::{HttpRequest, HttpResponse, Responder};

use crate::{response::update_user_response::UpdateUserResponse, usecase::llm_usecase::{self, LlmUsecase}};

pub struct LlmController {
    llm_usecase: LlmUsecase
}

impl LlmController {
    pub fn new(llm_usecase: LlmUsecase) -> Self {
        LlmController{ llm_usecase }
    }

    pub async fn doc_generate(&self, req: HttpRequest) -> impl Responder {
        let message = self.llm_usecase.generate("".to_owned()).await;
        HttpResponse::Ok().json(UpdateUserResponse{ message: message })
    }
}
