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
        x.zip_output(f64::tan)
            .crop_above(10.0)
            .crop_below(-10.0)
            .crop_left(2.0)
            .buffered_plot()
            .line("tan(x)"),
        x.clone()
            .map(|x| [x, (2.0 * x).sin()])
            .rect_bound_plot((0.0, 0.0), (10.0, 10.0))
            .line("sin(2x)"),
        x.clone()
            .map(|x| [x, 2.0 * x.cos()])
            .crop_above(1.4)
            .buffered_plot()
            .line("2*cos(x)"),
        x.clone()
            .map(|x| [x, 2.0 * x.cos()])
            .crop_above(1.4)
            .cloned_plot()
            .line("2*cos(x)")
    )
    .simple_theme(writer)
}

fn boxed_trig(writer: impl std::fmt::Write, steps: usize) -> std::fmt::Result {
    let x = (0..steps).map(move |x| (x as f64 / steps as f64) * 10.0);

    poloto::quick_fmt!(
        "box trig",
        "x",
        "y",
        poloto::build::BoxedPlot::new(
            x.zip_output(f64::tan)
                .crop_above(10.0)
                .crop_below(-10.0)
                .crop_left(2.0)
                .buffered_plot()
                .line("tan(x)"),
        ),
        poloto::build::BoxedPlot::new(
            x.clone()
                .map(|x| [x, (2.0 * x).sin()])
                .rect_bound_plot([0.0, 0.0], [10.0, 10.0])
                .line("sin(2x)"),
        ),
        poloto::build::BoxedPlot::new(
            x.clone()
                .map(|x| [x, 2.0 * x.cos()])
                .crop_above(1.4)
                .buffered_plot()
                .line("2*cos(x)"),
        ),
        poloto::build::BoxedPlot::new(
            x.clone()
                .map(|x| [x, 2.0 * x.cos()])
                .crop_above(1.4)
                .cloned_plot()
                .line("2*cos(x)"),
        )
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
