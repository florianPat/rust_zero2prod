use actix_web::HttpResponse;

pub async fn health() -> HttpResponse {
    HttpResponse::Ok().json("")
}

#[cfg(test)]
mod tests {
    use crate::infrastructure::actions::health::health;

    #[tokio::test]
    async fn health_check_succeeds() {
        let response = health().await;

        assert!(response.status().is_success());
        assert_eq!(
            response.headers().get("Content-Type").unwrap(),
            "application/json"
        );
    }
}
