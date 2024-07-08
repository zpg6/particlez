use rand::Rng;

#[derive(Clone, Copy)]
pub struct Particle {
    pub x: u16,
    pub y: u16,
    pub vx: i32,
    pub vy: i32,
}

impl Particle {
    pub fn new_random(min_x: u16, max_x: u16, min_y: u16, max_y: u16) -> Self {
        let mut rng = rand::thread_rng();

        Particle {
            x: rng.gen_range(min_x..max_x),
            y: rng.gen_range(min_y..max_y),
            vx: 0,
            vy: 0,
        }
    }
}
