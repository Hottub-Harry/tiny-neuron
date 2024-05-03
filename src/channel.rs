use crate::message::Message;

// I use a u128 for a channelId, type removes ambiguity
pub type ChannelId = u128;
pub struct Channel 
{
    id: ChannelId,
    message_queue: Vec<Message>,

}

impl Channel {

    fn get_message( &self ) -> Option<Message> {
        
    }
}