use crate::request::prelude::*;
use std::borrow::Cow;
use twilight_model::id::{ChannelId, MessageId};

/// Delete a message by [`ChannelId`] and [`MessageId`].
///
/// [`ChannelId`]: ../../../../twilight_model/id/struct.ChannelId.html
/// [`MessageId`]: ../../../../twilight_model/id/struct.MessageId.html
pub struct DeleteMessage<'a> {
    channel_id: ChannelId,
    fut: Option<Pending<'a, ()>>,
    http: &'a Client,
    message_id: MessageId,
    reason: Option<Cow<'a, str>>,
}

impl<'a> DeleteMessage<'a> {
    pub(crate) fn new(http: &'a Client, channel_id: ChannelId, message_id: MessageId) -> Self {
        Self {
            channel_id,
            fut: None,
            http,
            message_id,
            reason: None,
        }
    }

    /// Attach an audit log reason to this request.
    pub fn reason(mut self, reason: impl Into<Cow<'a, str>>) -> Self {
        self.reason.replace(reason.into());

        self
    }

    fn start(&mut self) -> Result<()> {
        let request = if let Some(reason) = self.reason.as_ref() {
            let headers = audit_header(&reason)?;
            Request::from((
                headers,
                Route::DeleteMessage {
                    channel_id: self.channel_id.0,
                    message_id: self.message_id.0,
                },
            ))
        } else {
            Request::from(Route::DeleteMessage {
                channel_id: self.channel_id.0,
                message_id: self.message_id.0,
            })
        };

        self.fut.replace(Box::pin(self.http.verify(request)));

        Ok(())
    }
}

poll_req!(DeleteMessage<'_>, ());