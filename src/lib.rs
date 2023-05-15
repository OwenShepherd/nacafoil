pub mod four;

fn linspace(a: f64, b: f64, n: i32) -> Vec<f64> {
    let delta = (b-a) / ((n - 1) as f64);
    let mut vec = Vec::<f64>::with_capacity(n as usize);
    vec.push(a);
    for i in 1..(n-1) {
        let temp = a + (i as f64) * delta;
        vec.push(temp);
    }
    vec.push(b);
    vec
}

pub fn generate_symmetric_xcoords(chord_length: f64, num: i32) -> Vec<f64>{
    let pi = std::f64::consts::PI;
    let start_value: f64 = 0.0;
    let radius = 0.5 * chord_length;
    let num_coordinates: i32 = if num % 2 != 0 { num } else { num + 1 }; // Force odd number of coords.
    let mut x_coordinates: Vec<f64> = Vec::<f64>::with_capacity(num_coordinates as usize);
    for index in 0..num_coordinates {
        x_coordinates.push(0.0);
    }
    let upper_theta_values = crate::linspace(start_value, pi, num / 2 + 1); // Airfoil top contains both endpoints.
    for index in 0..(num/2) {
        let temp_value = radius + radius * f64::cos(upper_theta_values[index as usize]);
        x_coordinates[index as usize] = temp_value;
        x_coordinates[(num - index - 1) as usize] = temp_value;
    }
    x_coordinates
}

#[cfg(test)]
mod tests {
    #[test]
    fn can_linspace() {
        use crate::linspace;
        let a = 0.0;
        let b = 1.0;
        let n = 5;
        let exp_vec = linspace(a,b,n);
        assert_eq!(exp_vec, [0.0, 0.25, 0.5, 0.75, 1.0]);
    }
    #[test]
    fn can_linspace_backwards() {
        use crate::linspace;
        let a = 1.0;
        let b = 0.0;
        let n = 5;
        let exp_vec = linspace(a,b,n);
        assert_eq!(exp_vec, [1.0, 0.75, 0.5, 0.25, 0.0]);
    }
    #[test]
    fn can_generate_xcoords() {
        let n = 13;
        let x = crate::generate_symmetric_xcoords(1.0, n);
        assert_eq!(x, [1.0, 0.9330, 0.75, 0.5, 0.25, 0.06698, 0.0, 0.06698, 0.25, 0.5, 0.75, 0.9330, 1.0]);
    }

}

