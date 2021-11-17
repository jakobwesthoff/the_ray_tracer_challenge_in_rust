use itertools::Itertools;
use rayon::prelude::*;
use std::f64::consts::PI;
use std::fs::write;
use std::sync::Mutex;
use the_ray_tracer_challenge::animator::*;
use the_ray_tracer_challenge::body::*;
use the_ray_tracer_challenge::camera::Camera;
use the_ray_tracer_challenge::canvas::to_png::*;
use the_ray_tracer_challenge::canvas::*;
use the_ray_tracer_challenge::light::PointLight;
use the_ray_tracer_challenge::material::{Material, Phong};
use the_ray_tracer_challenge::matrix::Matrix;
use the_ray_tracer_challenge::sphere::*;
use the_ray_tracer_challenge::tuple::*;
use the_ray_tracer_challenge::world::World;

use indicatif::ProgressBar;

fn main() {
  let animator = Animator::new(25 * 5);
  animator.animate(|frame| {
    let canvas_width = 1920;
    let canvas_height = 1080;

    let light_rotation_scale = frame.linear_scale().with_range(vec![0.0, PI * 2.0]);
    let light_transformation_matrix =
      Matrix::rotation_y(light_rotation_scale.scale(frame.current_as_float()));

    let light = PointLight::new(
      light_transformation_matrix * Tuple::point(-2.0, 4.0, -2.0),
      Color::new(1.0, 1.0, 1.0),
    );

    // Floor and walls
    let floor_and_wall_material = Phong::default()
      .with_color(Color::new(0.5, 0.45, 0.45))
      .with_specular(0.0);

    let floor_sphere = Sphere::new(
      Material::from(floor_and_wall_material),
      Matrix::scaling(10.0, 0.01, 10.0),
    );

    let left_wall_sphere = Sphere::new(
      Material::from(floor_and_wall_material),
      Matrix::translation(0.0, 0.0, 5.0)
        * Matrix::rotation_y(-PI / 4.0)
        * Matrix::rotation_x(PI / 2.0)
        * Matrix::scaling(10.0, 0.01, 10.0),
    );

    let right_wall_sphere = Sphere::new(
      Material::from(floor_and_wall_material),
      Matrix::translation(0.0, 0.0, 5.0)
        * Matrix::rotation_y(PI / 4.0)
        * Matrix::rotation_x(PI / 2.0)
        * Matrix::scaling(10.0, 0.01, 10.0),
    );

    // Spheres
    let left_material = Phong::default().with_color(Color::new(0.78, 0.28, 0.96));
    let left_sphere = Sphere::new(
      Material::from(left_material),
      Matrix::translation(-1.5, 0.33, -0.75) * Matrix::scaling(0.33, 0.33, 0.33),
    );

    let middle_material = Phong::default()
      .with_color(Color::new(1.0, 0.49, 0.0))
      .with_diffuse(0.7)
      .with_specular(0.1)
      .with_shininess(50.0);

    let middle_sphere_translation_scale =
      frame.linear_scale().with_range(vec![-0.5, -0.5, 0.5, -0.5]);
    let middle_sphere = Sphere::new(
      Material::from(middle_material),
      Matrix::translation(
        middle_sphere_translation_scale.scale(frame.current_as_float()),
        1.0,
        0.5,
      ),
    );

    let right_material = Phong::default().with_color(Color::new(0.51, 0.75, 0.06));
    let right_sphere_translation_scale = frame.linear_scale().with_range(vec![0.5, 1.0, 3.0, 0.5]);
    let right_sphere = Sphere::new(
      Material::from(right_material),
      Matrix::translation(
        1.5,
        right_sphere_translation_scale.scale(frame.current_as_float()),
        -0.5,
      ) * Matrix::scaling(0.5, 0.5, 0.5),
    );

    let world = World::new(
      vec![
        Body::from(floor_sphere),
        Body::from(left_wall_sphere),
        Body::from(right_wall_sphere),
        Body::from(left_sphere),
        Body::from(middle_sphere),
        Body::from(right_sphere),
      ],
      vec![light],
    );

    let camera = Camera::new(canvas_width, canvas_height, PI / 3.0).look_at_from_position(
      Tuple::point(0.0, 3.5, -5.0),
      Tuple::point(0.0, 1.0, 0.0),
      Tuple::vector(0.0, 1.0, 0.0),
    );

    let canvas_mutex = Mutex::new(Canvas::new(canvas_width, canvas_height));

    let pixel_count = canvas_width * canvas_height;
    println!("Raytracing {} pixels. Please be patient...", pixel_count);

    let progress = ProgressBar::new(pixel_count as u64);
    progress.set_draw_rate(5);

    (0..canvas_width) // x
      .cartesian_product(0..canvas_height) // y
      .par_bridge()
      .for_each(|(x, y)| {
        let color = world.color_at(camera.ray_for_pixel(x, y));
        let mut canvas = canvas_mutex.lock().unwrap();
        canvas.write_pixel(x, y, color);
        progress.inc(1);
      });

    progress.finish();

    let filename = frame.filename("./", "output", ".png");
    println!("Writing {}", filename);

    let canvas = canvas_mutex.lock().unwrap();
    let png = canvas.to_png();
    drop(canvas);
    write(filename, png).expect("Could not write ouput.png to disk.");
  });

  println!("Everything done.");
  println!("Use ffmpeg to convert the output images into an animation movie:");
  println!("ffmpeg -stream_loop 4 -r 25 -f image2 -s 1920x1080 -i output%06d.png -vcodec libx264 -crf 25 animation.mp4");
}
