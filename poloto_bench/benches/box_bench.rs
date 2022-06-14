use criterion::{black_box, criterion_group, criterion_main, Criterion};
use poloto::prelude::*;

struct EmptyWriter;
impl std::fmt::Write for EmptyWriter {
    fn write_str(&mut self, a: &str) -> std::fmt::Result {
        black_box(a);
        Ok(black_box(()))
    }
}
fn trig(writer: impl std::fmt::Write, steps: usize) -> std::fmt::Result {
    let x = (0..steps).map(move |x| (x as f64 / steps as f64) * 10.0);

    poloto::quick_fmt!(
        "trig",
        "x",
        "y",
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
    )
    .simple_theme(writer)
}

fn boxed_trig(writer: impl std::fmt::Write, steps: usize) -> std::fmt::Result {
    let x = (0..steps).map(move |x| (x as f64 / steps as f64) * 10.0);

    poloto::quick_fmt!(
        "box trig",
        "x",
        "y",
        poloto::build::BoxedPlot::new(poloto::build::line(
            "tan(x)",
            poloto::build::buffered_iter::buffered(
                x.clone()
                    .map(|x| [x, x.tan()])
                    .crop_above(10.0)
                    .crop_below(-10.0)
                    .crop_left(2.0),
            ),
        )),
        poloto::build::BoxedPlot::new(poloto::build::line(
            "sin(2x)",
            poloto::build::bounded_iter::from_rect(
                [0.0, 10.0],
                [0.0, 10.0],
                x.clone().map(|x| [x, (2.0 * x).sin()]),
            ),
        )),
        poloto::build::BoxedPlot::new(poloto::build::line(
            "2*cos(x)",
            poloto::build::buffered_iter::buffered(
                x.clone().map(|x| [x, 2.0 * x.cos()]).crop_above(1.4),
            ),
        )),
        poloto::build::BoxedPlot::new(poloto::build::line(
            "2*cos(x)",
            x.clone().map(|x| [x, 2.0 * x.cos()]).crop_above(1.4),
        ))
    )
    .simple_theme(writer)
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let num = 5_000;
    c.bench_function("trig", |b| {
        b.iter(|| {
            let mut s = EmptyWriter;
            trig(&mut s, black_box(num)).unwrap();
            black_box(s);
        })
    });
    c.bench_function("boxed trig", |b| {
        b.iter(|| {
            let mut s = EmptyWriter;
            boxed_trig(&mut s, black_box(num)).unwrap();
            black_box(s);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
