extern crate the_ray_tracer_challenge as raytracer;

use num_traits::Float;
use raytracer::tuple::*;

#[derive(Debug)]
struct Environment<T>
where
    T: Float,
{
    gravity: Tuple<T>,
    wind: Tuple<T>,
}

#[derive(Debug)]
struct Projectile<T>
where
    T: Float,
{
    position: Tuple<T>,
    velocity: Tuple<T>,
}

impl<T> Projectile<T>
where
    T: Float,
{
    pub fn new(position: Tuple<T>, velocity: Tuple<T>) -> Self {
        Projectile { position, velocity }
    }
}

impl<T> Environment<T>
where
    T: Float,
{
    pub fn new(gravity: Tuple<T>, wind: Tuple<T>) -> Self {
        Environment { gravity, wind }
    }
}

fn tick<T>(environment: &Environment<T>, projectile: &Projectile<T>) -> Projectile<T>
where
    T: Float,
{
    Projectile::new(
        projectile.position + projectile.velocity,
        projectile.velocity + environment.gravity + environment.wind,
    )
}

fn main() {
    let environment = Environment::new(
        Tuple::vector(0.0, -0.1, 0.0),
        Tuple::vector(-0.0001, 0.0, 0.0),
    );
    let projectile = Projectile::new(Tuple::point(0.0, 1.0, 0.0), Tuple::vector(0.02, 0.0, 0.0));

    println!("{:?}", environment);

    let mut current = projectile;
    let mut iteration: i32 = 0;
    while current.position.y > 0.0 {
        println!("{}: {:?}", iteration, current);
        current = tick(&environment, &current);
        iteration += 1;
    }
    println!("FINISHED => {}: {:?}", iteration, current);
}
