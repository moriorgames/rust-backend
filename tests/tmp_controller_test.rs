use backend::bounded_context;
use actix_web::{http::header::ContentType, test, App};
use actix_web::http::StatusCode;

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_web::test]
    async fn test_tmp_get() {
        let tmp = bounded_context::infrastructure::http::tmp_controller::tmp;
        let app_entry = App::new().service(tmp);
        let app = test::init_service(app_entry).await;
        let req = test::TestRequest::get().uri("/tmp").to_request();
        let resp = test::call_service(&app, req).await;
        
        assert_eq!(resp.status(), StatusCode::OK);
    
        let body = resp.into_body();
    
        let bytes = actix_web::body::to_bytes(body).await.unwrap();
        let body_str = std::str::from_utf8(&bytes).unwrap();
        
        assert_eq!(body_str, "Print some value: mysql://root:root@localhost:3306/rust");
    }    
}
