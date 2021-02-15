///This displays the same exact image as in the `custom_style.rs`,
///with the exception that the styles are not in the svg itself.
///The svg is embeded in the html, and we can modify its style by
///using styles that override the svg's style.
fn main() {
    let mut fs=String::new();
    
    let mut s = poloto::plot(&mut fs,"Demo: you can use CSS patterns if you embed SVG!", "x", "y");

    let x = (0..50).map(|x| (x as f32 / 50.0) * 10.0);

    s.line("cos", x.clone().map(|x| [x, x.cos()]));
    s.histogram("sin-10", x.clone().step_by(3).map(|x| [x, x.sin() - 10.]));
    s.render();
    println!(
        r###"
<html>
{0}
<button type="button" style="font-size: 24px;" onclick="monclick();">Remove Background Dynamically</button>
<svg width="0" height="0" viewBox="0 0 0 0">
    <defs>
        <pattern id="pattern" patternUnits="userSpaceOnUse" width="10" height="10">
            <circle cx="5" cy="5" r="5" fill="black" fill-opacity="0.2"/>
        </pattern>
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
        fs
    );
}

const HEADER: &'static str = r###"
<header>
<script>
    var flip=true;
    function monclick(){
        if(flip){
            var foo=document.getElementsByClassName('poloto_background')[0];
        
            foo.classList.remove('poloto_background');
            foo.classList.add('poloto_background2');
        }else{
            var foo=document.getElementsByClassName('poloto_background2')[0];
        
            foo.classList.remove('poloto_background2');
            foo.classList.add('poloto_background');
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
    .poloto_background2{
        fill:green;
    }
    .poloto_background.poloto_background{
        fill: url(#pattern);
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
