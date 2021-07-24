use the_ray_tracer_challenge::canvas::to_ppm::*;
use the_ray_tracer_challenge::canvas::to_png::*;
use the_ray_tracer_challenge::canvas::*;
use the_ray_tracer_challenge::body::*;
use the_ray_tracer_challenge::ray::*;
use the_ray_tracer_challenge::sphere::*;
use the_ray_tracer_challenge::tuple::*;
use std::fs::write;

use indicatif::ProgressBar;

fn main() {
  let ray_origin = Tuple::point(0.0, 0.0, -5.0);
  let wall_position_z = 5.0;
  let wall_size = 10.0;

  let canvas_size = 1024;
  let canvas_pixel_world_size = wall_size / canvas_size as f64;

  let yellow = Color::new(1.0, 1.0, 0.0);

  let mut canvas = Canvas::new(canvas_size, canvas_size);

  let sphere = Sphere::new(None);

  println!("Raytracing {} pixels. Please be patient...", canvas_size.pow(2));

  let progress = ProgressBar::new(canvas_size.pow(2) as u64);
  progress.set_draw_rate(5);

  for y in 0..canvas_size {
    for x in 0..canvas_size {
      let half = wall_size / 2.0;
      let world_x = -half + (x as f64) * canvas_pixel_world_size;
      let world_y = half - (y as f64) * canvas_pixel_world_size;

      let wall_point = Tuple::point(world_x, world_y, wall_position_z);

      let ray = Ray::new(ray_origin, (wall_point - ray_origin).normalize());

      let xs = sphere.intersect(ray);

      if xs.hit() != None {
        canvas.write_pixel(x, y, yellow);
      }
      progress.inc(1);
    }
  }
  progress.finish();

  println!("Writing ./output.ppm");
  let ppm = canvas.to_ppm();
  write("./output.ppm", ppm).expect("Could not write ouput.ppm to disk.");
  println!("Writing ./output.png");
  let png = canvas.to_png();
  write("./output.png", png).expect("Could not write ouput.png to disk.");

  println!("Everything done.");

}
