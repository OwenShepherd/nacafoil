pub struct Airfoil {
    pub surface: Surface,
    max_camber: f64,
    max_camber_location: f64,
    thickness: f64,
    pub chord_length: f64,
    pub half_num: i32
}

pub struct Surface {
    pub upper_x: Vec<f64>,
    pub lower_x: Vec<f64>,
    pub upper_y: Vec<f64>,
    pub lower_y: Vec<f64>
}

impl Surface {
    pub fn new(c: f64, n: i32) -> Self {
        let coordinates = generate_coordinates(n, c);
        Self {
            upper_x: coordinates.clone(),
            lower_x: coordinates,
            upper_y: vec![0.0; n as usize],
            lower_y: vec![0.0; n as usize]
        }
    }
}

impl Airfoil {
    pub fn new(name: String, c: f64, n: i32) -> Self {
        let m: f64 = ((name.as_bytes()[0] as char).to_digit(10).unwrap() as f64) / 100.0;
        let p: f64 = ((name.as_bytes()[1] as char).to_digit(10).unwrap() as f64) / 10.0;
        let t: f64 = name[2..4].parse::<f64>().unwrap() / 100.0;
        let real_num = if n % 2 == 0 { n } else { n + 1 };
        let mut airfoil = Self {
            max_camber: m,
            max_camber_location: p,
            thickness: t,
            chord_length: c,
            half_num: real_num / 2,
            surface: Surface::new(c, real_num / 2)
        };
        airfoil.generate_surface();
        airfoil
    }
    fn generate_surface(&mut self) {
        self.generate_surface_aux(true);
        self.generate_surface_aux(false);
    }
    fn generate_surface_aux(&mut self,  upper: bool) {
        let direction: f64 = if upper { -1.0 } else { 1.0 };
        let ordinates: &mut Vec<f64> = if upper { &mut self.surface.upper_y } else { &mut self.surface.lower_y };
        let coordinates: &mut Vec<f64> = if upper { &mut self.surface.upper_x } else { &mut self.surface.lower_x };
        let m = self.max_camber;
        let p = self.max_camber_location;
        let t = self.thickness;
        let c = self.chord_length;
        for index in 0..self.half_num as usize {
            let coordinate = coordinates[index];
            let thickness = thickness_line(t, c, coordinate);
            let theta_m;
            let camber;
            if m==0.0 && p==0.0 {
                theta_m = 0.0;
                camber = 0.0;
            } else {
                theta_m = slope_of_camber_line(m, p, coordinate, c);
                camber = camber_line(m, p, coordinate, c);    
            }
            let new_coordinate = coordinate + (1.0 * direction) * thickness*f64::sin(theta_m);
            let new_ordinate = camber + thickness * (-1.0 * direction) * f64::cos(theta_m);
            coordinates[index] = new_coordinate;
            ordinates[index] = new_ordinate;
        }
    }
}

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

fn generate_coordinates(num: i32, chord_length: f64) -> Vec<f64> {
    // num_coordinates should be even
    let pi = std::f64::consts::PI;
    let start_value: f64 = 0.0;
    let radius = 0.5 * chord_length;
    let upper_theta_values = crate::linspace(start_value, pi, num); // Airfoil top contains both endpoints.
    let mut theta_values = Vec::<f64>::with_capacity(num as usize);
    for index in (0..num).rev() {
        let temp_value = radius + radius * f64::cos(upper_theta_values[index as usize]);
        theta_values.push(temp_value);
    }
    theta_values
}

fn thickness_line(t: f64, c: f64, mut coordinate: f64) -> f64 {
    coordinate = coordinate / c;
    (t / 0.2) * (0.2969*coordinate.sqrt() - 0.1260 * coordinate - 0.3516 * coordinate.powf(2.0) + 0.2843 * coordinate.powf(3.0) - 0.1015 * coordinate.powf(4.0))
}

fn slope_of_camber_line(m: f64, p: f64, x_loc: f64, c: f64) -> f64 {
    let slope: f64;
    if x_loc <= p*c {
        slope = 2.0*m/p - 2.0*m*x_loc/c/p/p;
    } else {
        slope = (m*c - m*x_loc)/(1.0-p)/(1.0-p)/c + (1.0+x_loc/c-2.0*p)*(-1.0*m/(1.0-p)/(1.0-p));
    }
    slope.atan()
}

fn camber_line(m: f64, p: f64, x_loc: f64, c: f64) -> f64 {
    if x_loc <= p*c {
        m * (x_loc)/p/p * (2.0 * p - x_loc/c)
    } else {
        m * (c - x_loc) / (1.0-p).powf(2.0) * (1.0 + x_loc/c - 2.0 * p)
    }
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
}
