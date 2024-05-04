use crate::message::{Message, MessageBuffer};
use crate::id_manager::{Id, IdManager};

struct Neuron
{
    id:             Id,
    weight:         f32,
    input_data:     Vec<u8>,
    output_data:    Message,
    subscribed_to:  Vec<Id>,
}

impl Neuron 
{
    fn new(id_mnger: &mut IdManager) -> Neuron
    {
        let new_id = id_mnger.get_id();
        Neuron 
        { 
            id:             new_id, 
            weight:         0.0, 
            input_data:     Vec::new(), 
            output_data:    Message::new(new_id), 
            subscribed_to:  Vec::new()
        }
    }

    fn set_input_data(&mut self, msg_buffer: MessageBuffer) 
    {
        //construct the input data for the neuron based on what is in the buffer
        for msg in msg_buffer.messages.iter() 
        {
            //quick and dirty way of doing this. It will be a memory hog storing all these copies
            // will have to optimize for memory in the future
            for _id in self.subscribed_to.iter()
            {
                if *_id == msg.author
                {
                    self.input_data.push(msg.data.clone());
                }
            }
        }
    }

    fn send_output_data(&self, msg_buffer: &mut MessageBuffer)
    {
        msg_buffer.messages.push(self.output_data.clone());
    }

    fn subscribe(&mut self, id : Id)
    {
        self.subscribed_to.push(id);
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn two_neuron_msg_test()
    {
        let mut id_mngr = IdManager::new();

        let mut msg_buffer = MessageBuffer::new();

        let mut neuron1 = Neuron::new(&mut id_mngr); //id  1
        let mut neuron2 = Neuron::new(&mut id_mngr); //id  2

        neuron2.subscribe(1);
        neuron1.output_data.data = 0;
        neuron1.send_output_data(&mut msg_buffer);

        neuron2.set_input_data(msg_buffer);

        //hacky way to test data is the same   
        //output data that is written to the buffer, cloned out of the buffer and into the first pos of the neuron
        assert_eq!(neuron1.output_data.data, *neuron2.input_data.get(0).unwrap());
    }
}