pub mod four;

pub fn linspace(a: f64, b: f64, n: i32) -> Vec<f64> {
    let delta = (b-a) / ((n - 1) as f64);
    let mut vec = Vec::<f64>::with_capacity(n as usize);
    vec.push(a);
    for i in 1..(n-1) {
        let temp = a + (i as f64) * delta;
        vec.push(temp);
    }
    vec.push(b);
    return vec;
}
