use crate::bounds::ParticleBoundsHandling;
use crate::context::ParticleContext;
use crate::particle::Particle;
use rand::Rng;

pub enum ParticleMovement {
    None,
    ConstantVelocity(ParticleBoundsHandling),
    Gravity(ParticleBoundsHandling),
    RandomSomeDirection(ParticleBoundsHandling),
}

impl ParticleMovement {
    pub fn update(&self, particle: &mut Particle, context: &ParticleContext) {
        let mut rng = rand::thread_rng();
        match self {
            ParticleMovement::None => {}
            ParticleMovement::ConstantVelocity(bounds_handling) => {
                bounds_handling.handle(particle, context, particle.vx, particle.vy);
            }
            ParticleMovement::Gravity(bounds_handling) => {
                // Adjust the velocity to simulate gravity
                particle.vy += 1;
                bounds_handling.handle(particle, context, particle.vx, particle.vy);
            }
            ParticleMovement::RandomSomeDirection(bounds_handling) => {
                // Move 1 step in a random direction
                particle.vx = rng.gen_range(-1..2);
                particle.vy = rng.gen_range(-1..2);

                bounds_handling.handle(particle, context, particle.vx, particle.vy);
            }
        }
    }
}
