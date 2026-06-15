use plotters::{
    backend::BitMapBackend,
    drawing::IntoDrawingArea,
    chart::ChartBuilder,
    series::LineSeries,
    style::{
        AsRelative,
        full_palette::{RED, WHITE},
    },
};

const SAMPLES_PER_UNIT: f64 = 100.0;

fn plot_points<F>(start: f64, end: f64, func: F) -> impl Iterator<Item = (f64, f64)>
where
    F: Fn(f64) -> f64,
{
    let steps = ((end - start) * SAMPLES_PER_UNIT) as u64;
    (0..=steps).map(move |i| {
        let x = start + i as f64 / SAMPLES_PER_UNIT;
        (x, func(x))
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("images/three_plots.png", (1024, 768)).into_drawing_area();

    root.fill(&WHITE)?;

    let (upper, lower) = root.split_vertically((50).percent_height());
    let (upper_left, upper_right) = upper.split_horizontally((50).percent_width());

    // Upper left linear chart
    let mut upper_left_chart = ChartBuilder::on(&upper_left)
        .caption("Linear", ("Arial", 30))
        .margin(10)
        .set_left_and_bottom_label_area_size(30)
        .build_cartesian_2d(-5.0..5.0, -5.0..5.0)?;

    upper_left_chart.configure_mesh().draw()?;
    upper_left_chart.draw_series(LineSeries::new(plot_points(-5.0, 5.0, |x| x), &RED))?;

    // Upper right quadratic chart
    let mut upper_right_chart = ChartBuilder::on(&upper_right)
        .caption("Quadratic", ("Arial", 30))
        .margin(10)
        .set_left_and_bottom_label_area_size(30)
        .build_cartesian_2d(-5.0..5.0, -1.0..30.0)?;

    upper_right_chart.configure_mesh().draw()?;
    upper_right_chart.draw_series(LineSeries::new(plot_points(-5.0, 5.0, |x| x * x), &RED))?;

    // Lower chart sinusoidal
    let mut lower_chart = ChartBuilder::on(&lower)
        .caption("Sinusoid", ("Arial", 30))
        .margin(10)
        .set_left_and_bottom_label_area_size(30)
        .build_cartesian_2d(-5.0..20.0, -1.2..1.2)?;

    lower_chart
        .configure_mesh()
        .x_desc("Radians")
        .axis_desc_style(("sans-serif", 20))
        .draw()?;
    lower_chart.draw_series(LineSeries::new(plot_points(-5.0, 20.0, f64::sin), &RED))?;

    Ok(())
}
