use poloto::prelude::*;
fn main() {
    let x = (0..50).map(|x| (x as f64 / 50.0) * 10.0);

    let mut s = poloto::plot(
        "Demo: Hovering and shadows",
        "x",
        "y",
        f64::ctx(),
        f64::ctx(),
    );

    s.line("cos", x.clone().map(|x| [x, x.cos()]));
    s.histogram("sin-3", x.clone().step_by(3).map(|x| [x, x.sin() - 3.]));
    s.scatter("sin", x.clone().step_by(3).map(|x| [x, x.sin()]));

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
        poloto::disp(|a| s.simple_theme(a))
    );
}

const HEADER: &'static str = r###"
<head>
<svg width=0 height=0>
<defs>
<filter id="dropshadow" height="130%">
  <feGaussianBlur in="SourceAlpha" stdDeviation="3"/> <!-- stdDeviation is how much to blur -->
  <feOffset dx="2" dy="2" result="offsetblur"/> <!-- how much to offset -->
  <feComponentTransfer>
    <feFuncA type="linear" slope="0.5"/> <!-- slope is the opacity of the shadow -->
  </feComponentTransfer>
  <feMerge> 
    <feMergeNode/> <!-- this contains the offset blurred image -->
    <feMergeNode in="SourceGraphic"/> <!-- this contains the element that the filter is applied to -->
  </feMerge>
</defs>
</svg>
<style>
body {
    background-color: coral;
}
@keyframes gelatine {
    from, to { transform: scale(1, 1); }
    25% { transform: scale(0.9, 1.1); }
    50% { transform: scale(1.1, 0.9); }
    75% { transform: scale(0.95, 1.05); }
  }
.poloto{
    
    text-shadow: 2px 2px 5px black;
}
.poloto0stroke:hover{
    stroke:black;
    stroke-width:3;
}
.poloto1fill{
    animation: gelatine 3.0s infinite;
    transform-origin: center;
    filter:url(#dropshadow);
}
.poloto1fill:hover{
    
    stroke:black;
    stroke-width:3;
}

.poloto2fill:hover{

    stroke:black;
    stroke-width:3;
}
.poloto2fill{
    filter:url(#dropshadow);
}

</style>
</head> 
"###;
