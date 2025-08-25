pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match security_level {
        Security::Unknown => {
            // Returns server URL or panics (no custom message)
            server.unwrap().to_string()
        }
        Security::Message => {
            // Returns server URL or panics with "ERROR: program stops"
            server.expect("ERROR: program stops").to_string()
        }
        Security::Warning => {
            // Returns server URL or "WARNING: check the server"
            server.unwrap_or("WARNING: check the server").to_string()
        }
        Security::NotFound => {
            // Returns server URL or "Not found: [MESSAGE]"
            match server {
                Ok(url) => url.to_string(),
                Err(err_msg) => format!("Not found: {}", err_msg),
            }
        }
        Security::UnexpectedUrl => {
            // Returns error message or panics with server URL as panic message
            match server {
                Ok(url) => panic!("{}", url),
                Err(err_msg) => err_msg.to_string(),
            }
        }
    }
}
