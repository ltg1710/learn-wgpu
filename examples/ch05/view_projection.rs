use cgmath::*;
use std::f32::consts::FRAC_PI_6;

fn main() {
    // position of the viewer
    let eye: Point3<f32> = Point3::new(3.0, 4.0, 5.0);
    //point the viewer is looking at
    let center: Point3<f32> = Point3::new(-3.0, -4.0, -5.0);
    // vector pointing up
    let up: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);
    // construct view matrix:
    let view_mat = Matrix4::look_at_rh(eye, center, up);
    println!("\nposition of viewer: {:?}", eye);
    println!("point the viewer is looking at: {:?}", center);
    println!("up direction: {:?}", up);
    println!("view matrix: {:?}\n ", view_mat);

    // frustum and perspective parameters
    let left = -3.0;
    let right = 3.0;
    let bottom = -5.0;
    let top = 5.0;
    let near = 1.0;
    let far = 100.0;
    let fovy = FRAC_PI_6;
    let aspect = 1.5;
    // construct the frustum matrix
    let frustum_mat = frustum(left, right, bottom, top, near, far);
    // construct perspective projection matrix
    let persp_mat = perspective(Rad(fovy), aspect, near, far);
    println!("\nfrustum matrix: {:?}\n ", frustum_mat);
    println!("perspective matrix: {:?}\n ", persp_mat);

    // construct orthographic projection matrix
    let ortho_mat = ortho(left, right, bottom, top, near, far);
    println!("orthographic matrix: {:?}\n ", ortho_mat);
}
