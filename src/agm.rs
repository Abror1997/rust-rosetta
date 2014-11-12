// http://rosettacode.org/wiki/Arithmetric-geometric_mean
// Accepts two command line arguments
// cargo run --name agm arg1 arg2

use std::num;

#[cfg(not(test))]
fn main () {
    let args = std::os::args();
    let args = args.as_slice();

    let x = from_str::<f32>(args[1].as_slice()).unwrap() ;
    let y = from_str::<f32>(args[2].as_slice()).unwrap() ;

    let result = agm(x,y);
    println!("The arithmetic-geometric mean is {}", result);
}

fn agm (x: f32, y: f32) -> f32 {
    let e: f32 = 0.000001;
    let mut a = x;
    let mut g = y;
    let mut a1: f32;
    let mut g1: f32;

    if a * g < 0f32 { panic!("The arithmetric-geometric mean is undefined for numbers less than zero!"); }
    else {
        loop {
            a1 = (a + g) / 2f32;
            g1 = (a * g).sqrt();
            a = a1;
            g = g1;
            if num::abs( a - g) < e {  return a; }
        }
    }

}

#[test]
fn test_agm_12_6() {
    assert! ( agm(12f32,6f32) == 8.740746f32 );
}
