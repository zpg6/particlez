use crossterm::{cursor, terminal, ExecutableCommand};
use rand::Rng;
use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;

use particlez::{
    bounds::ParticleBoundsHandling, context::ParticleContext, movement::ParticleMovement,
    particle::Particle,
};

fn main() {
    // Start with a reasonable fixed size for the terminal window animation
    let context = ParticleContext {
        width: 180,
        height: 34,
    };

    // Create a vector of particles, each with a random position and velocity
    const NUM_PARTICLES: usize = 100;
    let mut particles = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..NUM_PARTICLES {
        // Generate a new random particle with x between 0 and width, and y between 0 and height
        let mut p = Particle::new_random(0, context.width, 0, context.height);

        // Randomize the velocity of the bug between 1 and 4
        p.vx = rng.gen_range(1..2);
        particles.push(p);
    }

    // Continuously update the particles with `RandomSomeDirection` movement and render them to the terminal
    loop {
        for particle in &mut particles {
            ParticleMovement::RandomSomeDirection(ParticleBoundsHandling::Wrap)
                .update(particle, &context);
        }

        // Clear the terminal so everything can be re-rendered
        stdout()
            .execute(terminal::Clear(terminal::ClearType::All))
            .unwrap();

        // Render each bug, updating the direction of the bug based on its velocity
        for particle in &particles {
            stdout()
                .execute(cursor::MoveTo(particle.x, particle.y))
                .unwrap()
                .write(if particle.vx > 0 { ">" } else { "<" }.as_bytes())
                .unwrap();
        }

        // Put back the cursor at the bottom of the terminal and flush the output
        stdout().execute(cursor::MoveTo(0, context.height)).unwrap();
        stdout().flush().unwrap();

        // Delay based on the desired frames per second
        let fps = 12;
        thread::sleep(Duration::from_millis(1000 / fps));
    }
}
