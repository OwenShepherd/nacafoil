# nacafoil
NACA airfoil generation.

## Usage
This can be used to generate boundary points for NACA 4-digit airfoils.  
```rust
use crate::nacafoil::Airfoil;
fn generate_naca0006_boundary() {
    let n = 1000;
    let c: f64 = 1.0;
    let name: String = "0006".to_string();
    let airfoil = Airfoil::new(name, c, n);
    let upper_x = airfoil.upper_x;
    let upper_y = airfoil.upper_y;
    let lower_x = airfoil.lower_x;
    let lower_y = airfoil.lower_y;
}
```
Results are tested againt airfoil data in tests/data.json within 0.3 percent of chord.  
All 4-digit airfoils from NACA #824 aside from 2424 and 4424 pass tests.
