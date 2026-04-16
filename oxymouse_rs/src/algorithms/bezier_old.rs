/// Rust port of the OxyMouse BezierMouse algorithm.
///
/// Reference (Python):
/// <https://github.com/oxylabs/OxyMouse/blob/f98e4e51bf92530fc87eedf8e2e184d9b2033a2e/oxymouse/algorithms/bezier_mouse/bezier_mouse.py>
use rand::Rng;

/// Rust equivalent of the Python `BezierMouse` class.
pub struct BezierMouse;

impl BezierMouse {
    /// The Bernstein polynomial of degree `n`, index `i`, evaluated at `t`.
    ///
    /// Equivalent to:  `C(n, i) * t^i * (1 - t)^(n - i)`
    pub fn bernstein_poly(i: usize, n: usize, t: f64) -> f64 {
        Self::comb(n, i) as f64 * t.powi(i as i32) * (1.0 - t).powi((n - i) as i32)
    }

    /// Binomial coefficient C(n, k) computed iteratively to stay exact for
    /// the small values of n used in Bézier curve calculations.
    fn comb(n: usize, k: usize) -> u64 {
        if k > n {
            return 0;
        }
        let k = k.min(n - k);
        let mut result: u64 = 1;
        for i in 0..k {
            result = result * (n - i) as u64 / (i + 1) as u64;
        }
        result
    }

    /// Given a set of control points, return the Bézier curve defined by
    /// those points sampled at `num_steps` evenly-spaced values of the
    /// parameter `t ∈ [0, 1]`.
    ///
    /// `points` is a slice of `(x, y)` integer control points, e.g.
    /// `&[(1, 1), (2, 3), (4, 5), (3, 5)]`.
    pub fn bezier_curve(points: &[(i32, i32)], num_steps: usize) -> Vec<(i32, i32)> {
        if points.is_empty() || num_steps == 0 {
            return Vec::new();
        }

        let n = points.len() - 1;
        let xpoints: Vec<f64> = points.iter().map(|p| p.0 as f64).collect();
        let ypoints: Vec<f64> = points.iter().map(|p| p.1 as f64).collect();

        // t values linearly spaced in [0, 1]
        let t_values: Vec<f64> = (0..num_steps)
            .map(|s| s as f64 / (num_steps - 1).max(1) as f64)
            .collect();

        // polynomial_array[i][s] = bernstein_poly(i, n, t_values[s])
        let polynomial_array: Vec<Vec<f64>> = (0..=n)
            .map(|i| {
                t_values
                    .iter()
                    .map(|&t| Self::bernstein_poly(i, n, t))
                    .collect()
            })
            .collect();

        // For each step s: x = dot(xpoints, polynomial_array[:][s])
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

    /// Generate mouse movements using Bézier curves.
    ///
    /// # Parameters
    /// - `start_x` / `start_y`: Starting screen coordinates.
    /// - `end_x`   / `end_y`:   Ending screen coordinates.
    /// - `duration`:   Duration of the movement in seconds (used to derive
    ///                 sample count assuming 60 fps).
    /// - `complexity`: Number of control points (minimum 4, includes start and
    ///                 end — i.e. at least a cubic Bézier).
    /// - `randomness`: Jitter scale for intermediate control points `[0.0, 1.0]`.
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

        let x_min = start_x.min(end_x);
        let x_max = start_x.max(end_x);
        let y_min = start_y.min(end_y);
        let y_max = start_y.max(end_y);

        // Build control-point list: start, (complexity-2) intermediate, end
        let mut control_points: Vec<(i32, i32)> = Vec::with_capacity(complexity);
        control_points.push((start_x, start_y));

        for _ in 0..(complexity - 2) {
            let cx = if x_min < x_max {
                rng.gen_range(x_min..=x_max)
            } else {
                x_min
            };
            let cy = if y_min < y_max {
                rng.gen_range(y_min..=y_max)
            } else {
                y_min
            };
            control_points.push((cx, cy));
        }
        control_points.push((end_x, end_y));

        // Apply random jitter to every intermediate control point
        let jitter = randomness * 100.0;
        let last = control_points.len() - 1;
        for pt in control_points[1..last].iter_mut() {
            pt.0 += rng.gen_range(-jitter..jitter) as i32;
            pt.1 += rng.gen_range(-jitter..jitter) as i32;
        }

        // Assuming 60 fps
        let num_steps = ((duration * 60.0) as usize).max(2);
        Self::bezier_curve(&control_points, num_steps)
    }

