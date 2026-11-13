pub struct LlmClient {
    pub endpoint: String,
}

impl LlmClient {
    pub fn new(endpoint: &str) -> Self {
        Self {
            endpoint: endpoint.to_string(),
        }
    }

    pub fn health_url(&self) -> String {
        format!("{}/health", self.endpoint)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_url() {
        let client = LlmClient::new("http://127.0.0.1:8080");
        assert_eq!(client.health_url(), "http://127.0.0.1:8080/health");
    }
}
