use crate::core::requests::form_builder::FormBuilder;
use crate::core::requests::{ChatId, RequestContext, Request, RequestFuture, ResponseResult};
use crate::core::types::Message;
use crate::core::network;

#[derive(Debug, Clone)]
/// Use this method to forward messages of any kind. On success, the sent
/// [`Message`] is returned.
pub struct ForwardMessage<'a> {
    ctx: RequestContext<'a>,

    /// Unique identifier for the target chat or username of the target channel
    /// (in the format @channelusername)
    pub chat_id: ChatId,
    /// Unique identifier for the target chat or username of the target channel
    /// (in the format @channelusername)
    pub from_chat_id: ChatId,
    /// Message identifier in the chat specified in from_chat_id
    pub message_id: i64,

    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
}

impl<'a> Request<'a> for ForwardMessage<'a> {
    type ReturnValue = Message;

    fn send(self) -> RequestFuture<'a, ResponseResult<Self::ReturnValue>> {
        Box::pin(async move {
            let params = FormBuilder::new()
                .add("chat_id", &self.chat_id)
                .add("from_chat_id", &self.from_chat_id)
                .add("message_id", &self.message_id)
                .add_if_some(
                    "disable_notification",
                    self.disable_notification.as_ref()
                )
                .build();

            network::request(
                &self.ctx.client,
                &self.ctx.token,
                "forwardMessage",
                Some(params),
            ).await
        })
    }
}

impl<'a> ForwardMessage<'a> {
    pub(crate) fn new(ctx: RequestContext<'a>,
               chat_id: ChatId,
               from_chat_id: ChatId,
               message_id: i64) -> Self {
        Self {
            ctx,
            chat_id,
            from_chat_id,
            message_id,
            disable_notification: None
        }
    }

    pub fn chat_id<T: Into<ChatId>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }

    pub fn from_chat_id<T: Into<ChatId>>(mut self, val: T) -> Self {
        self.from_chat_id = val.into();
        self
    }

    pub fn message_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.message_id = val.into();
        self
    }

    pub fn disable_notification<T: Into<bool>>(mut self, val: T) -> Self {
        self.disable_notification = Some(val.into());
        self
    }
}