    /// Generate a list of coordinates from `(from_x, from_y)` to
    /// `(to_x, to_y)` using Bézier curves with default parameters.
    pub fn generate_coordinates(
        from_x: i32,
        from_y: i32,
        to_x: i32,
        to_y: i32,
    ) -> Vec<(i32, i32)> {
        Self::generate_bezier_mouse_movements(from_x, from_y, to_x, to_y, 1.0, 4, 1.0)
    }

    /// Generate random coordinates within the given viewport dimensions.
    pub fn generate_random_coordinates(
        viewport_width: i32,
        viewport_height: i32,
    ) -> Vec<(i32, i32)> {
        let mut rng = rand::thread_rng();
        let end_x = rng.gen_range(0..=viewport_width);
        let end_y = rng.gen_range(0..=viewport_height);
        Self::generate_bezier_mouse_movements(0, 0, end_x, end_y, 1.0, 4, 1.0)
    }

    /// Generate a list of `(x, y)` coordinates for scrolling from `start_y`
    /// to `end_y` using Bézier curves (x is always 0).
    ///
    /// The final `(0, end_y)` entry is appended to guarantee the path reaches
    /// the exact destination (mirrors the Python reference implementation).
    pub fn generate_scroll_coordinates(start_y: i32, end_y: i32) -> Vec<(i32, i32)> {
        let mut movements =
            Self::generate_bezier_mouse_movements(0, start_y, 0, end_y, 1.0, 4, 1.0);
        movements.push((0, end_y));
        movements
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Bernstein basis polynomials must form a partition of unity for any t.
    #[test]
    fn test_bernstein_poly_partition_of_unity() {
        let n = 4;
        let t = 0.42;
        let sum: f64 = (0..=n).map(|i| BezierMouse::bernstein_poly(i, n, t)).sum();
        assert!((sum - 1.0).abs() < 1e-10, "partition-of-unity failed: {sum}");
    }

    /// B(0, n, 0) == 1 and B(n, n, 1) == 1 (corner cases).
    #[test]
    fn test_bernstein_poly_endpoints() {
        assert!((BezierMouse::bernstein_poly(0, 3, 0.0) - 1.0).abs() < 1e-10);
        assert!((BezierMouse::bernstein_poly(3, 3, 1.0) - 1.0).abs() < 1e-10);
    }

    /// The Bézier curve must start and end exactly at the given control points.
    #[test]
    fn test_bezier_curve_endpoints() {
        let points = vec![(0, 0), (100, 200), (400, 300), (500, 500)];
        let curve = BezierMouse::bezier_curve(&points, 60);
        assert_eq!(curve.len(), 60);
        assert_eq!(curve[0], (0, 0));
        assert_eq!(*curve.last().unwrap(), (500, 500));
    }

    /// `generate_coordinates` should produce a path that starts and ends
    /// at the requested coordinates.
    #[test]
    fn test_generate_coordinates_endpoints() {
        let coords = BezierMouse::generate_coordinates(10, 20, 800, 600);
        assert!(coords.len() >= 2, "expected at least 2 points");
        assert_eq!(coords[0], (10, 20));
        assert_eq!(*coords.last().unwrap(), (800, 600));
    }

    /// `generate_random_coordinates` returns a non-empty path that starts at (0, 0).
    #[test]
    fn test_generate_random_coordinates() {
        let coords = BezierMouse::generate_random_coordinates(1920, 1080);
        assert!(!coords.is_empty());
        assert_eq!(coords[0], (0, 0));
    }

    /// `generate_scroll_coordinates` must end exactly at `end_y`.
    #[test]
    fn test_generate_scroll_coordinates_end() {
        let coords = BezierMouse::generate_scroll_coordinates(0, 1000);
        assert!(!coords.is_empty());
        assert_eq!(*coords.last().unwrap(), (0, 1000));
    }
}
