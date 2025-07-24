
#[derive(Default)]
pub struct Apple {
    position: (u16, u16),
}

impl Apple {

    pub fn spawn(max_coords: (u16, u16)) -> Self {
        Apple {
            position: (rand::random_range(0..max_coords.0), rand::random_range(0..max_coords.1)),
        }
    }

    pub fn position(&self) -> (u16, u16) {
        self.position
    }

}
