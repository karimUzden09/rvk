use super::*;

/// <https://vk.com/dev/objects/comment>
#[derive(Deserialize, Clone, Debug)]
pub struct Comment {
    pub id: Integer,
    pub from_id: Integer,
    pub date: Integer,
    pub text: String,
    pub reply_to_user: Option<Integer>,
    pub reply_to_comment: Option<Integer>,
    pub attachments: Option<Vec<attachment::WallAttachment>>,
    pub parents_stack: Option<Vec<Integer>>,
    pub thread: Option<Thread>,
    pub deleted: Option<Boolean>,
    pub likes: Option<LikesShort>,
    // For attachment::MessageAttachment
    pub post_id: Option<Integer>,
    pub owner_id: Option<Integer>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Thread {
    pub count: Integer,
    pub items: Option<Vec<Comment>>,
    pub can_post: Boolean,
    pub show_reply_button: Boolean,
    pub groups_can_post: Boolean,
}

#[derive(Deserialize, Clone, Debug)]
pub struct LikesShort {
    pub can_like: i64,
    pub count: i64,
    pub user_likes: i64,
    pub can_publish: i64,
}
