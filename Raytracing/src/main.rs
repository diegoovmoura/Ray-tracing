// a simple ray tracer in rust translatin this tutorial
// https://raytracing.github.io/books/RayTracingInOneWeekend.html#overview

mod vec3;
mod ray;

use ray::Ray;
use vec3::Vec3;

fn color(r: &Ray) -> Vec3{
    let unit_direction = Vec3::unit_vector(&r.direction());
    let t = 0.5*unit_direction.y() + 1.0;

    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t 
}

fn main() {

    //image
    let width: i32 = 256;
    let height: i32 = 256;
    

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);

    //render

    println!("P3\n{} {}\n255", width, height);

    for j in (0..height).rev() {
        for i in 0..width {
            let u: f32 = i as f32 / width as f32;
            let v: f32 = j as f32 / height as f32;
            let r = Ray::ray(origin, lower_left_corner + horizontal * u + vertical * v);
            let col = color(&r);

            let ir: i32 = (255.99 * col.r()) as i32;
            let ig: i32 = (255.99 * col.g()) as i32;
            let ib: i32 = (255.99 * col.b()) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
