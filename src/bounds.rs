use crate::context::ParticleContext;
use crate::particle::Particle;

pub enum ParticleBoundsHandling {
    /// Lets particles move off the screen and disappear.
    None,
    /// Stops particles at the edge of the screen.
    Stop,
    /// Bounces particles off the walls.
    Bounce,
    /// Wraps particles around to the other side of the screen.
    Wrap,
}

impl ParticleBoundsHandling {
    pub fn handle(&self, particle: &mut Particle, context: &ParticleContext, vx: i32, vy: i32) {
        match self {
            ParticleBoundsHandling::Bounce => {
                if (particle.x as i32) + vx < 0 {
                    particle.x = 1; // bounce off the left wall
                } else if (particle.x as i32) + vx >= (context.width as i32) {
                    particle.x = context.width - 2; // bounce off the right wall
                } else {
                    particle.x = (particle.x as i32 + vx) as u16;
                }
                if (particle.y as i32) + vy < 0 {
                    particle.y = 1; // bounce off the top wall
                } else if (particle.y as i32) + vy >= (context.height as i32) {
                    particle.y = context.height - 2; // bounce off the bottom wall
                } else {
                    particle.y = (particle.y as i32 + vy) as u16;
                }
            }
            ParticleBoundsHandling::Wrap => {
                if (particle.x as i32) + vx < 0 {
                    particle.x = context.width - 1; // wrap around to the right wall
                } else if (particle.x as i32) + vx >= (context.width as i32) {
                    particle.x = 0; // wrap around to the left wall
                } else {
                    particle.x = (particle.x as i32 + vx) as u16; // not on the edge
                }

                if (particle.y as i32) + vy < 0 {
                    particle.y = context.height - 1; // wrap around to the bottom wall
                } else if (particle.y as i32) + vy >= (context.height as i32) {
                    particle.y = 0; // wrap around to the top wall
                } else {
                    particle.y = (particle.y as i32 + vy) as u16; // not on the edge
                }
            }
            ParticleBoundsHandling::Stop => {
                if (particle.x as i32) + vx < 0 {
                    particle.x = 0;
                    particle.vx = 0;
                } else if (particle.x as i32) + vx >= (context.width as i32) {
                    particle.x = context.width - 1;
                    particle.vx = 0;
                } else {
                    particle.x = (particle.x as i32 + vx) as u16;
                }

                if (particle.y as i32) + vy < 0 {
                    particle.y = 0;
                    particle.vy = 0;
                } else if (particle.y as i32) + vy >= (context.height as i32) {
                    particle.y = context.height - 1;
                    particle.vy = 0;
                } else {
                    particle.y = (particle.y as i32 + vy) as u16;
                }
            }
            ParticleBoundsHandling::None => {
                particle.x = (particle.x as i32 + vx) as u16;
                particle.y = (particle.y as i32 + vy) as u16;
            }
        }
    }
}
