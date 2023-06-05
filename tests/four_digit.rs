use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use nacafoil;

#[derive(Deserialize)]
struct JSONFoil {
    name: String,
    upper: Vec<(f64, f64)>,
    lower: Vec<(f64, f64)>
}

fn read_data() -> Result<Vec<JSONFoil>, Box<dyn Error>> {
    let base_path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = base_path.join("tests").join("data.json");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let foils: Vec<JSONFoil> = serde_json::from_reader(reader)?;
    Ok(foils)
}

#[test]
fn compare_surface_with_naca_technical_824() {
    let foils = read_data().unwrap();
    for foil in foils {
        if foil.name.len() == 4 {
            let m = (foil.name.as_bytes()[0] as char).to_digit(2).unwrap() as f64 / 100.0;
            let p = (foil.name.as_bytes()[1] as char).to_digit(2).unwrap() as f64 / 10.0;
            let t = &foil.name[2..3].parse::<f64>().unwrap() / 100.0;
            let c: f64 = 1.0;
            let possible_n: [i32; 4] = [100, 1000, 10000, 100000];
            let four_digit_diff = 0.002 * c;
            for num in possible_n {
                let airfoil_results = nacafoil::generate_airfoil_boundary(m, p, t, c, num);
            }
        }
    }
}