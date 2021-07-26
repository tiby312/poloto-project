fn gaussian(a:f64,b:f64,c:f64)->impl Fn(f64)->f64{
    move |x|{
        a*( -((x-b)*(x-b)) /(2.0*c*c) ).exp()
    }
}

// PIPE me to a file!
fn main() {
    
    let a=gaussian(10.0,0.0,0.3);
    let b=gaussian(05.0,0.0,0.2);
    let c=gaussian(15.0,0.0,0.1);

    let range=(0..10000).map(|x|x as f64/10000.0).map(|x|x*2.0-1.0);

    let mut s = poloto::plot("gaussian", "x", "y");
   
    s.line("a=10.0 c=0.3",range.clone().map(|x|[x,a(x)]));
    s.line("a= 5.0 c=0.2",range.clone().map(|x|[x,c(x)]));
    s.line("a=15.0 c=0.1",range.clone().map(|x|[x,b(x)]));
    
    println!("{}", s.render());
}
