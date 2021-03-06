#![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]

use devtimer::DevTime;
use anyhow::Result;
use indxvec::{wv,wi,Indices,merge::*};

use rstats::{Stats,MutVectors,Vecf64,VecVecf64,Vecu8,VecVecu8};
use rstats::functions::{genvec,genvecu8};

pub const EPS:f64 = 1e-7;
#[test]
fn u8() -> Result<()> {
   let v1 = vec![1_u8,2,2,3,3,3,4,4,4,4,5,5,5,5,5,6,6,6,6,6,6]; 
   println!("\n{}",wv(&v1));
   println!("Entropy: {}",wi(&v1.entropy()));
   let v2 = vec![1_u8,2,2,3,3,3,4,4,4,4,3,3,3,3,3,3,2,2,2,2,2]; 
   println!("{:?}",v2);
   println!("|v2-v1|: {}",wi(&v2.vdistu8(&v1)));
   println!("Cityblockd: {}",wi(&v2.cityblockdu8(&v1)));  
   println!("Entropy: {}",wi(&v2.entropy()));
   println!("Joint E: {}",wi(&v1.jointentropy(&v2)));
   println!("Dependence: {}",wi(&v1.dependence(&v2)));
   let d = 5_usize;
   let n = 7_usize;
   println!("Testing on a random set of {} points in {} d space\n",wi(&n),wi(&d));
   let pt = genvecu8(d,n,5,7); // random test data 
   let cov = pt.covar(&pt.acentroid());
   let com = pt.covar(&pt.gmedian(EPS));
   println!("Covariances:\n{}",wv(&cov));
   println!("Comediances:\n{}",wv(&com));
   println!("Cityblock Distance: {}\n",wi(&cov.cityblockd(&com)));
   Ok(())
}

#[test]
fn fstats() -> Result<()> { 
   let v0 = vec![1_u8,2,3,4,5,6,7,8,9,10,11,12,13,14,15];
   let v1 = v0.vecu8asvecf64(); // testing the cast
   println!("\n{:?}",v1);
   println!("Linear transform:\n{}",wv(&v1.lintrans()));
   println!("Arithmetic mean:{}",wi(&v1.amean().unwrap()));
   println!("Geometric mean:\t{}",wi(&v1.gmean().unwrap()));
   println!("Harmonic mean:\t{}",wi(&v1.hmean().unwrap()));
   println!("Magnitude:\t{}",wi(&v1.vmag()));
   println!("Arithmetic {}",wi(&v1.ameanstd().unwrap()));
   println!("Geometric  {}",wi(&v1.gmeanstd().unwrap()));
   println!("Autocorrelation:{}",wi(&v1.autocorr()));
   println!("{}\n",v1.median().unwrap());
   Ok(())
}
#[test]
fn intstats() -> Result<()> { 
   let v1 = vec![1_i64,2,3,4,5,6,7,8,9,10,11,12,13,14,15];
   println!("\n{:?}",v1);
   println!("Arithmetic mean:{}",wi(&v1.amean().unwrap()));
   println!("Geometric mean:\t{}",wi(&v1.gmean().unwrap()));
   println!("Harmonic mean:\t{}",wi(&v1.hmean().unwrap()));
   println!("Arithmetic {}",wi(&v1.ameanstd().unwrap()));
   println!("Geometric {}",wi(&v1.gmeanstd().unwrap()));
   println!("{}\n",v1.median().unwrap()); 
   Ok(())
}
#[test]
fn vecf64() -> Result<()> { 
   let v1 = vec![1_f64,2.,3.,4.,5.,6.,7.,8.,9.,10.,11.,12.,13.,14.,15.];
   println!("\n{}",wv(&v1));
   let v2 = vec![1_f64,14.,2.,13.,3.,12.,4.,11.,5.,10.,6.,9.,7.,8.,15.];
   println!("{}",wv(&v2)); 
   println!("Pearson's Correlation:\t{}",wi(&v1.correlation(&v2))); 
   println!("Kendall's Correlation:\t{}",wi(&v1.kendalcorr(&v2)));  
   println!("Spearman's Correlation:\t{}",wi(&v1.spearmancorr(&v2)));  
   println!("Cosine:\t\t\t{}",wi(&v1.cosine(&v2))); 
   println!("Cosine of ranks:\t{}",
        wi(&rank(&v1,true).indx_to_f64().cosine(&rank(&v2,true).indx_to_f64())));        
   println!("Euclidian distance:\t{}",wi(&v1.vdist(&v2)));
   println!("Difference magnitude:\t{}",wi(&v1.vsub(&v2).vmag()));   
   println!("Vector difference{}",wv(&v1.vsub(&v2))); 
   println!("Vector sum:{}",wv(&v1.vadd(&v2)));  
   println!("Scalar product:\t\t{}",wi(&v1.dotp(&v2)));
   println!("Parallelogram area:\t{}",wi(&v1.varea(&v2))); 
   println!("Similarity:\t\t{}",wi(&v1.vsim(&v2)));
   println!("Dissimilarity:\t\t{}\n",wi(&v1.vdisim(&v2))); 
   // println!("Arc area:\t\t{}\n",wi(v1.varc(&v2)));
   let sm = v1.symmatrix();
   for i in 0..5 { eprintln!("{}",wv(&sm[i])) };  
   Ok(())
}
#[test]
fn vecvec() -> Result<()> { 
   let d = 10_usize;
   let n = 101_usize;
   println!("testing on a random set of {} points in {} dimensional space",wi(&n),wi(&d));
   let pt = genvec(d,n,5,17); // random test data 
    //  let ptu8 = genvecu8(d,n,5,17); // random u8 dataset 
   let (med,medi,outd,outi) = pt.medoid();
   let (mede,medei,oute,outei) = pt.emedoid(EPS);
   let hcentroid = pt.hcentroid();
   let acentroid = pt.acentroid(); 
   let firstp = pt.firstpoint();
   let median = pt.gmedian(EPS);
   let outlier = &pt[outi]; 
   let eoutlier = &pt[outei];
   let zmed = pt.translate(&median); // zero median transformed data
  
   println!("\nSum of Outlier distances:\t{} Index: {}",wi(&outd),wi(&outi));
   println!("E-Outlier's distances:\t\t{}",wi(&pt.distsuminset(outei)));   
   println!("Outlier's distance to Median:\t{}",wi(&outlier.vdist(&median)));
   println!("E-Outlier's distance to Median:\t{}",wi(&eoutlier.vdist(&median)));      
   println!("Sum of Medoid's distances:\t{} Index: {}",wi(&med),wi(&medi));
   println!("Sum of HCentroid's distances:\t{}",wi(&pt.distsum(&hcentroid)));
   println!("Sum of ACentroid's distances:\t{}",wi(&pt.distsum(&acentroid)));  
   println!("Sum of Median's distances:\t{}",wi(&pt.distsum(&median)));
   let dists = pt.distsums();
   println!("Distances\t{}",dists.ameanstd().unwrap());
   println!("Distances\t{}\n",dists.median().unwrap());

   println!("Outlier's approx eccentricity:\t{}",wi(&pt.eccmember(outi).vmag()));
   println!("E-Outlier's eccentricity:\t{} Index: {}",wi(&oute),wi(&outei));
   println!("E-Medoid's eccentricity:\t{} Index: {}",wi(&mede),wi(&medei));
   println!("Centroid's approx eccentricity:\t{}",wi(&pt.eccnonmember(&acentroid).vmag()));
   println!("Firstpoint's app eccentricity:\t{}",wi(&pt.eccnonmember(&firstp).vmag()));
   println!("Median's ecc (passed epsilon):\t{}",wi(&pt.eccnonmember(&median).vmag()));
   println!("Median's error:\t{}",wi(&zmed.gmedian(EPS).vmag()));
   let (mu,eccmed) = pt.moe(EPS);
   println!("Eccentricities\t{}",mu);  
   println!("Eccentricities\t{}",eccmed);
   let (_, seccs) = pt.sortedeccs(true,EPS); 
   println!("Sorted eccs: {}\n", wv(&seccs));
   let medcnt = binsearch(&seccs,eccmed.median);
   println!("Items smaller or equal to median of eccs: {} last value: {}", wi(&medcnt), wi(&seccs[medcnt-1]));
   let mut weights = Vec::new();
   for i in 1..n+1 { weights.push(i as f64) }; // create test weights data
// create pretend median of medians
//   let medmed = vec![0.5_f64;n];
//   let (se, cpdf) = 
//    ptu8.wsortedcos( &medmed, &pt.wgmedian(&weights,EPS), &weights);
//   println!("Sorted coses:\n{}\ncpdf:\n{}\n",wv(se),wv(cpdf));
   Ok(())
}

