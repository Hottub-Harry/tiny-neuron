use crate::id_manager::Id;

///    Here is a message. These are passed between neurons. The Message struct is likely going to be filled with analytics items later on.
///     Hence the decision for using a struct instead of a type alias
/// 
#[derive(Clone)]
pub struct Message
{
    pub author: Id,
    pub data:   u8,
}

impl Message 
{
    pub fn new(id: Id) -> Message 
    {
        Message 
        { 
            author: id, 
            data:   0
        }
    }
}

/// MessageBuffer stores the data between each layer. 
pub struct MessageBuffer
{
    pub messages: Vec<Message>,
}

impl MessageBuffer 
{
    pub fn new() -> MessageBuffer
    {
        MessageBuffer
        {
            messages: Vec::new()
        }
    } 
}