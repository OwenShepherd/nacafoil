mod constants;

pub fn generate_airfoil(m: f64, p: f64, c: f64, x: &mut[f64], y: &mut[f64], num_samples: i32) -> () {
    //
    for i in 0..(num_samples / 2) {
        let xtemp = x[i as usize] / c;
        let ytemp = y[i as usize] / c;
        let slope = slope_of_camber_line(m,p,xtemp, c);
        let yc = camber_line(m,p, xtemp, c);

        x[i as usize] = (xtemp - ytemp * f64::sin(slope)) * c;
        x[((num_samples - 1) - i) as usize] = (xtemp + ytemp * f64::sin(slope)) * c;
        y[i as usize] = (yc + ytemp * f64::cos(slope)) * c;
        y[((num_samples - 1) - i) as usize] = (yc - ytemp * f64::cos(slope)) * c;
    }
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

fn generate_symmetric_airfoil(_m: f64, _p: f64, t: f64, chord_length: f64, num_samples: i32) -> (Vec<f64>, Vec<f64>) {
    let x_coordinates = generate_symmetric_xcoords(chord_length, num_samples);
    let mut y_coordinates = Vec::<f64>::with_capacity(num_samples as usize);
    let halfway_index = ( num_samples / 2) - 1;
    for i in 0..num_samples {
        let mut height = (t / 0.20) * symmetric_0020_airfoil_ordinate(x_coordinates[i as usize], chord_length);       
        if i > halfway_index {
            height *= -1.0;
        }   
        y_coordinates.push(height);
    }
    (x_coordinates, y_coordinates)
}

pub fn generate_symmetric_xcoords(chord_length: f64, num_samples: i32) -> Vec<f64>{
    let pi = std::f64::consts::PI;
    let start_value: f64 = 0.0;
    let end_value: f64 = pi * 2.0;
    let radius = 0.5 * chord_length;
    let mut x_coordinates = Vec::<f64>::with_capacity(num_samples as usize);
    if num_samples % 2 != 0 {
        let theta_values = crate::linspace(start_value, end_value, num_samples);
        for angle in theta_values {
            let temp_value = radius + radius * f64::cos(angle);
            x_coordinates.push(temp_value);
        }
    }
    else {
        let half_samples = num_samples / 2;
        let upper_theta_values = crate::linspace(start_value, pi, half_samples);
        let lower_theta_values = crate::linspace(pi, 2.0 * pi, half_samples);
        for i in 0..half_samples {
            let temp_value = radius + radius * f64::cos(upper_theta_values[i as usize]);
            x_coordinates.push(temp_value);
        }
        for i in 0..half_samples {
            let temp_value = radius + radius * f64::cos(lower_theta_values[i as usize]);
            x_coordinates.push(temp_value);
        }
    }
    x_coordinates
}

fn symmetric_0020_airfoil_ordinate(x: f64, chord_length: f64) -> f64 {
    chord_length * (
        (constants::A0 * (x / chord_length).sqrt()) + 
        (constants::A1 * (x / chord_length)) + 
        (constants::A2 * (x / chord_length).powf(2.0)) +
        (constants::A3 * (x / chord_length).powf(3.0)) + 
        (constants::A4 * (x / chord_length).powf(4.0)))
}

#[cfg(test)]
mod tests {
    #[test]
    fn can_generate_xcoords() {
        let n = 10;
        let x = crate::four::generate_symmetric_xcoords(1.0, n);
        assert_eq!(x.len() as i32, n);
    }
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
}
