# nacafoil
NACA airfoil generation.

## Usage
This can be used to generate boundary points for NACA 4-digit airfoils.  
```rust
use crate::nacafoil::generate_airfoil_boundary;
fn generate_naca0006_boundary() {
    let n = 100;
    let t: f64 = 0.06;
    let c: f64 = 1.0;
    let m: f64 = 0.0;
    let p: f64 = 0.0;
    let boundaries = generate_airfoil_boundary(m, p, t, c, n);
}
```
The return is a list of tuples [(x0, y0), (x1, y1), ...].

I have tested the NACA 0006 results against NACA Techincal report #824.  
Additionally, I have tested the NACA 2412 results against data from the UIUC
Applied Aerodynamics Group. Both results were tested within 0.2% of chord.  
Tests against the NACA 0006 were accurate, tests against NACA 2412 were not.
