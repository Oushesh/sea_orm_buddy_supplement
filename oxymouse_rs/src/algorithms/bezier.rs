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
pub struct BezierMovement;

impl MouseMovement for BezierMovement {

    

}