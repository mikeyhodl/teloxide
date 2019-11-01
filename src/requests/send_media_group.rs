use async_trait::async_trait;

use crate::{
    bot::Bot,
    network::request_multipart,
    requests::{form_builder::FormBuilder, Request, ResponseResult},
    types::{ChatId, InputFile, InputMedia, Message},
};

/// Use this method to send a group of photos or videos as an album.
#[derive(Debug, Clone)]
pub struct SendMediaGroup<'a> {
    bot: &'a Bot,

    pub chat_id: ChatId,
    pub media: Vec<InputMedia>,

    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i32>,
}

#[async_trait]
impl Request for SendMediaGroup<'_> {
    type Output = Vec<Message>;

    async fn send_boxed(self) -> ResponseResult<Self::Output> {
        self.send().await
    }
}

impl SendMediaGroup<'_> {
    pub async fn send(self) -> ResponseResult<Vec<Message>> {
        let form = FormBuilder::new()
            .add("chat_id", self.chat_id)
            .add("media", &self.media[..])
            .add("disable_notification", self.disable_notification)
            .add("reply_to_message_id", self.reply_to_message_id);

        let form = self
            .media
            .into_iter()
            .filter_map(|e| InputFile::from(e).into())
            .fold(form, |acc, path: std::path::PathBuf| {
                acc.add_file(
                    &path.file_name().unwrap().to_string_lossy().into_owned(),
                    path,
                )
            });

        request_multipart(
            self.bot.client(),
            self.bot.token(),
            "sendMediaGroup",
            form.build(),
        )
        .await
    }
}

impl<'a> SendMediaGroup<'a> {
    pub(crate) fn new<C, M>(bot: &'a Bot, chat_id: C, media: M) -> Self
    where
        C: Into<ChatId>,
        M: Into<Vec<InputMedia>>,
    {
        SendMediaGroup {
            bot,
            chat_id: chat_id.into(),
            media: media.into(),
            disable_notification: None,
            reply_to_message_id: None,
        }
    }

    pub fn chat_id<T>(mut self, value: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = value.into();
        self
    }

    pub fn media<T>(mut self, value: T) -> Self
    where
        T: Into<Vec<InputMedia>>,
    {
        self.media = value.into();
        self
    }

    pub fn disable_notification<T>(mut self, value: T) -> Self
    where
        T: Into<bool>,
    {
        self.disable_notification = Some(value.into());
        self
    }

    pub fn reply_to_message_id<T>(mut self, value: T) -> Self
    where
        T: Into<i32>,
    {
        self.reply_to_message_id = Some(value.into());
        self
    }
}
