use poloto::prelude::*;
fn main() -> core::fmt::Result {
    let mut s = poloto::plot_io("cows per year","year","cow");

    // TEST 3
    let data = [[0.000001, 0.000001], [0.000001000000001, 0.000001000000001]];

    s.scatter(wr!("data"), data.iter().map(|x| *x).twice_iter());

    s.render_io(std::io::stdout())?;

    Ok(())
}
