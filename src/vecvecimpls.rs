use crate::{Med, MutVectors, RStats, VecVec, Vectors};

impl VecVec for &[Vec<f64>] {
    /// Centroid = simple multidimensional arithmetic mean
    /// # Example
    /// ```
    /// use rstats::{Vectors,VecVec,functions::genvec};
    /// let pts = genvec(15,15,255,30);
    /// let centre = pts.acentroid();
    /// let dist = pts.distsum(&centre);
    /// assert_eq!(dist, 4.14556218326653_f64);
    /// ```
    fn acentroid(self) -> Vec<f64> {
        let mut centre = vec![0_f64; self[0].len()];
        for v in self {
            centre.mutvadd(&v)
        }
        centre.mutsmult(1.0 / self.len() as f64);
        centre
    }

    /// For each point, gives its sum of distances to all other points.
    /// This is the efficient workhorse of distances based analysis.
    fn distances(self) -> Vec<f64> {
        let n = self.len();
        let mut dists = vec![0_f64; n]; // distances accumulator for all points
                                        // examine all unique pairings (lower triangular part of symmetric flat matrix)
        for i in 1..n {
            let thisp = &self[i];
            for j in 0..i {
                let thatp = &self[j];
                let d = thisp.vdist(&thatp); // calculate each distance relation just once
                dists[i] += d;
                dists[j] += d; // but add it to both points
            }
        }
        dists
    }

    /// The sum of distances from a set point given by its `indx` to all the other points in self.
    /// This method is suitable for a single point. For all the points, use more
    /// efficient `distances`.    
    fn distsuminset(self, indx: usize) -> f64 {
        let n = self.len();
        let mut sum = 0_f64;
        let thisp = &self[indx];
        for i in 0..n {
            if i == indx {
                continue;
            };
            sum += self[i].vdist(&thisp)
        }
        sum
    }

    /// The sum of distances from any point v (typically not in self) to all the points in self.    
    /// Geometric Median is defined as the point v which minimises this function.
    fn distsum(self, v: &[f64]) -> f64 {
        self.iter().map(|p| p.vdist(v)).sum::<f64>()
    }

    /// Medoid is the point belonging to set of points `self`,
    /// which has the least sum of distances to all other points.
    /// Outlier is the point with the greatest sum of distances.
    /// This function returns a four-tuple:  
    /// (medoid_distance, medoid_index, outlier_distance, outlier_index).
    /// `d` is the number of dimensions = length of the point sub-slices.
    /// The entire set of points is held in one flat `&[f64]`.  
    /// This is faster than vec of vecs but we have to handle the indices.  
    /// # Example
    /// ```
    /// use rstats::{Vectors,VecVec,functions::genvec};
    /// let pts = genvec(15,15,255,30);
    /// let (dm,_,_,_) = pts.medoid();
    /// assert_eq!(dm,4.812334638782327_f64);
    /// ```
    fn medoid(self) -> (f64, usize, f64, usize) {
        self.distances().minmax()
    }

    /// Eccentricity vector for each point.
    /// This is the efficient workhorse of eccentrities analysis.
    fn eccentricities(self) -> Vec<Vec<f64>> {
        let n = self.len();
        // allocate vectors for the results
        let mut eccs = vec![vec![0_f64; self[0].len()]; n];
        // ecentricities vectors accumulator for all points
        // examine all unique pairings (lower triangular part of symmetric flat matrix)
        for i in 1..n {
            let thisp = &self[i];
            for j in 0..i {
                let e = self[j].vsub(&thisp).vunit(); // calculate each vector just once
                eccs[i].mutvadd(&e);
                eccs[j].mutvsub(&e); // mind the vector's orientation!
            }
        }
        eccs
    }

    /// Scalar positive measure of `not being a median` for a point belonging to the set.
    /// The point is specified by its index `indx`.
    /// The median does not have to be known. The perfect median would return zero.
    /// This is suitable for a single point. When eccentricities of all the points
    /// are needed, use more efficient `eccentricities`.
    fn eccentrinset(self, indx: usize) -> f64 {
        let n = self.len();
        let mut vsum = vec![0_f64; self[0].len()];
        let thisp = &self[indx];
        for i in 0..n {
            if i == indx {
                continue;
            }; // exclude this point
            vsum.mutvadd(&self[i].vsub(&thisp).vunit());
        }
        vsum.vmag() / n as f64
    }

    /// Returns (Measure, Eccentricity-Vector) of any point (typically one not belonging to the set).
    /// The first (scalar) part of the result is a positive measure of `not being a median`.
    /// The second part is the eccentricity vector, which always points towards the median.
    /// The vector is of particular value and interest.
    /// This function has no prior knowledge of the actual median.  
    /// This is suitable for a single point. When eccentricities of all the points
    /// are needed, use more efficient `eccentricities`.
    fn veccentr(self, thisp: &[f64]) -> (f64, Vec<f64>) {
        let mut vsum = vec![0_f64; self[0].len()];
        for thatp in self {
            let mut vdif = thatp.vsub(thisp);
            let mag = vdif.vmag();
            if mag.is_normal() {
                vdif.mutsmult(1. / mag); // using already computed magnitude to find unit vdif
                vsum.mutvadd(&vdif); // add it to the sum of vector eccentricities
            } // else mag = 0, so just skip thatp, as it is the same as thisp
        }
        (vsum.vmag()/thisp.len() as f64, vsum)
    }

