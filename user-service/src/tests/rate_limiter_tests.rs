use actix_governor::{Governor, GovernorConfigBuilder, KeyExtractor};
use actix_web::{App, dev::ServiceRequest, Error, test, web};
use futures::future::{ready, Ready};
use serial_test::serial;

use crate::config::database;
use crate::middleware::rate_limiter::configure_rate_limiter;
use crate::routes::user_routes;

#[derive(Clone)]
pub struct FixedKeyExtractor;

impl KeyExtractor for FixedKeyExtractor {
  type Key = String;
  type KeyExtractionError = std::convert::Infallible;

  fn extract(&self, _req: &ServiceRequest) -> Result<Self::Key, Self::KeyExtractionError> {
    Ok("fixed_key".to_string())
  }
}

#[actix_rt::test]
#[serial]
async fn test_rate_limiting() {
  let pool = database::establish_connection();

  let mut app = test::init_service(App::new()
    .app_data(web::Data::new(pool.clone()))
    .wrap(configure_rate_limiter())
    .service(user_routes())
  ).await;

  // Envia 10 requisições, que devem ser permitidas
  for _ in 0..10 {
    let req = test::TestRequest::get().uri("/users").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), actix_web::http::StatusCode::OK);
  }

  // A 11ª requisição deve ser limitada
  let req = test::TestRequest::get().uri("/users").to_request();
  let resp = test::call_service(&mut app, req).await;
  assert_eq!(resp.status(), actix_web::http::StatusCode::TOO_MANY_REQUESTS);
}
