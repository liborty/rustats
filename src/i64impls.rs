use anyhow::{Result,Context,ensure};
use crate::{MStats,RStats,Med,wsum,emsg};

impl RStats for &[i64] {
   
   /// Arithmetic mean of an i64 slice
   /// # Example
   /// ```
   /// use rstats::RStats;
   /// let v1:Vec<i64> = vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14];
   /// assert_eq!(v1.as_slice().amean().unwrap(),7.5_f64);
   /// ```
   fn amean(self) -> Result<f64> { 
      let n = self.len();
      ensure!(n>0,emsg(file!(),line!(),"amean - sample is empty!"));
      Ok( self.iter().map(|&x| x as f64).sum::<f64>() / (n as f64) )
   } 

   /// Arithmetic mean and standard deviation of an i64 slice
   /// # Example
   /// ```
   /// use rstats::RStats;
   /// let v1 = vec![1_i64,2,3,4,5,6,7,8,9,10,11,12,13,14];
   /// let res = v1.as_slice().ameanstd().unwrap();
   /// assert_eq!(res.mean,7.5_f64);
   /// assert_eq!(res.std,4.031128874149275_f64);
   /// ```
   fn ameanstd(self) -> Result<MStats> {
      let n = self.len();
      ensure!(n>0,emsg(file!(),line!(),"ameanstd - sample is empty!"));
      let mut sx2 = 0_f64;
      let mean = self.iter().map(|&x|{ let lx = x as f64;sx2+=lx*lx; lx}).sum::<f64>() / (n as f64);
      Ok( MStats { mean : mean, std : (sx2 /(n as f64) - mean.powi(2)).sqrt() } )
   }

   /// Linearly weighted arithmetic mean of an i64 slice.     
   /// Linearly descending weights from n down to one.    
   /// Time dependent data should be in the stack order - the last being the oldest.
   /// # Example
   /// ```
   /// use rstats::RStats;
   /// let v1 = vec![1_i64,2,3,4,5,6,7,8,9,10,11,12,13,14];
   /// assert_eq!(v1.as_slice().awmean().unwrap(),5.333333333333333_f64);
   /// ```
   fn awmean(self) -> Result<f64> {
      let n = self.len();
      ensure!(n>0,emsg(file!(),line!(),"awmean - sample is empty!"));
      let mut w = (n+1)as f64; // descending linear weights
	   Ok( self.iter().map(|&x| { w -= 1.; w*x as f64 }).sum::<f64>() / wsum(n))
   }

   /// Liearly weighted arithmetic mean and standard deviation of an i64 slice.    
   /// Linearly descending weights from n down to one.    
   /// Time dependent data should be in the stack order - the last being the oldest.
   /// # Example
   /// ```
   /// use rstats::RStats;
   /// let v1 = vec![1_i64,2,3,4,5,6,7,8,9,10,11,12,13,14];
   /// let res = v1.as_slice().awmeanstd().unwrap();
   /// assert_eq!(res.mean,5.333333333333333_f64);
   /// assert_eq!(res.std,3.39934634239519_f64);
   /// ```
   fn awmeanstd(self) -> Result<MStats> {
      let n = self.len();
      ensure!(n>0,emsg(file!(),line!(),"awmeanstd - sample is empty!"));
      let mut sx2 = 0_f64;
      let mut w = n as f64; // descending linear weights
      let mean = self.iter().map( |&x| {
         let lx = x as f64; let wx = w*lx;
         sx2 += wx*lx; w -= 1.; wx } ).sum::<f64>() as f64 / wsum(n);
   Ok( MStats { mean: mean, std: (sx2/wsum(n) - mean.powi(2)).sqrt() } )  
   }

   /// Harmonic mean of an i64 slice.
   /// # Example
   /// ```
   /// use rstats::RStats;
   /// let v1 = vec![1_i64,2,3,4,5,6,7,8,9,10,11,12,13,14];
   /// assert_eq!(v1.as_slice().hmean().unwrap(),4.305622526633627_f64);
   /// ```
   fn hmean(self) -> Result<f64> {
   let n = self.len();
   ensure!(n>0,emsg(file!(),line!(),"hmean - sample is empty!"));
   let mut sum = 0_f64;
   for &x in self {
      ensure!(x!=0_i64,emsg(file!(),line!(),"hmean does not accept zero valued data!"));  
      sum += 1.0/(x as f64) 
      }
   Ok ( n as f64 / sum )
   }

