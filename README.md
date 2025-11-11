# Cli-Boids
_To rice or not to rice, that's not even a question_

## Intro
This project started with one simple question, can I add more rice to my terminal? With all of the competition in the terminal time wasting space, it had to be something unique, warranting the creation of cli-boids. These boids can peacefully fly around while you are working on your next project, or be used as filler to show off your setup.

## How
These boids follow [three basic rules](https://vanhunteradams.com/Pico/Animal_Movement/Boids-algorithm.html):
- **Cohesion**: Each boid is attracted to the center of local boids.
- **Separation**: Each boid is repelled by boids that are too close.
- **Alignment**: Each boid tries to match the speed of local boids.

These three rules together can result in complex behavior similar to that of a swarm of birds (and that in your terminal!). Besides these three rules, the simulation currently also allows for:
- Mouse interaction
- Gravity
- Borders (So they don't fly away)
- Randomness
- .... (WIP)

Besides these rules, some effort has been put in optimizing the performance using grids for checking the neighbourhood, so hopefully it should be able to deal with any of your swarm simulating needs.

## TODO
- Custom parameter input.

**Extra**
- (Background) Color
- Path tracing
- Custom paths for boids to follow
- Custom boid shapes (arrows, lines)

**Extra-Extra**
- 3D boids
- Crab mode?
