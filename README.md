# Better-Cli-Boids
_To rice or not to rice, that's not even a question_

## Intro
This project started with one simple question, can I add better rice to my terminal? With all of the competition in the terminal time-wasting space, it had to be something unique, better even, warranting the creation of better-cli-boids. These better-boids can peacefully fly around while you are working on your next project, or be used as filler to show off your setup.

## Usage
Figure it out, _better_

## How
These boids follow [three basic rules](https://vanhunteradams.com/Pico/Animal_Movement/Boids-algorithm.html):
- **Cohesion**: Each better-boid is attracted to the center of local better-boids.
- **Separation**: Each better-boid is repelled by better-boids that are too close.
- **Alignment**: Each better-boid tries to match the speed of local better-boids.

These three rules together can result in complex behavior similar to that of a swarm of birds (and that in your terminal!). Besides these three rules, the simulation currently also allows for:
- Mouse interaction
- Gravity
- Borders (So they don't fly away)
- Randomness
- .... (WIP)

Besides these better rules, a lot of effort has been put in optimizing the performance using better-grids for checking the neighbourhood, so it most certainly takes care of any of your swarm simulating needs.

## TODO
- Styling and readability (my sincerest apologies everyone)
- Custom parameter input.

**Extra**
- (Background) Color
- Path tracing
- Custom paths for boids to follow
- Custom boid shapes (arrows, lines)

**Extra-Extra**
- 3D better-boids
- better-better-boids
- Crab mode?
