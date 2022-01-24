use poloto::prelude::*;
///This displays the same exact image as in the `custom_style.rs`,
///with the exception that the styles are not in the svg itself.
///The svg is embedded in the html, and we can modify its style by
///using styles that override the svg's style.
fn main() {
    let mut s = poloto::plot(
        "Demo: you can use CSS patterns if you embed SVG!",
        "x",
        "y",
        f64::ctx(),
        f64::ctx(),
    );

    let x = (0..50).map(|x| (x as f64 / 50.0) * 10.0);

    s.line("cos", x.clone().map(|x| [x, x.cos()]));
    s.histogram("sin-10", x.clone().step_by(3).map(|x| [x, x.sin() - 10.]));

    println!(
        r###"
<html>
{0}
<button type="button" style="font-size: 24px;" onclick="monclick();">Remove Background Dynamically</button>
<svg width="0" height="0" viewBox="0 0 0 0">
    <defs>
        <pattern id="pattern2" patternUnits="userSpaceOnUse" width="10" height="10">
            <line x1="0" y1="5" x2="10" y2="5" stroke="red" stroke-width="5"/>
        </pattern> 
    </defs>
</svg>
<div id="test" class="phase1">
{1}
</div>
</html>
        "###,
        HEADER,
        poloto::disp(move |a| s.simple_theme(a))
    );
}

const HEADER: &'static str = r###"
<header>
<script>
    var flip=true;
    function monclick(){
        if(flip){
            var foo=document.getElementsByClassName('poloto')[0];
        
            foo.classList.remove('poloto');
            foo.classList.add('poloto2');
        }else{
            var foo=document.getElementsByClassName('poloto2')[0];
        
            foo.classList.remove('poloto2');
            foo.classList.add('poloto');
        }
        flip=!flip;   
    }
</script>
<style>
    body {
        background-color: coral;
    }
    .poloto_text2{
        fill:white;
    }
    .poloto2{
        stroke-linecap:round;
        stroke-linejoin:round;
        font-family: 'Tahoma', sans-serif;
        stroke-width:2;
        fill:green;
    }
    
    .poloto0stroke.poloto0stroke{
        stroke-dasharray:10 2 2;
    }
    .poloto1fill.poloto1fill{
        fill: url(#pattern2);
    }
</style>
</header>
"###;
