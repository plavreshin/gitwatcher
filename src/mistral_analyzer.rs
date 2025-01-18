use mistralai_client::v1::{
    chat::{ChatMessage, ChatMessageRole, ChatParams},
    client::Client,
    constants::Model,
};

pub struct MistralAnalyzer {
    client: Client,
}

impl MistralAnalyzer {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let client = Client::new(None, None, None, None)?;
        Ok(Self { client })
    }

    pub async fn analyze_changes(
        &self,
        changes: Vec<String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let model = Model::CodestralLatest;
        let prompt = format!(
            "Analyze these Git changes and provide a brief summary: {:?}",
            changes
        );

        let messages = vec![ChatMessage {
            role: ChatMessageRole::User,
            content: prompt,
            tool_calls: None,
        }];

        let options = ChatParams {
            temperature: 0.7,
            ..Default::default()
        };

        let result = self
            .client
            .chat_async(model, messages, Some(options))
            .await?;
        Ok(result.choices[0].message.content.clone())
    }
}