#[test]
/// Trend between two data sets in space of the same dimensions but 
/// numbers of points can differ
fn trend() -> Result<()> {
   let d = 7_usize;
   let pts1 = genvec(d,28,13,19); // random test data 
   let pts2 = genvec(d,38,23,31);
   println!("\nTrend vector:\n{}\n",wv(&pts1.trend(EPS,pts2)));
   Ok(())
}

#[test]
fn geometric_medians() -> Result<()> {
    const ITERATIONS:usize = 20;
    let n = 700_usize;
    let d = 10_usize;
    println!("timing {} medians of {} points in {} dimensions",wi(&ITERATIONS),wi(&n),wi(&d)); 
 
   let mut timer = DevTime::new_simple();
   let mut sumg = 0_f64;
   let mut sumtime = 0_u128; 
   for i in 1..ITERATIONS {
      let pts = genvec(d,n,i as u32,5*i as u32);
      timer.start();
      let gm = pts.gmedian(EPS);
      timer.stop();
      sumtime += timer.time_in_nanos().unwrap();
      sumg += pts.distsum(&gm)    
   }
   // sumg /= (ITERATIONS*n*d) as f64;
   println!("Gmedian all distances: {} ns:\t{}",wi(&sumg),wi(&sumtime)); 
 
   sumg = 0_f64;
   sumtime = 0_u128;
   timer = DevTime::new_simple();
 
   for i in 1..ITERATIONS {
      let pts = genvec(d,n,i as u32,5*i as u32);
      timer.start();
      let gm = pts.nmedian(EPS);
      timer.stop();
      sumtime += timer.time_in_nanos().unwrap();
      sumg += pts.distsum(&gm)
   } 
   // sumg /= (ITERATIONS*n*d) as f64;  
   println!("Nmedian all distances: {} ns:\t{}",wi(&sumg),wi(&sumtime));   

   sumg = 0_f64;
   sumtime = 0_u128;
   timer = DevTime::new_simple();
 
   for i in 1..ITERATIONS {
      let pts = genvec(d,n,i as u32,5*i as u32);
      timer.start();
      let gm = pts.acentroid();
      timer.stop();
      sumtime += timer.time_in_nanos().unwrap();
      sumg += pts.distsum(&gm)
   } 
   // sumg /= (ITERATIONS*n*d) as f64;  
   println!("Centroid all distncs:  {} ns:     {}",wi(&sumg),wi(&sumtime)); 
    Ok(())  
 }
