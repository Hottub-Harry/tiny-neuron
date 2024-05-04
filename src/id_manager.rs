//! Internal manager for issuing Id's to neurons. 
//! Could be a hectic process if done another way, depending on the size of the model
pub type Id = u128;

pub struct IdManager 
{
    current_id: Id,
}

impl IdManager 
{
    pub fn new() -> IdManager
    {
        IdManager { current_id: 0 }
    }

    pub fn get_id(&mut self) -> Id
    {
        self.current_id += 1;
        return self.current_id;
    }
}

