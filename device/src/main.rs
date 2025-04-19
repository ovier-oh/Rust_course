use plotters::prelude::*;
use std::f64::consts::E;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parámetros físicos
    let is = 1e-12; // Corriente de saturación (A)
    let q = 1.602e-19; // Carga del electrón (C)
    let k = 1.381e-23; // Constante de Boltzmann (J/K)
    let t = 300.0; // Temperatura en Kelvin
    let n = 1.0; // Factor idealidad

    // Rango de voltaje
    let voltajes: Vec<f64> = (-100..800).map(|x| x as f64 / 1000.0).collect();

    // Calcula corriente para cada voltaje
    let datos_iv: Vec<(f64, f64)> = voltajes
        .iter()
        .map(|&v| {
            let i = is * ((E.powf((q * v) / (n * k * t))) - 1.0);
            (v, i)
        })
        .collect();

    // Crear el gráfico
    let root = BitMapBackend::new("curva_iv.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Curva I-V del Diodo", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(0.0..0.8, 0.0..1e-2)?;

    chart.configure_mesh()
        .x_desc("Voltaje (V)")
        .y_desc("Corriente (A)")
        .draw()?;

    chart.draw_series(LineSeries::new(
        datos_iv.into_iter(),
        &RED,
    ))?;

    println!("¡Gráfico guardado como curva_iv.png!");
    Ok(())
}
