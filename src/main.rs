// Ejemplo de uso de la API de Numverify para validar números de teléfono

use reqwest::{Error, Response};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
struct PhoneNumberInfo {
    valid: bool,
    number: String,
    local_format: String,
    international_format: String,
    country_prefix: String,
    country_code: String,
    country_name: String,
    // location: String,
    carrier: String,
    line_type: String,
}

async fn fetch_phone_number_info(
    api_key: &str,
    phone_number: &str,
    country_code: &str,
    format: &str,
) -> Result<PhoneNumberInfo, Box<dyn std::error::Error>> {
    let url = format!(
        "http://apilayer.net/api/validate?access_key={}&number={}&country_code={}&format={}",
        api_key, phone_number, country_code, format
    );

    let response = reqwest::get(&url).await?;
    
    if !response.status().is_success() {
        return Err(format!("HTTP Error: {:?}", response.status()).into());
    }

    let json: Value = response.json().await?;
    
    let info: PhoneNumberInfo = match serde_json::from_value(json) {
        Ok(info) => info,
        Err(_) => {
            // Manejar el caso en el que el campo `valid` no está presente en la respuesta JSON
            PhoneNumberInfo {
                valid: false,
                number: String::new(),
                local_format: String::new(),
                international_format: String::new(),
                country_prefix: String::new(),
                country_code: String::new(),
                country_name: String::new(),
                // location: String::new(),
                carrier: String::new(),
                line_type: String::new(),
            }
        }
    };
    
    Ok(info)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = "API Key";
    let phone_number = "Number"; // Ex 787
    let country_code = "Code"; // Ex US
    let format = "1";
    
    match fetch_phone_number_info(api_key, phone_number, country_code, format).await {
        Ok(info) => {
            if info.valid {
                println!("Valid Phone Number {}", info.number);
                println!("Country: {}", info.country_name);
                // println!("Ubicación: {}", info.location);
                println!("Carrier: {}", info.carrier);
                println!("Line Type: {}", info.line_type);
            } else {
                println!("Invalid Phone Number");
            }
        }
        Err(err) => {
            eprintln!("Error: {:?}", err);
        }
    }
  
    Ok(())
}
