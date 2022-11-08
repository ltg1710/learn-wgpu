use cgmath::{Matrix4, Vector3, Vector4};
fn main() {
    // create original vector
    let my_vec = Vector4::new(1.0, 2.0, 3.0, 1.0);
    // create first translation matrix
    let my_mat = Matrix4::from_translation(Vector3::new(2.0, 2.5, 3.0));
    // get total translation matrix after another translation
    let trans_mat = my_mat * Matrix4::from_translation(Vector3::new(-3.0, -2.0, -1.0));
    // get final translated vector
    let trans_vec = trans_mat * my_vec;
    println!("\nOriginal vector: my_vec = \n{:?}", my_vec);
    println!(
        "Total translation matrix after two translations: trans_mat: \n{:?}",
        trans_mat
    );
    println!(
        "Vector after two translations: trans_vec = trans_mat * my_vec = \n{:?}\n ",
        trans_vec
    );
}
