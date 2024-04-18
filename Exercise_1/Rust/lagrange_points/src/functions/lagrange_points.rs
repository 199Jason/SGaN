extern crate plotters;
extern crate nalgebra as na;
extern crate roots;

use plotters::prelude::*;
use roots::{find_root_brent, SimpleConvergency, SearchError};
use std::ops::FnMut;
use na::DVector;

pub struct Data {
    pub mu: f64,
}

pub struct LagrangePoints {
    pub l1: DVector<f64>,
    pub l2: DVector<f64>,
    pub l3: DVector<f64>,
    pub l4: DVector<f64>,
    pub l5: DVector<f64>,
}

pub fn lagrange_points(data: &Data) -> LagrangePoints {
    let mu = data.mu;

    // Find L1
    let l1 = find_lagrange_point(|x| x - (1.0 - mu) / (x + mu).powi(2) + mu / (x - 1.0 + mu).powi(2)).unwrap();

    // Find L2
    let l2 = find_lagrange_point(|x| x - (1.0 - mu) / (x + mu).powi(2) - mu / (x - 1.0 + mu).powi(2)).unwrap();

    // Find L3
    let l3 = find_lagrange_point(|x| x + (1.0 - mu) / (x + mu).powi(2) + mu / (x - 1.0 + mu).powi(2)).unwrap();

    // Find component of L4
    let l4x = 0.5 - mu;
    let l4y = 0.5 * 3.0f64.sqrt();

    // Find component of L5
    let l5x = 0.5 - mu;
    let l5y = -0.5 * 3.0f64.sqrt();

    LagrangePoints {
        l1: DVector::from_vec(vec![l1, 0.0, 0.0]),
        l2: DVector::from_vec(vec![l2, 0.0, 0.0]),
        l3: DVector::from_vec(vec![l3, 0.0, 0.0]),
        l4: DVector::from_vec(vec![l4x, l4y, 0.0]),
        l5: DVector::from_vec(vec![l5x, l5y, 0.0]),
    }
}

fn find_lagrange_point<F>(f: F) -> Result<f64, SearchError>
where
    F: FnMut(f64) -> f64,
{
    let mut convergency = SimpleConvergency {
        eps: 1e-15f64,
        max_iter: 1000,
    };
    match find_root_brent(-10.0, 10.0, f, &mut convergency) {
        Ok(root) => Ok(root),
        Err(err) => Err(err),
    }
}

// ...

// ...

pub fn plot_lagrange_points(lp: &LagrangePoints, mu: f64) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("lagrange_points.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Lagrange Points", ("Arial", 40).into_font())
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(-2.0..2.0, -1.0..1.0)?;

    chart.configure_mesh().draw()?;

    let style = BLACK.filled();

    // Plot Lagrange points
    chart.draw_series(std::iter::once(
        Circle::new((lp.l1[0], 0.0), 5, style.clone())
    ))?;

    chart.draw_series(std::iter::once(
        Text::new("L1", (lp.l1[0] - 0.1, 0.1), ("Arial", 15).into_font()),
    ))?;

    chart.draw_series(std::iter::once(
        Circle::new((lp.l2[0], 0.0), 5, style.clone())
    ))?;

    chart.draw_series(std::iter::once(
        Text::new("L2", (lp.l2[0] + 0.1, 0.1), ("Arial", 15).into_font()),
    ))?;

    chart.draw_series(std::iter::once(
        Circle::new((lp.l3[0], 0.0), 5, style.clone())
    ))?;

    chart.draw_series(std::iter::once(
        Text::new("L3", (lp.l3[0], -0.05), ("Arial", 15).into_font()),
    ))?;

    chart.draw_series(std::iter::once(
        Circle::new((lp.l4[0], lp.l4[1]), 5, style.clone())
    ))?;

    chart.draw_series(std::iter::once(
        Text::new("L4", (lp.l4[0], lp.l4[1]-0.05), ("Arial", 15).into_font()),
    ))?;

    chart.draw_series(std::iter::once(
        Circle::new((lp.l5[0], lp.l5[1]-0.05), 5, style)
    ))?;

    chart.draw_series(std::iter::once(
        Text::new("L5", (lp.l5[0], lp.l5[1]), ("Arial", 15).into_font()),
    ))?;

    // Plot celestial bodies
    chart.draw_series(std::iter::once(
        Circle::new((-mu, 0.0), 15, RED.filled())
    ))?;

    chart.draw_series(std::iter::once(
        Text::new("Sun", (-mu, -0.05), ("Arial", 10).into_font()),
    ))?;

    chart.draw_series(std::iter::once(
        Circle::new((1.0 - mu, 0.0), 3, BLUE.filled())
    ))?;

    chart.draw_series(std::iter::once(
        Text::new("Earth", (1.0 - mu, -0.05), ("Arial", 10).into_font()),
    ))?;

    Ok(())
}
