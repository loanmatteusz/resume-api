use actix_governor::{Governor, GovernorConfigBuilder, KeyExtractor};
use actix_governor::governor::middleware::NoOpMiddleware;
use actix_web::dev::ServiceRequest;

#[derive(Clone)]
pub struct FixedKeyExtractor;

impl KeyExtractor for FixedKeyExtractor {
    type Key = String;
    type KeyExtractionError = std::convert::Infallible; // Ajuste aqui para um erro infalível

    fn extract(&self, _req: &ServiceRequest) -> Result<Self::Key, Self::KeyExtractionError> {
        Ok("fixed_key".to_string())
    }
}

pub fn configure_rate_limiter() -> Governor<FixedKeyExtractor, NoOpMiddleware> {
    // Configuração do rate limiter
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(10) // 10 requisições por segundo
        .burst_size(10) // Permitir "burst" de até 10 requisições
        .key_extractor(FixedKeyExtractor)
        .finish()
        .unwrap();

    Governor::new(&governor_conf)
}
