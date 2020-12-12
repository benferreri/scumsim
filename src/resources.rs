#[derive(Default)]
pub struct Night(u8);

impl Night {
    pub fn new(n: u8) -> Self {
        Night(n)
    }
    pub fn get(&self) -> u8 {
        self.0
    }
}

#[derive(Default)]
pub struct CurrentNight(pub Night);
