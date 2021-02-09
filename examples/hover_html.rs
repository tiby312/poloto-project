fn main() {
    let mut s = poloto::plot("Demo: you can use CSS patterns if you embed SVG!", "x", "y");

    let x = (0..50).map(|x| (x as f32 / 50.0) * 10.0);

    s.line("cos", x.clone().map(|x| [x, x.cos()]));
    s.histogram("sin-10", x.clone().step_by(3).map(|x| [x, x.sin() - 10.]));

    println!(
        r###"
<html>
{0}
<div id="test" class="navy">
{1}
</div>
</html>
        "###,
        HEADER,
        s.render_to_document()
    );
}

const HEADER: &'static str = r###"
<head>
<style>
body {
    background-color: coral;
}
.poloto0stroke:hover{
    stroke:black;
    stroke-width:5;

}
.poloto1fill:hover{
    stroke:black;
    stroke-width:5;
}
</style>
</head> 
"###;
