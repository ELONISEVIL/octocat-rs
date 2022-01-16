use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;

use github_rest::model::Commit;
use octocat_rs::{handler::EventHandler, Client, ClientBuilder, Command};

#[tokio::main]
async fn main() -> Result<()> {
    #[derive(Debug)]
    struct Handler {}

    #[async_trait]
    impl EventHandler for Handler {
        type Message = ();
        type GitHubClient = Client<Self>;

        fn webhook_port(&self) -> u16 {
            2022
        }

        async fn commit_pushed(
            &self,
            _github_client: Arc<Self::GitHubClient>,
            _commit: Commit,
        ) -> Command<Self::Message> {
            println!("Commit pushed!");
            Command::none()
        }
    }

    Ok(ClientBuilder::new().event_handler(Handler {}).build()?.start().await)
}