   /// Linearly weighted harmonic mean of an i64 slice.    
   /// Linearly descending weights from n down to one.    
   /// Time dependent data should be in the stack order - the last being the oldest.
   /// # Example
   /// ```
   /// use rstats::RStats;
   /// let v1 = vec![1_i64,2,3,4,5,6,7,8,9,10,11,12,13,14];
   /// assert_eq!(v1.as_slice().hwmean().unwrap(),3.019546395306663_f64);
   /// ```
   fn hwmean(self) -> Result<f64> {
      let n = self.len();
      ensure!(n>0,emsg(file!(),line!(),"hwmean - sample is empty!"));
      let mut sum = 0_f64;
      let mut w = n as f64;
      for &x in self {
         ensure!(x!=0_i64,emsg(file!(),line!(),"hwmean does not accept zero valued data!"));
         sum += w/x as f64;
         w -= 1_f64; 
      }
   Ok( wsum(n) / sum )
   }

   /// Geometric mean of an i64 slice.  
   /// The geometric mean is just an exponential of an arithmetic mean
   /// of log data (natural logarithms of the data items).  
   /// The geometric mean is less sensitive to outliers near maximal value.  
   /// Zero valued data is not allowed.
   /// # Example
   /// ```
   /// use rstats::RStats;
   /// let v1 = vec![1_i64,2,3,4,5,6,7,8,9,10,11,12,13,14];
   /// assert_eq!(v1.as_slice().gmean().unwrap(),6.045855171418503_f64);
   /// ```
   fn gmean(self) -> Result<f64> {
      let n = self.len();
      ensure!(n>0,emsg(file!(),line!(),"gmean - sample is empty!"));
      let mut sum = 0_f64;
      for &x in self {   
         ensure!(x!=0_i64,emsg(file!(),line!(),"gmean does not accept zero valued data!"));
         sum += (x as f64).ln()
      }
   Ok( (sum/(n as f64)).exp() )
   }

   /// Linearly weighted geometric mean of an i64 slice.  
   /// Descending weights from n down to one.    
   /// Time dependent data should be in the stack order - the last being the oldest.  
   /// The geometric mean is just an exponential of an arithmetic mean
   /// of log data (natural logarithms of the data items).  
   /// The geometric mean is less sensitive to outliers near maximal value.  
   /// Zero data is not allowed - would at best only produce zero result.
   /// # Example
   /// ```
   /// use rstats::RStats;
   /// let v1 = vec![1_i64,2,3,4,5,6,7,8,9,10,11,12,13,14];
   /// assert_eq!(v1.as_slice().gwmean().unwrap(),4.144953510241978_f64);
   /// ```
   fn gwmean(self) -> Result<f64> {
      let n = self.len();
      ensure!(n>0,emsg(file!(),line!(),"gwmean - sample is empty!"));
      let mut w = n as f64; // descending weights
      let mut sum = 0_f64;
      for &x in self {  
         ensure!(x!=0_i64,emsg(file!(),line!(),"gwmean does not accept zero valued data!"));
         sum += w*(x as f64).ln();
         w -= 1_f64;
      }
   Ok( (sum/wsum(n)).exp() )
   }

   /// Geometric mean and std ratio of an i64 slice.  
   /// Zero valued data is not allowed.  
   /// Std of ln data becomes a ratio after conversion back.
   /// # Example
   /// ```
   /// use rstats::RStats;
   /// let v1 = vec![1_i64,2,3,4,5,6,7,8,9,10,11,12,13,14];
   /// let res = v1.as_slice().gmeanstd().unwrap();
   /// assert_eq!(res.mean,6.045855171418503_f64);
   /// assert_eq!(res.std,2.1084348239406303_f64);
   /// ```
   fn gmeanstd(self) -> Result<MStats> {
      let n = self.len();
      ensure!(n>0,emsg(file!(),line!(),"gmeanstd - sample is empty!"));
      let mut sum = 0_f64; let mut sx2 = 0_f64;
      for &x in self { 
         ensure!(x!=0_i64,emsg(file!(),line!(),"gmeanstd does not accept zero valued data!"));
      let lx = (x as f64).ln();
      sum += lx; sx2 += lx*lx    
      }
      sum /= n as f64;
      Ok( MStats { mean: sum.exp(), std: (sx2/(n as f64) - sum.powi(2)).sqrt().exp() } )
   }

