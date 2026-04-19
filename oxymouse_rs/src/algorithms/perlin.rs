use std::time::{Duration, Instant};
use std::thread::sleep;
use rand::Rng;
use noise::{NoiseFn, Perlin, Fbm, MultiFractal};

pub struct PerlinConfig {
    pub duration: f64,
    pub octaves: i32,
    pub persistence: f64,
    pub lacunarity: f64,
    pub seed: u32,
    pub from_x:i32,
    pub from_y:i32,
    pub to_x:i32,
    pub to_y:i32,
}

impl Default for PerlinConfig {
    fn default() -> Self {
        Self {
            duration: 1.0,
            octaves: 6,
            persistence: 0.5,
            lacunarity: 2.0,
            seed: rand::thread_rng().gen_range(0..=100000),
        }
    }
}

pub struct PerlinMouse;

impl PerlinMouse {
    // Added pub so you can call it from main, and & to borrow the config
    pub fn generate_perlin_mouse_movements(config: &PerlinConfig) -> Vec<(i32, i32)> {
        // Use floats for the math to avoid casting inside the loop
        let screen_width = 1920.0;
        let screen_height = 1080.0;

        let mut coordinates = Vec::new();

        // 1. Setup the Noise Generator (Done once)
        let perlin = Perlin::new(config.seed);
        let fbm = Fbm::<Perlin>::new(perlin)
            .set_octaves(config.octaves as usize)
            .set_persistence(config.persistence)
            .set_lacunarity(config.lacunarity);

        let start_time = Instant::now();

        while start_time.elapsed().as_secs_f64() < config.duration {
            let elapsed = start_time.elapsed().as_secs_f64();

            // Normalize time (0.0 to 1.0)
            let t = elapsed / config.duration;

            // Generate noise values
            let x_noise = fbm.get([t, config.seed as f64]);
            let y_noise = fbm.get([t, (config.seed + 1) as f64]);

            // Map noise (-1.0 to 1.0) to screen coordinates
            // (noise + 1.0) / 2.0 brings it to 0.0 -> 1.0 range
            let x = ((x_noise + 1.0) / 2.0 * screen_width) as i32;
            let y = ((y_noise + 1.0) / 2.0 * screen_height) as i32;

            coordinates.push((x, y));

            // Sleep to mimic 100Hz frequency
            sleep(Duration::from_millis(10));
        }

        coordinates // No need for 'return' at the end of functions in Rust
    }
    pub fn generate_coordinates()->Vec<(i32, i32)>{
        //
    }
    pub fn generate_random_coordinates()->Vec<> {

    }
}

//Finish implementing this code + the tests here.
