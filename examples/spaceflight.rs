use crossterm::{cursor, terminal, ExecutableCommand};

use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;

use rand::Rng;

use particlez::{
    bounds::ParticleBoundsHandling, context::ParticleContext, movement::ParticleMovement,
    particle::Particle,
};

fn main() {
    let context = ParticleContext {
        width: 180,
        height: 34,
    };
    const NUM_PARTICLES: usize = 100;
    let mut particles = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..NUM_PARTICLES {
        let mut p = Particle::new_random(0, context.width, 0, context.height);

        p.vx = rng.gen_range(1..4); // Random velocity between 1 and 3
        particles.push(p);
    }

    loop {
        stdout()
            .execute(terminal::Clear(terminal::ClearType::All))
            .unwrap();
        for particle in &particles {

            let character = if particle.vx > 2 {
                "*"
            } else if particle.vx > 1 {
                "."
            } else {
                "."
            };

            stdout()
                .execute(cursor::MoveTo(particle.x, particle.y))
                .unwrap()
                .write(character.as_bytes())
                .unwrap();
        }

        // Put back the cursor at the bottom of the terminal
        stdout().execute(cursor::MoveTo(0, context.height)).unwrap();

        stdout().flush().unwrap();

        for particle in &mut particles {
            ParticleMovement::ConstantVelocity(ParticleBoundsHandling::Wrap)
                .update(particle, &context);
        }

        // 60 frames per second
        thread::sleep(Duration::from_millis(1000 / 60));
    }
}
