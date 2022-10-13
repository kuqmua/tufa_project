#[derive(Debug, PartialEq, Clone)]
pub struct Rotate {
    degrees: u16,
}

impl Rotate {
    pub fn new(possible_degrees: u16) -> Result<Self, u16> {
        if possible_degrees > 360 {
            return Err(possible_degrees);
        }
        Ok(Self {
            degrees: possible_degrees,
        })
    }
    pub fn change(mut self, possible_degrees: u16) -> Result<Self, Self> {
        if possible_degrees > 360 {
            return Err(self);
        }
        self.degrees = possible_degrees;
        Ok(self)
    }
    pub fn get_degrees(&self) -> u16 {
        self.degrees
    }
}
