/*
Translation of Gaussian.rs
https://www.youtube.com/watch?v=9cQBNYsCqQs
 */

use rand::Rng;
use rand_distr::{Distribution, Normal};

#[derive(Debug,Clone,Copy)]
pub struct GaussianConfig {
    pub duration: f64,
    pub smoothness: f64,
    pub randomness: f64,
}


impl Default for GaussianConfig {
    fn default() -> Self {
        Self
        {
            duration: 1.0,
            smoothness: 2.0,
            randomness: 1.0,
        }
    }
}
pub struct GaussianMouse;
impl GaussianMouse {
    fn random_walk (length:i32,stddev:f64)-> Vec<f64>{
        //calculate the culmunative random sum of the vector
        let mut rng = rand::thread_rng();
        let normal = Normal::new(0.0,stddev).unwrap();
        let mut walk = Vec::with_capacity(length);
        let mut current = 0.0;
        for _ in 0.. num_points {
            current += normal.sample(&mut rng);
            walk.push(current);
        }

        return walk;
    }

    //Gaussian smoothing function Applies a Gaussian Blur Kernel to the path to make it smooth.
    fn gaussian_smooth(data:Vec<f64>,sigma:f64)->Vec<f64>{
        /*
        Gaussian Kernel: formula
        99.7% lies within 3 standard deviation --> otherwise we go to infinity
         */
        let size = (sigma*3.0).ceil() as i32;
        let mut smoothed = Vec::with_capacity(data.len());

        //Loop over the data from the original vector
        // Apply the operations of kernel multiplication
        // to the vector
        // for j in range(-size,size+1) -->
        // in or loop

        // The Gaussian Physics: e^(-j^2/(2*sigma^2))

        for i in 0..data.len() {
            let mut val = 0.0;
            let mut weight_sum  = 0.0;

            for j in -size.. =size {
                let idx = (i as i32 + j).clamp(0, data.len() as i32 -1) as usize;
                let weight = (-(j as f64).powi(2)/(2.0*sigma.powi(2))).exp();
                val += data[idx]* weight;
                weight_sum += weight;
            }
        }
    }
    fn morph_distribution->{

    }

    fn bezier_curve()->{

    }


    //implement a
    fn generate_gaussian_mouse_movements()->Vec<(i32,i32)>{
        //Generate mouse movements using Gaussian random walk and Bezier curves.
        // :param start_x: Starting x-coordinate
        // :param start_y: Starting y-coordinate
        // :param end_x: Ending x-coordinate
        // : param duration: Duration of the movement in seconds
        // : param smoothness: Controls the smoothness of the path (higher value=smoother)
        // : param randomness: Controls the randomness of the path ()

        /*
        include: The professional way to implement the argument defualt from python
        into Rust ist via Default struct.
        1. Define the struct that needs default and implement
        Default trait.
        2.

         */

        duration


    }

    fn generate_coordinates()->{


    }

    fn generate_random_coordinates(viewport_width:32)->
    {

    }

    fn generate_scroll_coordinates(start_y:i32,end_y:i32)->Vec<(i32, i32)>{
        //Generate a list of y-coordinates for scrolling from start_y and end_y using Gaussian
        //random noise

    }



}


/*
how to port default arguments from python to rust.
 */

//Complete this code here for the porting of the code from python to rust.