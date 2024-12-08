# particlez

A toy particle system written in Rust with `crossterm`, maybe useful for TUI applications.

## Examples

#### Fall to Floor

Uses `ParticleBoundsHandling::Stop` and displays particle based on its velocity.

```
cargo run --example fall_to_floor
```

![fall_to_floor](https://github.com/user-attachments/assets/c4255373-25b5-459c-92b6-817741f6f2d0)

#### Rainfall

Uses `ParticleBoundsHandling::Wrap` and displays particle based on its velocity.

```
cargo run --example rainfall
```

![rainfall](https://github.com/user-attachments/assets/dc5e1a02-7620-47e2-b20f-b61487e7dc02)

#### Bugs Crawl

Uses `ParticleBoundsHandling::Wrap` and varying velocities to simulate bugs crawling.

```
cargo run --example bugs_crawl
```

![bugs_crawl](https://github.com/user-attachments/assets/6bcdb9cf-4472-4d6f-901c-054d8b54a94e)

#### Spaceflight

Uses `ParticleBoundsHandling::Wrap` and varying velocities to simulate passing objects near and far in space.

```
cargo run --example spaceflight
```

![spaceflight](https://github.com/user-attachments/assets/1635d5a7-4f5f-416b-89ed-cd31a47c3229)

## Changelog

| Date       | Version | Description                          |
| ---------- | ------- | ------------------------------------ |
| 2024-07-08 | 0.0.1   | Supports minimal animation examples. |
