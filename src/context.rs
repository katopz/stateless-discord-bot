use serde::Deserialize;

use std::collections::HashMap;

use crate::discord::interaction::Interaction;
use crate::discord::verification::verify_signature;
use crate::error::Error;
use crate::http::{HttpError, HttpRequest, HttpResponse};

#[derive(Deserialize)]
pub(crate) struct Context {
    pub(crate) env: HashMap<String, String>,
    pub(crate) request: HttpRequest,
}

impl Context {
    fn env(&self, key: &str) -> Result<&String, Error> {
        self.env
            .get(key)
            .ok_or_else(|| Error::EnvironmentVariableNotFound(key.to_string()))
    }

    fn perform_verification(&self) -> Result<(), Error> {
        let public_key = self.env("PUBLIC_KEY")?;
        let signature = self.request.header("x-signature-ed25519")?;
        let timestamp = self.request.header("x-signature-timestamp")?;

        verify_signature(public_key, signature, timestamp, &self.request.body)
            .map_err(Error::VerificationFailed)
    }

    async fn handle_payload(&self) -> Result<String, Error> {
        let payload = &self.request.body;
        let interaction =
            serde_json::from_str::<Interaction>(payload).map_err(Error::JsonFailed)?;
        let response = interaction.perform().await;

        serde_json::to_string(&response.unwrap()).map_err(Error::JsonFailed)
    }

    pub(crate) async fn handle_http_request(&self) -> HttpResponse {
        // TOFIX: handle invalid payload
        let _verified_result = self.perform_verification().map_err(HttpError::from);
        let result = self.handle_payload().await.map_err(HttpError::from);

        match result {
            Ok(body) => HttpResponse { status: 200, body },
            Err(error) => HttpResponse {
                body: error.to_string(),
                status: error.status as u16,
            },
        }
    }
}