   /// Linearly weighted version of gmeanstd.
   /// # Example
   /// ```
   /// use rstats::RStats;
   /// let v1 = vec![1_i64,2,3,4,5,6,7,8,9,10,11,12,13,14];
   /// let res = v1.as_slice().gwmeanstd().unwrap();
   /// assert_eq!(res.mean,4.144953510241978_f64);
   /// assert_eq!(res.std,2.1572089236412597_f64);
   /// ```
   fn gwmeanstd(self) -> Result<MStats> {
      let n = self.len();
      ensure!(n>0,emsg(file!(),line!(),"gwmeanstd - sample is empty!"));
      let mut w = n as f64; // descending weights
      let mut sum = 0_f64; let mut sx2 = 0_f64;
      for &x in self { 
         ensure!(x!=0_i64,emsg(file!(),line!(),"gwmeanstd does not accept zero valued data!"));
         let lnx = (x as f64).ln();
         sum += w*lnx; sx2 += w*lnx*lnx;
         w -= 1_f64;
      }
   sum /= wsum(n);
   Ok( MStats { mean : sum.exp(), std : (sx2 as f64/wsum(n) - sum.powi(2)).sqrt().exp() } )
   }

   /// Fast median (avoids sorting).  
   /// The data values must be within a moderate range not exceeding u16size (65535).
   /// # Example
   /// ```
   /// use rstats::RStats;
   /// let v1 = vec![1_i64,2,3,4,5,6,7,8,9,10,11,12,13,14];
   /// let res = v1.as_slice().median().unwrap();
   /// assert_eq!(res.median,7.5_f64);
   /// assert_eq!(res.lquartile,4_f64);
   /// assert_eq!(res.uquartile,11_f64);
   /// ```
   fn median(self) -> Result<Med> {
      let n = self.len() as u32;
      ensure!(n>0,emsg(file!(),line!(),"median - sample is empty!"));
      ensure!(n<=u32::max_value(),
         emsg(file!(),line!(),"median - sample is probably too large!"));
      let &max = self.iter().max().with_context(||emsg(file!(),line!(),"median failed to find maximum"))?;
      let &min = self.iter().min().with_context(||emsg(file!(),line!(),"median failed to find minimum"))?;
      let range = (max-min+1) as usize;
      ensure!(range <= u32::max_value() as usize, // range is probably too large to use as subscripts
      "{}:{} rstats median range {} of values is too large",file!(),line!(),range);
	   let mut acc = vec![0_u32; range]; // min max values inclusive
      for &item in self { acc[(item-min) as usize] += 1_u32 } // computes frequency distribution
      let mut result: Med = Default::default();
      let mut cumm = 0_u32;
      let mut i2;

      for i in 0..range {  // find the lower quartile
      cumm += acc[i]; // accummulate frequencies
      if 4 * cumm >= n {
         result.lquartile = (i as i64 + min) as f64; // restore min value
         break;
         }
      }
      cumm = 0u32;
      for i in (0..range).rev() {  // find the upper quartile
      cumm += acc[i]; // accummulate frequencies
      if 4 * cumm >= n {
         result.uquartile = (i as i64 + min) as f64;
         break;
         }
      }
      cumm = 0u32;
      for i in 0..range {  // find the midpoint of the frequency distribution
      cumm += acc[i]; // accummulate frequencies
      if 2 * cumm == n {
         // even, the other half must have the same value
         i2 = i + 1;
         while acc[i2] == 0 { i2 += 1 }
         // first next non-zero acc[i2] must represent the other half
         result.median = ((i + i2) as i64 + 2*min) as f64 / 2_f64;
         break;
         }
      if 2 * cumm > n {
         // first over the half items, this must be the odd midpoint
         result.median = (i as i64 + min) as f64;
         break;
         }      
      }
      Ok(result)
   }

   /// Returns vector of ranks 1..n,
   /// ranked from the biggest number in self (rank 1) to the smallest (rank n).
   /// Equalities lead to fractional ranks, hence Vec<f64> output and the range of rank values is reduced.
   fn ranks(self) -> Result<Vec<f64>> {
      let n = self.len();
      let mut rank = vec![1_f64;n];
      // make all n*(n-1)/2 comparisons just once
      for i in 1..n {
         let x = self[i];
         for j in 0..i {
            if x > self[j] { rank[j]+=1_f64; continue };
            if x < self[j] { rank[i]+=1_f64; continue };
            rank[i]+=0.5; rank[j]+=0.5;
         }
      }
      Ok(rank)
   }

}

