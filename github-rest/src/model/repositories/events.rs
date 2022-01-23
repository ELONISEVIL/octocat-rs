use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum::{EnumString, EnumVariantNames};

use crate::{
    methods::util,
    model::{
        commits::comments::CommitComment,
        repositories::{
            events::nested::{Commit, HeadCommit, Pusher},
            Repository,
        },
        user::User,
    },
    GithubRestError, Requester,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PushEvent {
    #[serde(rename = "ref")]
    pub ref_field: String,
    pub before: String,
    pub after: String,
    pub repository: Repository,
    pub pusher: Pusher,
    pub sender: User,
    pub created: bool,
    pub deleted: bool,
    pub forced: bool,
    pub base_ref: Value,
    pub compare: String,
    pub commits: Vec<Commit>,
    pub head_commit: HeadCommit,
}

impl PushEvent {
    /// Adds a comment to the commit that triggered the event.
    ///
    /// See also: <https://docs.github.com/en/rest/reference/commits#create-a-commit-comment>
    pub async fn add_comment_to_commit(
        &self,
        client: Arc<&impl Requester>,
        body: String,
        path: Option<String>,
        position: Option<String>,
    ) -> Result<CommitComment, GithubRestError> {
        util::helper_for_helper_for_helper(
            *client,
            self.head_commit.url.clone(),
            self.head_commit.id.clone(),
            body,
            path,
            position,
        )
        .await
    }
}

pub mod nested {
    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    use crate::model::user::SimpleUser;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Pusher {
        pub name: String,
        pub email: String,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Commit {
        pub id: String,
        pub tree_id: String,
        pub distinct: bool,
        pub message: String,
        pub timestamp: String,
        pub url: String,
        pub author: SimpleUser,
        pub committer: SimpleUser,
        pub added: Vec<String>,
        pub removed: Vec<Value>,
        pub modified: Vec<Value>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct HeadCommit {
        pub id: String,
        pub tree_id: String,
        pub distinct: bool,
        pub message: String,
        pub timestamp: String,
        pub url: String,
        pub author: SimpleUser,
        pub committer: SimpleUser,
        pub added: Vec<String>,
        pub removed: Vec<Value>,
        pub modified: Vec<Value>,
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StarEvent {
    pub action: StarAction,
    pub repository: Repository,
    pub sender: User,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum StarAction {
    Created,
    Deleted,
}

// TODO: Watch event
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForkEvent {
    forkee: Repository,
    repository: Repository,
    sender: User,
}
