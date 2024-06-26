# mgfw
## Rust Mini Game Framework
MGFW is a batteries-included framework written in Rust for rapid prototyping of game concepts and algorithms yet still featureful and flexible to use as a starting point for larger, more complete games.

FYI, this repo is out of date, see Galaxion Trade Empire and mirr/orb repos for the most up-to-date version.

### Features
MGFW supports the following high level features:
- Entity-Component-System (ECS) Data-Driven Architecture
- 1200 Hz Simulation Executive Clock
- 60 Hz Render Clock
- 150 Hz Physics Euler Integrator
- Dedicated Continuous Cache Block for game state
- OpenGL rendering for Lines, Polygons, and Texture Billboards
- Built-in Bitmap Font
- Mouse event handling (Keyboard and Gamepad in work)
- Performance Monitoring and Reporting

### Documentation
Placeholder for information on feature examples contained inside this repo.

### Tutorials
Placeholder for tutorials on building simple common games.

### Games
Links to mini games that use this framework.
- [Halloween Mahjong Solitaire](https://github.com/Syn-Nine/rust-mini-games/tree/main/2d-games/mahjong)
- [Match Three](https://github.com/Syn-Nine/rust-mini-games/tree/main/2d-games/match-three)
- [Tet-Rust](https://github.com/Syn-Nine/rust-mini-games/tree/main/2d-games/tet-rust)
- [Tic-Tac-Toe](https://github.com/Syn-Nine/rust-mini-games/tree/main/2d-games/tictactoe)

### Future Work
Placeholder for list of desired future features.

### Support Crates
MGFW uses the following crates for direct feature support:
- gl
- glutin
- takeable-option
- rand
- image
- cgmath
