//// Rust port of the Oxyhouse BezierMouse Algorith

use rand::Rng;

pub struct BezierMouse;

impl BezierMouse{
    ////Bernstein polynomial calculation remains exact.

    pub fn bernstein_poly(i:usize,n:usize,t:f64) -> f64 {
        Self::comb(n,i) as f64 * t.powi(i as i32) * (1.0 - t).powi((n - i) as i32)
    }

    fn comb(n:usize,k:usize) -> u64 {
        if k>n {return 0;}

        //else part
        let k = k.min(n-k);
        let mut result: u64 = 1;

        for i in 0..k {
            result = result*(n-i) as u64/(i+1) as u64;
        }
        return result;
    }

    /// New: Numerical function for Ease-In-Out timing (Velecity Bias).
    /// This mimics human acceleration and deceleration.

    fn ease_in_out(t: f64) -> f64 {
        if t < 0.5 {
            2.0 * t * t
        } else {
            let p = -2.0 * t + 2.0;
            1.0 - (p * p * p) / 2.0
        }
    }

    pub fn bezier_curve(points: &[(i32, i32)], num_steps: usize) -> Vec<(i32, i32)> {
        if points.is_empty() || num_steps == 0 {
            return Vec::new();
        }

        let n = points.len() - 1;
        let xpoints: Vec<f64> = points.iter().map(|p| p.0 as f64).collect();
        let ypoints: Vec<f64> = points.iter().map(|p| p.1 as f64).collect();

        // MODIFIED: Apply ease_in_out to t_values to create human velocity
        let t_values: Vec<f64> = (0..num_steps)
            .map(|s| {
                let linear_t = s as f64 / (num_steps - 1).max(1) as f64;
                Self::ease_in_out(linear_t) // Non-linear spacing of points
            })
            .collect();

        let polynomial_array: Vec<Vec<f64>> = (0..=n)
            .map(|i| {
                t_values
                    .iter()
                    .map(|&t| Self::bernstein_poly(i, n, t))
                    .collect()
            })
            .collect();

        (0..num_steps)
            .map(|s| {
                let x: f64 = xpoints
                    .iter()
                    .zip(polynomial_array.iter())
                    .map(|(xi, poly)| xi * poly[s])
                    .sum();
                let y: f64 = ypoints
                    .iter()
                    .zip(polynomial_array.iter())
                    .map(|(yi, poly)| yi * poly[s])
                    .sum();
                (x as i32, y as i32)
            })
            .collect()
    }

    pub fn generate_bezier_mouse_movements(
        start_x: i32,
        start_y: i32,
        end_x: i32,
        end_y: i32,
        duration: f64,
        complexity: usize,
        randomness: f64,
    ) -> Vec<(i32, i32)> {
        let complexity = complexity.max(4);
        let mut rng = rand::thread_rng();

        let mut control_points: Vec<(i32, i32)> = Vec::with_capacity(complexity);
        control_points.push((start_x, start_y));

        // MODIFIED: Improved OxyLabs-style heuristic for intermediate points.
        // Instead of pure random bounds, we pull points toward start/end with offsets.
        for i in 1..(complexity - 1) {
            let ratio = i as f64 / (complexity - 1) as f64;

            // Interpolate base position
            let base_x = start_x as f64 + (end_x - start_x) as f64 * ratio;
            let base_y = start_y as f64 + (end_y - start_y) as f64 * ratio;

            // Apply "Human Pull" (randomized arc scale)
            let jitter_scale = randomness * 150.0;
            let cx = base_x + rng.gen_range(-jitter_scale..jitter_scale);
            let cy = base_y + rng.gen_range(-jitter_scale..jitter_scale);

            control_points.push((cx as i32, cy as i32));
        }
        control_points.push((end_x, end_y));

        let num_steps = ((duration * 60.0) as usize).max(2);
        Self::bezier_curve(&control_points, num_steps)
    }

    pub fn generate_coordinates(from_x: i32, from_y: i32, to_x: i32, to_y: i32) -> Vec<(i32, i32)> {
        // Duration 0.8s - 1.2s is standard human movement
        Self::generate_bezier_mouse_movements(from_x, from_y, to_x, to_y, 1.0, 4, 1.0)
    }

    pub fn generate_random_coordinates(viewport_width: i32, viewport_height: i32) -> Vec<(i32, i32)> {
        let mut rng = rand::thread_rng();
        let end_x = rng.gen_range(0..=viewport_width);
        let end_y = rng.gen_range(0..=viewport_height);
        Self::generate_bezier_mouse_movements(0, 0, end_x, end_y, 1.2, 5, 0.8)
    }

    pub fn generate_scroll_coordinates(start_y: i32, end_y: i32) -> Vec<(i32, i32)> {
        let mut movements = Self::generate_bezier_mouse_movements(0, start_y, 0, end_y, 0.8, 4, 0.5);
        movements.push((0, end_y));
        movements
    }
}