use backend::bounded_context;

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::header::ContentType, test, App};
    use actix_web::http::StatusCode;

    #[actix_web::test]
    async fn test_index_get() {
        let index = bounded_context::infrastructure::http::index_controller::index;
        let app_entry = App::new()
                .service(index);
        let app = test::init_service(app_entry).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = test::call_service(&app, req).await;    
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
