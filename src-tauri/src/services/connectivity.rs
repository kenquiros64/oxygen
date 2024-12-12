use std::sync::Arc;
use crate::models::error::ErrorResponse as Error;
use reqwest;
use std::time::Duration;
use crate::enums::error::ErrorType;

pub async fn is_internet_available() -> Result<(), Error> {
    let addr = "8.8.8.8".parse().unwrap();
    let data = [1,2,3,4];  // ping data
    let data_arc = Arc::new(&data[..]);
    let timeout = Duration::from_secs(1);
    let options = ping_rs::PingOptions { ttl: 128, dont_fragment: true };
    let future = ping_rs::send_ping_async(&addr, timeout, data_arc, Some(&options));

    let result = futures::executor::block_on(future);

    match result {
        Ok(_) => {
            println!("Ping successful!");
            return Ok(());
        }
        Err(e) => {
            println!("Ping failed: {:?}", e);
        }
    }

    // Step 2: Fallback to HTTP Check
    let url = "https://www.google.com";
    match reqwest::get(url).await {
        Ok(response) if response.status().is_success() => {
            println!("HTTP check successful!");
            Ok(())
        }
        Ok(_) => Err(
            Error::new(
                ErrorType::UnreachableInternet,
                "Verificar conexión a internet".to_string().as_str()
            )
        ),
        Err(_) => Err(
            Error::new(
                ErrorType::NoInternetConnection,
                "Sin conexión a internet".to_string().as_str()
            )
        ),
    }
}