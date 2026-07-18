use rig::{
    client::CompletionClient,
    completion::{AssistantContent, CompletionModel},
    providers::openai::Client,
};

mod error;

pub use error::AgentError;

pub struct Agent {
    client: Client,
}

impl Agent {
    pub fn new(api_key: &str) -> Result<Self, AgentError> {
        let client = Client::new(api_key).map_err(|_| AgentError::Config)?;
        Ok(Self { client })
    }

    pub async fn suggestion(&self, forecasts: String) -> Result<String, AgentError> {
        let model = self.client.completion_model("gpt-5-nano");
        let response = model
            .completion_request(
                "Recommend what I should wear today based on the supplied weather conditions. Do not ask any follow up question.",
            )
            .preamble(forecasts)
            .send()
            .await
            .map_err(|err| AgentError::OpenAI(err.to_string()))?;
        let text = response
            .choice
            .into_iter()
            .filter_map(|content| match content {
                AssistantContent::Text(text) => Some(text.text),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("\n");

        if text.is_empty() {
            return Err(AgentError::NoTextResponse);
        }

        Ok(text)
    }
}
