use std::iter::Flatten;

use super::*;

use super::marker::Area;



impl<P:IntoPlotIterator<L=L>, L: Point> IntoPlotIterator for Vec<P> {
    type L = L;
    type P = Flatten<std::vec::IntoIter<P::P>>;
    fn into_plot(self) -> PlotRes<Self::P,Self::L>  {
        
        let (areas,its): (Vec<_>, Vec<_>)=self.into_iter().map(|x|{
            let PlotRes{area,it}=x.into_plot();
            (area,it)
        }).unzip();

        let mut area = Area::new();
        for a in areas {
            area.grow_area(&a);
        }

        let it = its.into_iter().flatten();

        PlotRes{area,it}
    }
}



impl<const K:usize,P:IntoPlotIterator<L=L>, L: Point> IntoPlotIterator for[P;K] {
    type L = L;
    type P = Flatten<std::vec::IntoIter<P::P>>;
    fn into_plot(self) -> PlotRes<Self::P,Self::L>  {
        
        let (areas,its): (Vec<_>, Vec<_>)=self.into_iter().map(|x|{
            let PlotRes{area,it}=x.into_plot();
            (area,it)
        }).unzip();

        let mut area = Area::new();
        for a in areas {
            area.grow_area(&a);
        }

        let it = its.into_iter().flatten();

        PlotRes{area,it}
    }
}


