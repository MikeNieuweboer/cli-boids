# Cli-Boids
_To rice or not to rice, that's not even a question_

<p align="center">
  <a href="#how-it-works">Key Features</a> •
  <a href="#how-to-use">How To Use</a> •
  <a href="#installation">Installation</a> •
  <a href="#configuration">Customization</a> •
  <a href="#todo">Project goals, alternatives</a><br>
</p>

## Intro
This project started with one simple question, can I add more rice to my terminal? With all of the competition in the terminal time wasting space, it had to be something unique, warranting the creation of cli-boids. These boids can peacefully fly around while you are working on your next project, or be used as filler to show off your setup.

## How it works
These boids follow [three basic rules](https://vanhunteradams.com/Pico/Animal_Movement/Boids-algorithm.html):
- **Cohesion**: Each boid is attracted to the center of local boids.
- **Separation**: Each boid is repelled by boids that are too close.
- **Alignment**: Each boid tries to match the speed of local boids.

These three rules together can result in complex behavior similar to that of a swarm of birds (and that in your terminal!). Besides these three rules, the simulation currently also allows for:
- Mouse interaction
- Gravity
- Friction
- Borders (So they don't fly away)
- Randomness
- .... (WIP)

Besides these rules, some effort has been put in optimizing the performance using grids for checking the neighbourhood, so hopefully it should be able to deal with any of your swarm simulating needs.

## How To Use
WIP

## Installation
WIP

## Configuration
WIP

## TODO
- Extend menu for all parameters
- CLI input.
- Store settings between runs.

**Extra**
- Improve performance (custom kdtree/maybe some other cool ds?)
- Port to library crate
- (Background) Color
- Path tracing
- Custom paths for boids to follow
- Custom boid shapes (arrows, lines)

**Extra-Extra**
- 3D boids
- Crab mode?
