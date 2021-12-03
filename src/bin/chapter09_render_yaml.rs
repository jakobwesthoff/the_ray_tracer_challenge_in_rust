use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use rayon::prelude::*;
use std::fs::{read_to_string, write};
use std::sync::Mutex;
use the_ray_tracer_challenge::canvas::to_png::*;
use the_ray_tracer_challenge::canvas::*;
use the_ray_tracer_challenge::world_loader::yaml::Yaml;
use the_ray_tracer_challenge::world_loader::WorldLoader;

use indicatif::ProgressBar;

fn main() -> Result<()> {
  let args: Vec<String> = std::env::args().collect();

  if args.len() != 2 {
    println!(
      r#"
The Raytracer Challenge Rust Renderer
(c) 2021 Jakob Westhoff

Usage: {} <world.yaml>
    "#,
      args[0]
    );
    return Err(anyhow!(
      "Expected 1 argument but got {}: {:?}.",
      args.len() - 1,
      args
    ));
  }

  let yaml_loader = Yaml::default();
  let source_file = &args[1];
  let source =
    read_to_string(source_file).context(format!("Could not read world file {}", source_file))?;
  let (world, cameras) = yaml_loader.load_world(source).context(format!(
    "Tried parsing {} as YAML world definition, but failed",
    source_file
  ))?;

  println!(
    "Going to render {} camera perspectives for a world with {} lights and {} bodies.",
    cameras.len(),
    world.lights.len(),
    world.bodies.len()
  );

  for (name, camera) in cameras.iter() {
    let canvas_mutex = Mutex::new(Canvas::new(camera.hsize, camera.vsize));

    let pixel_count = camera.hsize * camera.vsize;

    println!("Raytracing {} with {} pixels...", name, pixel_count);
    let progress = ProgressBar::new(pixel_count as u64);
    progress.set_draw_rate(5);

    (0..camera.hsize) // x
      .cartesian_product(0..camera.vsize) // y
      .par_bridge()
      .for_each(|(x, y)| {
        let color = world.color_at(camera.ray_for_pixel(x, y));
        let mut canvas = canvas_mutex.lock().unwrap();
        canvas.write_pixel(x, y, color);
        progress.inc(1);
      });

    progress.finish();

    println!("Writing ./{}.png", name);

    let canvas = canvas_mutex.lock().unwrap();
    let png = canvas.to_png();
    drop(canvas);
    write(format!("./{}.png", name), png)
      .context(format!("Could not write {}.png to disk.", name))?;
  }

  println!("Everything done.");

  Ok(())
}
