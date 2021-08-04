/// https://en.wikipedia.org/wiki/Collatz_conjecture
fn collatz(mut a: usize) -> impl Iterator<Item = usize> + Clone {
    std::iter::from_fn(move || {
        //Base case
        if a == 1 {
            None
        } else {
            let temp = a;
            a = if a % 2 == 0 { a / 2 } else { 3 * a + 1 };
            Some(temp)
        }
    })
}

// PIPE me to a file!
fn main() {
    let mut p = poloto::plot("collatz", "x", "y");

    for i in 1000..1006 {
        p.line(poloto::formatm!("c({})", i), collatz(i).enumerate());
    }

    p.simple_theme_dark(poloto::upgrade_write(std::io::stdout()));
}
