use criterion::{black_box, criterion_group, criterion_main, Criterion};
use poloto::prelude::*;

fn trig(
    writer: impl std::fmt::Write,
    canvas: &poloto::render::Canvas,
    steps: usize,
) -> std::fmt::Result {
    let x = (0..steps).map(move |x| (x as f64 / steps as f64) * 10.0);

    // Using poloto::Croppable, we can filter out plots and still have discontinuity.
    let data = plots!(
        poloto::build::line(
            "tan(x)",
            poloto::build::buffered_iter::buffered(
                x.clone()
                    .map(|x| [x, x.tan()])
                    .crop_above(10.0)
                    .crop_below(-10.0)
                    .crop_left(2.0),
            ),
        ),
        poloto::build::line(
            "sin(2x)",
            poloto::build::bounded_iter::from_rect(
                [0.0, 10.0],
                [0.0, 10.0],
                x.clone().map(|x| [x, (2.0 * x).sin()]),
            ),
        ),
        poloto::build::line(
            "2*cos(x)",
            poloto::build::buffered_iter::buffered(
                x.clone().map(|x| [x, 2.0 * x.cos()]).crop_above(1.4),
            ),
        ),
        poloto::build::line(
            "2*cos(x)",
            x.clone().map(|x| [x, 2.0 * x.cos()]).crop_above(1.4),
        )
    );

    let plotter = canvas.build(data).plot(
        "Some Trigonometry Plots 🥳",
        formatm!("This is the {} label", 'x'),
        "This is the y label",
    );

    plotter.simple_theme(writer)
}

fn boxed_trig(
    writer: impl std::fmt::Write,
    canvas: &poloto::render::Canvas,
    steps: usize,
) -> std::fmt::Result {
    let x = (0..steps).map(move |x| (x as f64 / steps as f64) * 10.0);

    // Using poloto::Croppable, we can filter out plots and still have discontinuity.
    let data = vec![
        poloto::build::line(
            "tan(x)",
            poloto::build::buffered_iter::buffered(
                x.clone()
                    .map(|x| [x, x.tan()])
                    .crop_above(10.0)
                    .crop_below(-10.0)
                    .crop_left(2.0),
            ),
        )
        .into_boxed(),
        poloto::build::line(
            "sin(2x)",
            poloto::build::bounded_iter::from_rect(
                [0.0, 10.0],
                [0.0, 10.0],
                x.clone().map(|x| [x, (2.0 * x).sin()]),
            ),
        )
        .into_boxed(),
        poloto::build::line(
            "2*cos(x)",
            poloto::build::buffered_iter::buffered(
                x.clone().map(|x| [x, 2.0 * x.cos()]).crop_above(1.4),
            ),
        )
        .into_boxed(),
        poloto::build::line(
            "2*cos(x)",
            x.clone().map(|x| [x, 2.0 * x.cos()]).crop_above(1.4),
        )
        .into_boxed(),
    ];

    let plotter = canvas.build(poloto::build::plots_dyn(data)).plot(
        "Some Trigonometry Plots 🥳",
        formatm!("This is the {} label", 'x'),
        "This is the y label",
    );

    plotter.simple_theme(writer)
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let num = 3_000;
    let canvas = poloto::render::canvas();
    c.bench_function("trig", |b| {
        b.iter(|| {
            let mut s = String::new();
            trig(&mut s, &canvas, black_box(num)).unwrap();
            black_box(s);
        })
    });
    c.bench_function("boxed trig", |b| {
        b.iter(|| {
            let mut s = String::new();
            boxed_trig(&mut s, &canvas, black_box(num)).unwrap();
            black_box(s);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
