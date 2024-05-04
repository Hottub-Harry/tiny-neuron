//! 2 Rolling buffers used between layers of neurons
//! Each buffer is a nested vector of bytes. After a layer computes, and writes to the output buffer, the buffers swap.
//! After the swap the output buffer is cleared. 
//!
//! SIGNIFIGANT: EACH NEURON IN LAYER READS FROM THE INDEX POS IT SITS AT
//! 
//! For example the first neuron in the layer, reads index 0 in the buffer, and so on.
//! 
pub type Buffer = Vec<Vec<u8>>;

pub struct BufferManager
{
    pub input_buffer:   Buffer,
    pub output_buffer:  Buffer,
    pub should_swap:    bool  ,
}

impl BufferManager 
{
    pub fn new() -> BufferManager
    {
        BufferManager
        {
            input_buffer :  Buffer::new(),
            output_buffer:  Buffer::new(),
            should_swap  :  false        ,
        }
    }

    pub fn swap(&mut self) 
    {
        if self.should_swap
        {
            self.input_buffer = self.output_buffer.clone();
            self.output_buffer                    .clear();
        }
    }
}