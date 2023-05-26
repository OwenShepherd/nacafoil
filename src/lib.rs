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

fn generate_symmetric_xcoords(chord_length: f64, num: i32) -> Vec<f64> {
    let pi = std::f64::consts::PI;
    let start_value: f64 = 0.0;
    let radius = 0.5 * chord_length;
    let mut x_coordinates: Vec<f64> = Vec::<f64>::with_capacity(num as usize);
    for _ in 0..num {
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

fn generate_yt(t: f64, x_coordinates: &Vec<f64>) -> Vec<f64> {
    let num = x_coordinates.len();
    let mut yt = Vec::<f64>::with_capacity(num as usize);
    for item in x_coordinates {
        let current_calc = (t / 0.2) * (0.29690*item.sqrt() - 0.126 * item - 0.3516 * item.powf(2.0) + 0.2843 * item.powf(3.0) - 0.1015 * item.powf(4.0));
        yt.push(current_calc);
    }
    yt
}

fn slope_of_camber_line(m: f64, p: f64, x_div_c: f64, c: f64) -> f64 {
    if x_div_c <= p {
        return f64::atan((m*x_div_c*c/p.powf(2.0))*(-1.0/c)+(m/p.powf(2.0))*(2.0*p-x_div_c));
    } else {
        return f64::atan((m*(c-x_div_c*c)/(1.0-p.powf(2.0)))*(1.0/c)+((-1.0*m)/(1.0-p.powf(2.0)))*(1.0+x_div_c-2.0*p));
    }
}

fn camber_line(m: f64, p: f64, x_div_c: f64, c: f64) -> f64 {
    if x_div_c <= p {
        return m * (x_div_c*c)/p.powf(2.0) * (2.0 * p - x_div_c);
    } else {
        return m * (c - x_div_c*c) / (1.0-p).powf(2.0) * (1.0 + x_div_c - 2.0 * p);
    }
}

pub fn generate_airfoil_boundary(m: f64, p: f64, t: f64, c: f64, num: i32) -> Vec<(f64, f64)> {
    let skip_pivot = num / 2 + 1; // Below this index is x_upper, above it is x_lower.
    let num_coordinates: i32 = if num % 2 != 0 { num } else { num + 1 }; // Force odd number of coords.
    let mut boundary = Vec::<(f64, f64)>::with_capacity(num_coordinates as usize);
    let x_coordinates = generate_symmetric_xcoords(c, num_coordinates);
    let y_thickness = generate_yt(t, &x_coordinates);
    for index in 0..num_coordinates {
        let current_x = x_coordinates[index as usize];
        let current_yt = y_thickness[index as usize];
        let theta_m = slope_of_camber_line(m, p, current_x/c, c);
        let current_yc = camber_line(m, p, current_x/c, c);
        let current: (f64, f64);
        if index < skip_pivot {
            current = (current_x - current_yt*f64::sin(theta_m), current_yc + current_yt * f64::cos(theta_m));
        } else if index > skip_pivot {
            current = (current_x + current_yt*f64::sin(theta_m), current_yc - current_yt * f64::cos(theta_m));
        } else {
            current = (0.0, 0.0);
        }
        boundary.push(current);
    }
    boundary
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
        let test_against: Vec<f64> = [1.0, 0.9330, 0.75, 0.5, 0.25, 0.06698, 0.0, 0.06698, 0.25, 0.5, 0.75, 0.9330, 1.0].to_vec();
        let delta = 0.0001;
        for index in 0..n {
            assert!((x[index as usize]-test_against[index as usize]).abs() < delta)
        }
    }
    #[test]
    fn generate_naca0006_boundary() {
        // Test or "True" data from  NACA Techincal report #824
        let test_against: Vec<(f64, f64)> = [
            (0.0, 0.0),
            (0.0125, 0.0097),
            (0.025, 0.01307),
            (0.05, 0.0177),
            (0.075, 0.02100),
            (0.1, 0.02341),
            (0.15, 0.02673),
            (0.2, 0.02809),
            (0.25, 0.02971),
            (0.3, 0.03001),
            (0.4, 0.02902),
            (0.5, 0.02647),
            (0.6, 0.02282),
            (0.7, 0.01832),
            (0.8, 0.01312),
            (0.9, 0.00724),
            (0.95, 0.00403),
            (1.0, 0.0063)].to_vec();
        let n = 10000;
        let t: f64 = 0.06;
        let c: f64 = 1.0;
        let m: f64 = 0.0;
        let p: f64 = 0.0;
        let boundary = crate::generate_airfoil_boundary(m, p, t, c, n);
        let mut test_index = 0;
        let mut exp_index = 0;
        let mut prev_diff = std::f64::MAX;
        let test_diff = 0.035;
        while test_index < test_against.len() {
            let current_test_x = test_against[test_index].0;
            let current_exp_x = boundary[exp_index].0;
            let current_diff = (current_test_x - current_exp_x).abs();
            if current_diff >= prev_diff || (exp_index as i32==(n-1) && current_diff <= prev_diff) {
                let current_test_y = test_against[test_index].1;
                let current_exp_y = boundary[exp_index].1.abs();
                println!("True X: {current_test_x}, Exp X: {current_exp_x}");
                println!("True Y: {current_test_y}, Exp Y: {current_exp_y}");
                assert!((current_test_y - current_exp_y).abs() <= test_diff);
                test_index = test_index + 1;
                prev_diff = std::f64::MAX;
            } else {
                prev_diff = current_diff;
                exp_index = exp_index + 1;
            }
        }
    }
}
