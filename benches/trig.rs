use criterion::{black_box, criterion_group, criterion_main, Criterion};
use poloto::prelude::*;

fn trig(steps: usize) -> poloto::render::Plotter<impl poloto::render::Disp> {
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

    let plotter = data.build().stage().plot(
        "Some Trigonometry Plots 🥳",
        formatm!("This is the {} label", 'x'),
        "This is the y label",
    );

    plotter
}

fn boxed_trig(steps: usize) -> poloto::render::Plotter<impl poloto::render::Disp> {
    let x = (0..steps).map(move |x| (x as f64 / steps as f64) * 10.0);

    // Using poloto::Croppable, we can filter out plots and still have discontinuity.
    let data = vec![
        poloto::build::box_plot(poloto::build::line(
            "tan(x)",
            poloto::build::buffered_iter::buffered(
                x.clone()
                    .map(|x| [x, x.tan()])
                    .crop_above(10.0)
                    .crop_below(-10.0)
                    .crop_left(2.0),
            ),
        )),
        poloto::build::box_plot(poloto::build::line(
            "sin(2x)",
            poloto::build::bounded_iter::from_rect(
                [0.0, 10.0],
                [0.0, 10.0],
                x.clone().map(|x| [x, (2.0 * x).sin()]),
            ),
        )),
        poloto::build::box_plot(poloto::build::line(
            "2*cos(x)",
            poloto::build::buffered_iter::buffered(
                x.clone().map(|x| [x, 2.0 * x.cos()]).crop_above(1.4),
            ),
        )),
        poloto::build::box_plot(poloto::build::line(
            "2*cos(x)",
            x.clone().map(|x| [x, 2.0 * x.cos()]).crop_above(1.4),
        )),
    ];

    let plotter = poloto::build::plots_dyn(data).build().stage().plot(
        "Some Trigonometry Plots 🥳",
        formatm!("This is the {} label", 'x'),
        "This is the y label",
    );

    plotter
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let num = 3_000;
    c.bench_function("trig", |b| b.iter(|| black_box(trig(black_box(num)))));
    c.bench_function("boxed trig", |b| {
        b.iter(|| black_box(boxed_trig(black_box(num))))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
