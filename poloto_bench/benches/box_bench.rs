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

    let p = poloto::plots!(
        poloto::buffered_plot(
            x.zip_output(f64::tan)
                .crop_above(10.0)
                .crop_below(-10.0)
                .crop_left(2.0)
        )
        .line("tan(x)"),
        poloto::buffered_plot(x.clone().map(|x| [x, 2.0 * x.cos()]).crop_above(1.4))
            .line("2*cos(x)"),
        poloto::cloned_plot(x.clone().map(|x| [x, 2.0 * x.cos()]).crop_above(1.4)).line("2*cos(x)")
    );

    poloto::data(p)
        .build()
        .labels("trig", "x", "y")
        .append_to(poloto::simple_light())
        .render_fmt_write(writer)
}

fn boxed_trig(writer: impl std::fmt::Write, steps: usize) -> std::fmt::Result {
    let x = (0..steps).map(move |x| (x as f64 / steps as f64) * 10.0);

    let p = poloto::plots!(
        poloto::build::BoxedPlot::new(
            poloto::buffered_plot(
                x.zip_output(f64::tan)
                    .crop_above(10.0)
                    .crop_below(-10.0)
                    .crop_left(2.0)
            )
            .line("tan(x)"),
        ),
        poloto::build::BoxedPlot::new(
            poloto::buffered_plot(x.clone().map(|x| [x, 2.0 * x.cos()]).crop_above(1.4))
                .line("2*cos(x)"),
        ),
        poloto::build::BoxedPlot::new(
            poloto::cloned_plot(x.clone().map(|x| [x, 2.0 * x.cos()]).crop_above(1.4))
                .line("2*cos(x)"),
        )
    );

    poloto::data(p)
        .build()
        .labels("box trig", "x", "y")
        .append_to(poloto::simple_light())
        .render_fmt_write(writer)
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