    /// This convenience wrapper calls `veccentr` and extracts just the eccentricity (residual error for median).
    /// Thus this method is the equivalent of `eccentr`
    /// but suited for any explicitly given point, typically not belonging to the set.  
    /// When the eccentricity vector is needed, use `veccentr`
    fn ecc(self, v: &[f64]) -> f64 {
        let (eccentricity, _) = self.veccentr(v);
        eccentricity
    }

    /// Magnitudes of the vectors in self
    fn mags(self) -> Vec<f64> {
        let mut magsv = Vec::new();
        for v in self {
            magsv.push(v.vmag())
        }
        magsv
    }

    /// Scalar measures of eccentricity for the whole set.
    /// The output can be typically passed to `median`
    /// or `minmax` to find the Outlier and the Medoid.
    fn scalarecc(self) -> Vec<f64> {
        let mut scecc = self.mags();
        scecc.mutsmult(1_f64 / self.len() as f64);
        scecc
    }

    /// Median of eccentricities measures (MOE).
    /// This is a new robust measure of spread of multidimensional points
    /// (or multivariate sample).  
    fn moe(self) -> Med {
        self.eccentricities().scalarecc().median().unwrap()
    }

    /// Eccentricity defined Medoid and Outlier.
    /// This can give different results to `medoid` above, defined by sums of distances,
    /// especially for the outliers. See tests.rs.  
    /// Consider some point c and some other points, bunched up at a distance r from c.
    /// The sum of their distances will be n*r. Now, spread those points around a circle of radius r from c.
    /// The sum of their distances from c will remain the same but the eccentricity of c will be much reduced.
    /// # Example
    /// ```
    /// use rstats::{Vectors,VecVec,functions::genvec};
    /// let d = 6_usize;
    /// let pt = genvec(d,24,7,13); // random test data 5x20
    /// let (_medoideccentricity,medei,_outlierecccentricity,outei) = pt.emedoid();
    /// assert_eq!(medei,10); // index of e-medoid
    /// assert_eq!(outei,9);  // index of e-outlier
    /// ```
    fn emedoid(self) -> (f64, usize, f64, usize) {
        self.eccentricities().scalarecc().minmax()
    }

    /// Geometric Median (gm) is the point that minimises the sum of distances to a given set of points.
    /// It has (provably) only vector iterative solutions.
    /// Search methods are slow and difficult in highly dimensional space.
    /// Weiszfeld's fixed point iteration formula had known problems with sometimes failing to converge.
    /// Especially, when the points are dense in the close proximity of the gm,
    /// or it coincides with one of them.  
    /// However, these problems are fixed in my new algorithm here.      
    /// There will eventually be a multithreaded version of `nmedian`.
    /// # Example
    /// ```
    /// use rstats::{VecVec,functions::genvec};
    /// let pt = genvec(15,15,255,30);
    /// let gm = pt.nmedian(1e-5);
    /// let error = pt.ecc(&gm);
    /// assert_eq!(error,0.000004826966175302838_f64);
    /// ```
    fn nmedian(self, eps: f64) -> Vec<f64> {
        let mut oldpoint = self.acentroid(); // start iterating from the centroid
        loop {
            let (rsum, mut newv) = self.betterpoint(&oldpoint);
            newv.mutsmult(1.0 / rsum); // scaling the returned sum of unit vectors
            if newv.vdist(&oldpoint) < eps {
                // test the magnitude of this move for termination
                oldpoint = newv;
                break; // use the last small iteration anyway, as it is already computed
            };
            oldpoint = newv // set up next iteration
        }
        oldpoint
    }

    /// betterpoint is called by nmedian.
    /// Scaling by rsum is left as the final step at calling level,
    /// in order to facilitate data parallelism.
    fn betterpoint(self, v: &[f64]) -> (f64, Vec<f64>) {
        let mut rsum = 0_f64;
        let mut vsum = vec![0_f64; v.len()];
        for thatp in self {
            let dist = v.vdist(&thatp);
            if dist.is_normal() {
                let recip = 1.0 / dist;
                rsum += recip;
                vsum.mutvadd(&thatp.smult(recip))
            }
        }
        (rsum, vsum)
    }

    /// Trend computes the vector connecting the geometric medians of two sets of multidimensional points.
    /// This is a robust relationship between two unordered multidimensional sets.
    /// The two sets have to be in the same space but can have different numbers of points.
    fn trend(self, eps: f64, v: Vec<Vec<f64>>) -> Vec<f64> {
        let m1 = self.nmedian(eps);
        let m2 = v.nmedian(eps);
        m2.vsub(&m1)
    }

    /// Translates the whole set by vector -m. Returns Vec of Vecs.
    /// When m is set to the geometric median, this produces the zero median form.
    /// The geometric median is invariant with respect to rotation,
    /// unlike the often misguidedly used mean (`acentroid` here), or the quasi median,
    /// both of which depend on the choice of axis.
    /// The quasi-median is not even implemented by rstats.
    fn translate(self, m: &[f64]) -> Vec<Vec<f64>> {
        let mut result = Vec::new();
        for point in self {
            result.push(point.vsub(m))
        }
        result
    }
}