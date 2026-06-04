use plotters::{
    backend::BitMapBackend,
    chart::ChartBuilder,
    prelude::IntoDrawingArea,
    series::LineSeries,
    style::full_palette::{RED, WHITE},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("images/quadratic.png", (1024, 768)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .build_cartesian_2d(-5.0..5.0, -1.0..30.0)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            (-500..=500).map(|x| {
                let x = x as f64 / 100.0;
                (x, x * x)
            }),
            &RED,
        ))?;
    Ok(())
}
