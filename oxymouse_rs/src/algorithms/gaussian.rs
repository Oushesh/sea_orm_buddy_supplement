/// Rust port of the OxyMouse GaussianMouse algorithm.
///
/// Reference (Python):
/// <https://github.com/oxylabs/OxyMouse/blob/master/oxymouse/algorithms/gaussian_mouse/gaussian_mouse.py>
use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct GaussianConfig {
    pub duration: f64,
    pub smoothness: f64,
    pub randomness: f64,
}

impl Default for GaussianConfig {
    fn default() -> Self {
        Self {
            duration: 1.0,
            smoothness: 2.0,
            randomness: 1.0,
        }
    }
}

pub struct GaussianMouse;

impl GaussianMouse {
    fn sample_normal(rng: &mut impl Rng, stddev: f64) -> f64 {
        if stddev <= 0.0 {
            return 0.0;
        }
        let u1 = rng.gen_range(1e-12_f64..1.0_f64);
        let u2 = rng.gen_range(0.0_f64..1.0_f64);
        let z0 = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
        z0 * stddev
    }

    fn random_walk(length: usize, stddev: f64) -> Vec<f64> {
        let mut rng = rand::thread_rng();
        let mut walk = Vec::with_capacity(length);
        let mut current = 0.0;
        for _ in 0..length {
            current += Self::sample_normal(&mut rng, stddev);
            walk.push(current);
        }
        walk
    }

    fn gaussian_smooth(data: &[f64], sigma: f64) -> Vec<f64> {
        if data.is_empty() {
            return Vec::new();
        }
        if sigma <= 0.0 {
            return data.to_vec();
        }

        let size = (sigma * 3.0).ceil() as i32;
        let mut smoothed = Vec::with_capacity(data.len());

        for i in 0..data.len() {
            let mut value = 0.0;
            let mut weight_sum = 0.0;
            for j in -size..=size {
                let idx = (i as i32 + j).clamp(0, data.len() as i32 - 1) as usize;
                let weight = (-(j as f64).powi(2) / (2.0 * sigma.powi(2))).exp();
                value += data[idx] * weight;
                weight_sum += weight;
            }
            smoothed.push(value / weight_sum);
        }

        smoothed
    }

    fn morph_distribution(data: &[f64], target_mean: f64, target_std: f64) -> Vec<f64> {
        if data.is_empty() {
            return Vec::new();
        }

        let mean = data.iter().sum::<f64>() / data.len() as f64;
        let variance = data.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / data.len() as f64;
        let std = variance.sqrt();

        if std <= f64::EPSILON {
            return vec![target_mean; data.len()];
        }

        data.iter()
            .map(|v| ((v - mean) / std) * target_std + target_mean)
            .collect()
    }

    fn bezier_curve(p0: f64, p1: f64, p2: f64, t: f64) -> f64 {
        (1.0 - t).powi(2) * p0 + 2.0 * (1.0 - t) * t * p1 + t.powi(2) * p2
    }

    pub fn generate_gaussian_mouse_movements(
        start_x: i32,
        start_y: i32,
        end_x: i32,
        end_y: i32,
        config: GaussianConfig,
    ) -> Vec<(i32, i32)> {
        let num_points = ((config.duration.max(0.0) * 60.0).round() as usize).max(2);

        let stddev = config.randomness.max(0.0) * 10.0;
        let random_x = Self::random_walk(num_points, stddev);
        let random_y = Self::random_walk(num_points, stddev);

        let smooth_x = Self::gaussian_smooth(&random_x, config.smoothness.max(0.0));
        let smooth_y = Self::gaussian_smooth(&random_y, config.smoothness.max(0.0));

        let delta_x = (end_x - start_x) as f64;
        let delta_y = (end_y - start_y) as f64;
        let human_mean_x = delta_x / 2.0;
        let human_std_x = delta_x / 6.0;
        let human_mean_y = delta_y / 2.0;
        let human_std_y = delta_y / 6.0;

        let morphed_x = Self::morph_distribution(&smooth_x, human_mean_x, human_std_x);
        let morphed_y = Self::morph_distribution(&smooth_y, human_mean_y, human_std_y);

        let mut rng = rand::thread_rng();
        let control_x = if start_x <= end_x {
            rng.gen_range(start_x..=end_x) as f64
        } else {
            rng.gen_range(end_x..=start_x) as f64
        };
        let control_y = if start_y <= end_y {
            rng.gen_range(start_y..=end_y) as f64
        } else {
            rng.gen_range(end_y..=start_y) as f64
        };

        let mut path = Vec::with_capacity(num_points);
        for i in 0..num_points {
            let t = i as f64 / (num_points - 1) as f64;
            let bx = Self::bezier_curve(start_x as f64, control_x, end_x as f64, t);
            let by = Self::bezier_curve(start_y as f64, control_y, end_y as f64, t);
            let x = (bx + morphed_x[i]).round() as i32;
            let y = (by + morphed_y[i]).round() as i32;
            path.push((x, y));
        }

        if let Some(first) = path.first_mut() {
            *first = (start_x, start_y);
        }
        if let Some(last) = path.last_mut() {
            *last = (end_x, end_y);
        }

        path
    }

    pub fn generate_coordinates(from_x: i32, from_y: i32, to_x: i32, to_y: i32) -> Vec<(i32, i32)> {
        Self::generate_gaussian_mouse_movements(from_x, from_y, to_x, to_y, GaussianConfig::default())
    }

    pub fn generate_random_coordinates(viewport_width: i32, viewport_height: i32) -> Vec<(i32, i32)> {
        let mut rng = rand::thread_rng();
        let end_x = rng.gen_range(0..=viewport_width.max(0));
        let end_y = rng.gen_range(0..=viewport_height.max(0));
        Self::generate_gaussian_mouse_movements(0, 0, end_x, end_y, GaussianConfig::default())
    }

    pub fn generate_scroll_coordinates(start_y: i32, end_y: i32) -> Vec<(i32, i32)> {
        let mut movements =
            Self::generate_gaussian_mouse_movements(0, start_y, 0, end_y, GaussianConfig::default());
        if movements.last().copied() != Some((0, end_y)) {
            movements.push((0, end_y));
        }
        movements
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_coordinates_endpoints() {
        let coords = GaussianMouse::generate_coordinates(10, 20, 800, 600);
        assert!(coords.len() >= 2);
        assert_eq!(coords[0], (10, 20));
        assert_eq!(*coords.last().unwrap(), (800, 600));
    }

    #[test]
    fn test_generate_random_coordinates_starts_at_origin() {
        let coords = GaussianMouse::generate_random_coordinates(1920, 1080);
        assert!(!coords.is_empty());
        assert_eq!(coords[0], (0, 0));
    }

    #[test]
    fn test_generate_scroll_coordinates_end() {
        let coords = GaussianMouse::generate_scroll_coordinates(0, 1000);
        assert!(!coords.is_empty());
        assert_eq!(*coords.last().unwrap(), (0, 1000));
    }
}
