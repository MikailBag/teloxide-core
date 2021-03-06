use serde::Serialize;

use crate::{
    net,
    requests::{RequestOld, ResponseResult},
    types::{ChatId, InputFile, Message, ReplyMarkup},
    Bot,
};

/// As of [v.4.0], Telegram clients support rounded square mp4 videos of up to 1
/// minute long. Use this method to send video messages.
///
/// [The official docs](https://core.telegram.org/bots/api#sendvideonote).
///
/// [v.4.0]: https://telegram.org/blog/video-messages-and-telescope
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct SendVideoNote {
    #[serde(skip_serializing)]
    bot: Bot,
    pub chat_id: ChatId,
    pub video_note: InputFile,
    pub duration: Option<i32>,
    pub length: Option<i32>,
    pub thumb: Option<InputFile>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i32>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[async_trait::async_trait]
impl RequestOld for SendVideoNote {
    type Output = Message;

    async fn send(&self) -> ResponseResult<Message> {
        net::request_multipart(self.bot.client(), self.bot.token(), "sendVideoNote", self).await
    }
}

impl SendVideoNote {
    pub(crate) fn new<C>(bot: Bot, chat_id: C, video_note: InputFile) -> Self
    where
        C: Into<ChatId>,
    {
        Self {
            bot,
            chat_id: chat_id.into(),
            video_note,
            duration: None,
            length: None,
            thumb: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    /// Unique identifier for the target chat or username of the target channel
    /// (in the format `@channelusername`).
    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = val.into();
        self
    }

    /// Video note to send.
    ///
    /// Pass [`InputFile::File`] to send a file that exists on
    /// the Telegram servers (recommended), pass an [`InputFile::Url`] for
    /// Telegram to get a .webp file from the Internet, or upload a new one
    /// using [`InputFile::FileId`]. [More info on Sending Files »].
    ///
    /// [`InputFile::File`]: crate::types::InputFile::File
    /// [`InputFile::Url`]: crate::types::InputFile::Url
    /// [`InputFile::FileId`]: crate::types::InputFile::FileId
    /// [More info on Sending Files »]: https://core.telegram.org/bots/api#sending-files
    pub fn video_note(mut self, val: InputFile) -> Self {
        self.video_note = val;
        self
    }

    /// Duration of sent video in seconds.
    pub fn duration(mut self, val: i32) -> Self {
        self.duration = Some(val);
        self
    }

    /// Video width and height, i.e. diameter of the video message.
    pub fn length(mut self, val: i32) -> Self {
        self.length = Some(val);
        self
    }

    /// Thumbnail of the file sent; can be ignored if thumbnail generation for
    /// the file is supported server-side.
    ///
    /// The thumbnail should be in JPEG format and less than 200 kB in size. A
    /// thumbnail‘s width and height should not exceed 320. Ignored if the
    /// file is not uploaded using `multipart/form-data`. Thumbnails can’t
    /// be reused and can be only uploaded as a new file, so you can pass
    /// `attach://<file_attach_name>` if the thumbnail was uploaded using
    /// `multipart/form-data` under `<file_attach_name>`. [More info on
    /// Sending Files »].
    pub fn thumb(mut self, val: InputFile) -> Self {
        self.thumb = Some(val);
        self
    }

    /// Sends the message [silently]. Users will receive a notification with no
    /// sound.
    ///
    /// [silently]: https://telegram.org/blog/channels-2-0#silent-messages
    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }

    /// If the message is a reply, ID of the original message.
    pub fn reply_to_message_id(mut self, val: i32) -> Self {
        self.reply_to_message_id = Some(val);
        self
    }

    /// Additional interface options.
    ///
    /// A JSON-serialized object for an [inline keyboard], [custom reply
    /// keyboard], instructions to remove reply keyboard or to force a reply
    /// from the user.
    ///
    /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
    /// [custom reply keyboard]: https://core.telegram.org/bots#keyboards
    pub fn reply_markup(mut self, val: ReplyMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }
}
