/*
fetcher/src/algorithms/bezier.rs
*/

/*
1. pub struct BezierMovement
In python, a Class is a package that contains both Data(attributes) and Behavior (methods)

In Rust, we separate them: 
	struct: Data only --> you can also add mathematical formula here.
	example input (x coordinate, y coordinate) --> tension or speed

	UniStruct.


2 Types of Impl in Rust: 

Type A: Normal Python Class

impl BezierMovement {
	pub fn new () --> Self {Self} --> //__init__ in python (default constructor)

}


Type B: 
This is what you saw with impl MouseMovement for BezierMovement.

*/

use crate::algorithm::base::MouseMovement;
use rand::Rng;
pub struct BezierMovement;


impl MouseMovement for BezierMovement {
    fn bernstein_poly(i:i32, j:i32, x:i32, y:i32) -> (i32,i32) {
        return 5;
    }

    fn bezier_curve ()->(){

    }


    fn generate_coordinates (from_x:f64, from_y:f64, to_x:f64, to_y:f64) -> (f64, f64) {

    }
    fn generate_random_coordinates () -> (f64,f64) {


    }


    fn generate_scroll_coordinates(start_y:i32)-> Vec!(): {
    /*
    Generate a vec of y-coordinates for scrolling
    from start_y to end_y using Bezier Curves
     */
        }
}