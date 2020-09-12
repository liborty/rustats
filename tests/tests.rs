#![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]

use std::fmt;
use anyhow::{Result};
use rstats::{RStats,MutVectors,Vectors,genvec,green};
use devtimer::DevTime;

/// GreenVec struct facilitates printing (in green) of vectors of any type
/// that has Display implemented.
pub struct GreenVec<T: fmt::Display>(Vec<T>);
impl<T: fmt::Display> fmt::Display for GreenVec<T> {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      let mut stringy = String::from("[");
      let n = self.0.len();
      if n == 0 { return write!(f,"\x1B[01;92m[]\x1B[0m") };
      stringy.push_str(&self.0[0].to_string()); // first item
      for i in 1..n {
         stringy.push_str(", ");
         stringy.push_str(&self.0[i].to_string());
      }       
      write!(f, "\x1B[01;92m{}]\x1B[0m", stringy)
   }
}

#[test]
fn fstats() -> Result<()> { 
   let v1 = vec![1_f64,2.,3.,4.,5.,6.,7.,8.,9.,10.,11.,12.,13.,14.];
   let s1 = v1.as_slice();
   println!("\n{:?}",s1);
   println!("Arithmetic mean:{}",green(s1.amean().unwrap()));
   println!("Geometric mean:\t{}",green(s1.gmean().unwrap()));
   println!("Harmonic mean:\t{}",green(s1.hmean().unwrap()));
   println!("Magnitude:\t{}",green(s1.vmag()));
   println!("Arithmetic\t{}",s1.ameanstd().unwrap());
   println!("Geometric\t{}",s1.gmeanstd().unwrap());
   println!("Autocorrelation:{}",green(s1.autocorr().unwrap()));
   println!("{}",s1.median().unwrap());
   let v2 = vec![1_f64,14.,2.,13.,3.,12.,4.,11.,5.,10.,6.,9.,7.,8.];
   let s2 = v2.as_slice();
   println!("\n{:?}",s2);
   println!("Ranking: {}",GreenVec(s2.ranks().unwrap()));
   println!("Pearson's Correlation:\t{}",green(s1.correlation(s2).unwrap())); 
   println!("Kendall's Correlation:\t{}",green(s1.kendalcorr(s2).unwrap()));  
   println!("Spearman's Correlation:\t{}",green(s1.spearmancorr(s2).unwrap()));     
   println!("Scalar product: {}",green(s1.dotp(s2)));
   println!("Euclidian distance: {}",green(s1.vdist(s2)));
   println!("Magnitude of difference: {}",green(s1.vsub(s2).as_slice().vmag()));   
   println!("Vector difference:\n{}",GreenVec(s1.vsub(s2))); 
   println!("Vector addition:\n{}",GreenVec(s1.vadd(s2)));   
   Ok(())
}

#[test]
fn intstats() -> Result<()> { 
   let v1 = vec![1_i64,2,3,4,5,6,7,8,9,10,11,12,13,14];
   let s1 = v1.as_slice();
   println!("\n{:?}",&s1);
   println!("Arithmetic mean:{}",green(s1.amean().unwrap()));
   println!("Geometric mean:\t{}",green(s1.gmean().unwrap()));
   println!("Harmonic mean:\t{}",green(s1.hmean().unwrap()));
   println!("Arithmetic\t{}",&s1.ameanstd().unwrap());
   println!("Geometric\t{}",s1.gmeanstd().unwrap());
   println!("{}",s1.median().unwrap());   
   Ok(())
}

#[test]
fn multidimensional() -> Result<()> { 
   let d = 5_usize;
   let pt = genvec(d,20,3,13); // random test data 5x20
   let pts = pt.as_slice();
   let (med,medi,outd,outi) = pts.medoid(d).unwrap();
   let centroid = pts.acentroid(d);
   let median = pts.nmedian(d, 1e-5).unwrap();

   println!("\nSum of Outlier distances:\t{} Index: {}",green(outd),green(outi as f64));
   println!("Sum of Medoid distances:\t{} Index: {}",green(med),green(medi as f64));
   println!("Sum of Centroid distances:\t{}",green(pts.distsum(d,&centroid)));
   println!("Sum of Median distances:\t{}\n",green(pts.distsum(d,&median)));

   println!("Outlier eccentricity:\t{}",green(pts.eccentr(d,outi)));
   println!("Medoid ecentricity:\t{}",green(pts.eccentr(d,medi)));
   println!("Centroid ecentricity:\t{}",green(pts.ecc(d,&centroid)));   
   println!("Median eccentricity:\t{}\n",green(pts.ecc(d,&median)));
   println!("MOE (Median of eccentricities)\n{}",pts.moe(d));  
   Ok(())
}
#[test]
fn difficult_data() -> Result<()> {
   let pts:Vec<f64> = vec![0.,0.,0.,0., 1.,0.,0.,0., 0.,1.,0.,0., 0.,0.,1.,0., 0.,0.,0.,1.,
      -1.,0.,0.,0., 0.,-1.,0.,0., 0.,0.,-1.,0., 0.,0.,0.,-1.];
   let gm = pts.as_slice().nmedian(4, 1e-5).unwrap();
   println!("\nMedian residual error: {}",green(pts.as_slice().ecc(4,&gm)));
   Ok(())
}

#[test]
fn medians() -> Result<()> {
   const ITERATIONS:u32 = 10;
   let mut sumg = 0_f64;
   let mut sumtime = 0_u128;
   let mut timer = DevTime::new_simple();

   for i in 1..ITERATIONS {
      let pts = genvec(2,7000,i,2*i);
      timer.start();
      let gm = pts.as_slice().nmedian(2, 1e-5).unwrap();
      timer.stop();
      sumtime += timer.time_in_nanos().unwrap();
      sumg += pts.as_slice().ecc(2,&gm);
   }
   // timer.stop();
   println!("\nSum of {} medians residual errors {} time in ns: {}",
      ITERATIONS,green(sumg),sumtime);     
   Ok(())  
}
