use rand::Rng;
#[derive(Debug, Clone)]
pub struct Dice {
    pub sides: usize,
}
impl Dice {
    pub fn new(sides: usize) -> Self {
        Dice { sides }
    }

    pub fn roll(&self) -> usize {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=self.sides)
    }
}
