use crate::request::prelude::*;
use twilight_model::{id::ChannelId, invite::Invite};

/// Get the invites for a guild channel.
///
/// This method only works if the channel is of type `GuildChannel`.
pub struct GetChannelInvites<'a> {
    channel_id: ChannelId,
    fut: Option<Pending<'a, Vec<Invite>>>,
    http: &'a Client,
}

impl<'a> GetChannelInvites<'a> {
    pub(crate) fn new(http: &'a Client, channel_id: ChannelId) -> Self {
        Self {
            channel_id,
            fut: None,
            http,
        }
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from(
            Route::GetChannelInvites {
                channel_id: self.channel_id.0,
            },
        ))));

        Ok(())
    }
}

poll_req!(GetChannelInvites<'_>, Vec<Invite>);
