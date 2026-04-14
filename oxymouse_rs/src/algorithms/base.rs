/*
This script is the equivalent of base.py from the oxymouse algorithm/folder
In Rust base.rs will define the trai and mod.rs (or lib.rs) will orhestrate
how these algorithms are exposed.

Folder Structure: 
fetcher/
└── src/
    ├── lib.rs
    └── algorithms/
        ├── mod.rs        <-- Exposes the sub-modules
        ├── base.rs       <-- The Trait (Equivalent to base.py)
        ├── bezier.rs     <-- Implementation (Equivalent to bezier.py)
        └── horizontal.rs <-- Implementation (Equivalent to horizontal.py)

*/


/*
algorithms/base.rs (The Blueprint)
Abstract class in Rust is trait.
*/


pub trait MouseMovement {
	fn generate_coordinates(
		from_x: i32,
		from_y: i32,
		to_x:i32,
		to_y:i32
		) -> Vec <(i32,i32)>;
		//Generate a list/vec of coordinates from (from_x,from_y) to (to_x,to_y).

	fn generate_random_coordinates(
		viewport_width:i32,
		viewport_height:i32,
		) -> Vec<(i32,i32)>;
		//Generate random coordinates with the given viewport dimensions.

	fn generate_scroll_coordinates(
		start_y:i32,
		end_y:i32
		) -> Vec<(i32,i32)>;
		//Generate a list of y-coordinates for scrolling from start_y to end_y. 
}



//This function will be custom implemented in either bezier.rs, gaussian.rs,