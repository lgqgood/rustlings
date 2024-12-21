fn main() {
    // Fix the Clippy lint by using the standard library constant for pi.
    let pi = std::f64::consts::PI;
    let radius: f32 = 5.0;

    let area = pi * (radius as f64).powi(2);

    println!("The area of a circle with radius {:.2} is {:.5}", radius, area);
}