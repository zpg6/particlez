use crossterm::{cursor, terminal, ExecutableCommand};
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
    for _ in 0..NUM_PARTICLES {
        // Generate a new random particle with x between 0 and width, and y between 0 and height
        let p = Particle::new_random(0, context.width, 0, context.height);
        particles.push(p);
    }

    // Draw the particles at their initial positions
    stdout()
        .execute(terminal::Clear(terminal::ClearType::All))
        .unwrap();
    for particle in &particles {
        stdout()
            .execute(cursor::MoveTo(particle.x, particle.y))
            .unwrap()
            .write("@".as_bytes())
            .unwrap();
    }

    // Put back the cursor at the bottom of the terminal
    stdout().execute(cursor::MoveTo(0, context.height)).unwrap();

    // Wait 1 second before starting the animation
    thread::sleep(Duration::from_secs(1));

    // Continuously update the particles with `Gravity` movement and render them to the terminal
    // Moving particles render as "V" if they are moving down, and "_" if they are landed (flattened).
    loop {
        for particle in &mut particles {
            ParticleMovement::Gravity(ParticleBoundsHandling::Stop).update(particle, &context);
        }

        // Clear the terminal so everything can be re-rendered
        stdout()
            .execute(terminal::Clear(terminal::ClearType::All))
            .unwrap();

        // Render each particle, updating its character based on its velocity
        for particle in &particles {
            stdout()
                .execute(cursor::MoveTo(particle.x, particle.y))
                .unwrap()
                .write(if particle.vy > 0 { "V" } else { "_" }.as_bytes())
                .unwrap();
        }

        // Put back the cursor at the bottom of the terminal and flush the output
        stdout().execute(cursor::MoveTo(0, context.height)).unwrap();
        stdout().flush().unwrap();

        // Delay based on the desired frames per second
        let fps = 40;
        thread::sleep(Duration::from_millis(1000 / fps));
    }
}
