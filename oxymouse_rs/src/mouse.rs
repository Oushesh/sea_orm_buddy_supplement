//mouse.rs is the main coordinator of this software
//we define which algorithm to implement here.

//We have 3 algorithms: Bezier, Gaussian and Perlin 

/*
mouse.rs (The main coordinator)
utils.rs (General Helpers)
algorithms/
	-> mod.rs
	-> bezier.rs (Bezier Curves)
	-> Gaussian.rs (Gaussian Noise)
	-> perlin.rs (Perlin noise)
	-> 
*/


pub enum Algorithm {
	Bezier,
	Gaussian,
	Perlin,
}

pub struct OxyMouse {
	pub algorithm: Algorithm,

	pub screen_width: f64,
	pub screen_height: f64,
}


//TODO: implementation here: 

impl OxyMouse {
	
}