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

fn find_closest(coordinates: &Vec<f64>, ordinates: &Vec<f64>, desired: f64) -> (f64, f64) {
    let num = coordinates.len();
    let mut current_min = std::f64::MAX;
    let mut target_x = 0.0;
    let mut target_y = 0.0;
    for index in 0..num {
        let test = coordinates[index];
        let current_diff = (test - desired).abs();
        if current_diff < current_min {
            current_min = current_diff;
            target_x = test;
            target_y = ordinates[index];
        }
    }
    (target_x, target_y)
}

fn assert_within_percent_chord(foil: &JSONFoil, airfoil: &nacafoil::Airfoil, percent: f64, upper: bool) -> Option<(f64, f64, String)> {
    let mut exp;
    let predicted_x;
    let predicted_y;
    let mut notch = 1.0;
    if upper {
        exp = &foil.upper;
        predicted_x = &airfoil.surface.upper_x;
        predicted_y = &airfoil.surface.upper_y;
    } else {
        exp = &foil.lower;
        if exp==&[(-2.0 as f64, -2.0 as f64)].to_vec() {
            exp = &foil.upper;
            notch = -1.0;
        }
        predicted_x = &airfoil.surface.lower_x;
        predicted_y = &airfoil.surface.lower_y;
    }
    let num = exp.len();
    for index in 0..num {
        let current_test_x = exp[index].0 * airfoil.chord_length / 100.0;
        let current_test_y = exp[index].1 * notch * airfoil.chord_length / 100.0;
        let predicted = find_closest(&predicted_x, &predicted_y, current_test_x);
        if (predicted.0 - current_test_x).abs() > percent / 100.0 * airfoil.chord_length {
            return Some((predicted.0, current_test_x, "x".to_string()));
        } else if (predicted.1 - current_test_y).abs() > percent / 100.0 * airfoil.chord_length {
            return Some((predicted.1, current_test_y, "y".to_string()));
        }
    }
    None
}

#[test]
fn compare_surface_with_naca_technical_824() {
    let foils = read_data().unwrap();
    let mut found_values = Vec::<(bool, String)>::with_capacity(foils.len());
    for foil in foils {
        if foil.name.len() == 4 {
            let c: f64 = 1.0;
            let possible_n: [i32; 3] = [100, 1000, 10000];
            let mut upper_result;
            let mut lower_result;
            let mut assert_message: String = "".to_string();
            let mut airfoil;
            let mut found_value = false;
            for num in possible_n {
                airfoil = nacafoil::Airfoil::new(foil.name.clone(), c, num);
                upper_result = assert_within_percent_chord(&foil, &airfoil, 0.3, true);
                lower_result = assert_within_percent_chord(&foil, &airfoil, 0.3, false);
                match (upper_result, lower_result) {
                    (Some(rtr), _) | (_, Some(rtr))=> {
                        assert_message = format!("Cannot generate NACA {} within {} percent of chord.\nFailed on predicted_{}: {}, expected_{}: {}", foil.name, "0.3", rtr.2, rtr.0, rtr.2, rtr.1);
                        continue;
                    }
                    (None, None) => {
                        assert!(true);
                        found_value = true;
                        println!("Generates NACA {} within {} percent of chord with N = {}", foil.name, "0.3", num);
                        break;
                    }
                }
            }
            if found_value {
                found_values.push((found_value, "".to_string()));
                continue;
            } else {
                found_values.push((found_value, assert_message));
            }
        }
    }
    for val in found_values {
        if !val.0 {
            assert!(false, "{}", val.1);
        }
    }
}