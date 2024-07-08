use crossterm::{cursor, terminal, ExecutableCommand};

use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;

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
    for _ in 0..NUM_PARTICLES {
        let p = Particle::new_random(0, context.width, 0, context.height);
        particles.push(p);
    }

    // Draw the particles
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

    loop {
        stdout()
            .execute(terminal::Clear(terminal::ClearType::All))
            .unwrap();
        for particle in &particles {
            stdout()
                .execute(cursor::MoveTo(particle.x, particle.y))
                .unwrap()
                .write(if particle.vy > 0 { "V" } else { "_" }.as_bytes())
                .unwrap();
        }

        // Put back the cursor at the bottom of the terminal
        stdout().execute(cursor::MoveTo(0, context.height)).unwrap();

        stdout().flush().unwrap();

        for particle in &mut particles {
            ParticleMovement::Gravity(ParticleBoundsHandling::Stop).update(particle, &context);
        }

        thread::sleep(Duration::from_millis(40));
    }
}
