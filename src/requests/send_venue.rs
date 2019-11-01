use async_trait::async_trait;

use crate::{
    bot::Bot,
    network,
    requests::{Request, ResponseResult},
    types::{ChatId, Message, ReplyMarkup},
};

/// Use this method to send information about a venue.
/// Message is returned.
#[derive(Debug, Clone, Serialize)]
pub struct SendVenue<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or
    /// username of the target channel (in the format @channelusername)
    pub chat_id: ChatId,
    /// Latitude of the venue
    pub latitude: f64,
    /// Longitude of the venue
    pub longitude: f64,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Foursquare identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue, if known. (For
    /// example, “arts_entertainment/default”, “arts_entertainment/aquarium” or
    /// “food/icecream”.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
    /// Sends the message silently. Users will receive a
    /// notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original
    /// message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i32>,
    ///	InlineKeyboardMarkup or ReplyKeyboardMarkup or ReplyKeyboardRemove or
    /// ForceReply 	Optional 	Additional interface options. A JSON-serialized
    /// object for an inline keyboard, custom reply keyboard, instructions to
    /// remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[async_trait]
impl Request for SendVenue<'_> {
    type Output = Message;

    async fn send_boxed(self) -> ResponseResult<Self::Output> {
        self.send().await
    }
}

impl SendVenue<'_> {
    pub async fn send(self) -> ResponseResult<Message> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "sendVenue",
            &self,
        )
        .await
    }
}

impl<'a> SendVenue<'a> {
    pub(crate) fn new<Lt, Lg, C, T, A>(
        bot: &'a Bot,
        chat_id: C,
        latitude: Lt,
        longitude: Lg,
        title: T,
        address: A,
    ) -> Self
    where
        Lt: Into<f64>,
        Lg: Into<f64>,
        C: Into<ChatId>,
        T: Into<String>,
        A: Into<String>,
    {
        Self {
            bot,
            chat_id: chat_id.into(),
            latitude: latitude.into(),
            longitude: longitude.into(),
            title: title.into(),
            address: address.into(),
            foursquare_id: None,
            foursquare_type: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }
    pub fn chat_id<T>(mut self, value: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = value.into();
        self
    }

    pub fn longitude<Lg>(mut self, value: Lg) -> Self
    where
        Lg: Into<f64>,
    {
        self.longitude = value.into();
        self
    }

    pub fn latitude<Lt>(mut self, value: Lt) -> Self
    where
        Lt: Into<f64>,
    {
        self.latitude = value.into();
        self
    }

    pub fn title<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.title = value.into();
        self
    }

    pub fn address<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.address = value.into();
        self
    }

    pub fn foursquare_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.foursquare_id = Some(value.into());
        self
    }

    pub fn disable_notification<T>(mut self, value: T) -> Self
    where
        T: Into<bool>,
    {
        self.disable_notification = Some(value.into());
        self
    }

    pub fn foursquare_type<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.foursquare_type = Some(value.into());
        self
    }

    pub fn reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<ReplyMarkup>,
    {
        self.reply_markup = Some(value.into());
        self
    }
}
