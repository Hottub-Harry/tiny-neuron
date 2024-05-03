use crate::channel::{Channel, ChannelId};

struct Neuron
{
    id: u128,
    weight: f64,
    input_data: Vec<u8>,
    output_data: Vec<u8>,
    pub_channels: Vec<ChannelId>, 
    sub_channels: Vec<ChannelId>
}