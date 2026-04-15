use plotters::prelude::*;

pub fn visualize_movements(movements: &[(i32, i32)], file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create a 1920x1080 canvas (matching your screen size)
    let root = BitMapBackend::new(file_name, (1920, 1080)).into_drawing_area();
    root.fill(&WHITE)?;

    // 2. Build the chart. We flip the Y axis because in screens (0,0) is top-left,
    // but in graphs (0,0) is usually bottom-left.
    let mut chart = ChartBuilder::on(&root)
        .caption("Mouse Bezier Path", ("sans-serif", 50).into_font())
        .margin(10)
        .build_cartesian_2d(0..1920, 1080..0)?; // Note: 1080..0 flips the Y axis

    chart.configure_mesh().draw()?;

    // 3. Draw the path as a line series
    chart.draw_series(LineSeries::new(
        movements.iter().map(|&(x, y)| (x, y)),
        &RED,
    ))?
        .label("Mouse Path")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    // 4. Draw dots for the points so you can see the "steps"
    chart.draw_series(movements.iter().map(|&(x, y)| {
        Circle::new((x, y), 2, RED.filled())
    }))?;

    root.present()?;
    println!("📈 Plot saved to {}", file_name);
    Ok(())
}