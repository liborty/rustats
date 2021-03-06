pub mod statsf64;
pub mod statsi64;
pub mod vecf64;
pub mod vecu8;
pub mod vecvecu8;
pub mod mutvec;
pub mod vecvecf64;
pub mod functions;

// reexporting to avoid duplication and for backwards compatibility
pub use indxvec::{here,wi}; 
/// simple error handling
use anyhow::{Result,bail}; 

/// Median and quartiles
#[derive(Default)]
pub struct Med {
    pub lquartile: f64,
    pub median: f64,
    pub uquartile: f64,
}
impl std::fmt::Display for Med {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "median:\n\tLower Q: {}\n\tMedian:  {}\n\tUpper Q: {}",
            wi(&self.lquartile),
            wi(&self.median),
            wi(&self.uquartile)
        )
    }
}

/// Mean and standard deviation (or std ratio for geometric mean)
#[derive(Default)]
pub struct MStats {
    pub mean: f64,
    pub std: f64,
}
impl std::fmt::Display for MStats {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "mean±std: {}±{}", wi(&self.mean), wi(&self.std))
    }
}

/// Basic one dimensional (1-d) statistical measures and ranking.
/// These methods operate on just one vector (of data) and take no arguments.
pub trait Stats {

    /// Arithmetic mean
    fn amean(self) -> Result<f64> 
        where Self: std::marker::Sized { bail!("amean not implemented for this type")}
    /// Arithmetic mean and standard deviation
    fn ameanstd(self) -> Result<MStats> 
        where Self: std::marker::Sized { bail!("ameanstd not implemented for this type")}
    /// Weighted arithmetic mean
    fn awmean(self) -> Result<f64> 
        where Self: std::marker::Sized { bail!("awmean not implemented for this type")}
    /// Weighted arithmetic men and standard deviation
    fn awmeanstd(self) -> Result<MStats>
        where Self: std::marker::Sized { bail!("awmeanstd not implemented for this type")}
    /// Harmonic mean
    fn hmean(self) -> Result<f64>
        where Self: std::marker::Sized { bail!("hmean not implemented for this type")}
    /// Weighted harmonic mean
    fn hwmean(self) -> Result<f64> 
        where Self: std::marker::Sized { bail!("hwmean not implemented for this type")}
    /// Geometric mean
    fn gmean(self) -> Result<f64>
        where Self: std::marker::Sized { bail!("gmean not implemented for this type")}
    /// Geometric mean and standard deviation ratio
    fn gmeanstd(self) -> Result<MStats>
        where Self: std::marker::Sized { bail!("gmeanstd not implemented for this type")}
    /// Weighed geometric mean
    fn gwmean(self) -> Result<f64> 
        where Self: std::marker::Sized { bail!("gwmean not implemented for this type")}
    /// Weighted geometric mean and standard deviation ratio
    fn gwmeanstd(self) -> Result<MStats>
        where Self: std::marker::Sized { bail!("gwmeanstd not implemented for this type")}
    /// Median and quartiles
    fn median(self) -> Result<Med>
        where Self: std::marker::Sized { bail!("median not implemented for this type")}
}

/// Vector algebra on one or two vectors.
pub trait Vecf64 {
    /// Scalar multiplication of a vector
    fn smult(self, s: f64) -> Vec<f64>;
    /// Scalar addition to vector
    fn sadd(self, s: f64) -> Vec<f64>; 
    /// Scalar product of two vectors
    fn dotp(self, v: &[f64]) -> f64;
    /// Inverse vecor of magnitude 1/|v|
    fn vinverse(self) -> Vec<f64>;
    /// Cosine = a.dotp(b)/(a.vmag*b.vmag)
    fn cosine(self, _v: &[f64]) -> f64; 
    /// Vector subtraction
    fn vsub(self, v: &[f64]) -> Vec<f64>;
    /// Vector negtion
    fn negv(self) -> Vec<f64>;
    /// Vector addition
    fn vadd(self, v: &[f64]) -> Vec<f64>;
    /// Vector magnitude
    fn vmag(self) -> f64;
    /// Vector magnitude squared
    fn vmagsq(self) -> f64;
    /// Euclidian distance between two points
    fn vdist(self, v: &[f64]) -> f64;
    /// vdist between two points squared
    fn vdistsq(self, v: &[f64]) -> f64;
    /// cityblock distance
    fn cityblockd(self, v:&[f64]) -> f64;   
    /// Unit vector
    fn vunit(self) -> Vec<f64>;
    /// Unit vector of difference (done together for efficiency)
    fn vsubunit(self, v: &[f64]) -> Vec<f64>;
    /// Area of parallelogram between two vectors (magnitude of cross product)
    fn varea(self, v:&[f64]) -> f64;
    /// Area proportional to the swept arc
    fn varc(self, v:&[f64]) -> f64; 
    /// Vector similarity in the interval [0,1]: (1+cos(theta))/2
    fn vsim(self, v:&[f64]) -> f64;
    /// Vector dissimilarity in the interval [0,1]: (1-cos(theta))/2
    fn vdisim(self, v:&[f64]) -> f64; 
    /// Correlation
    fn correlation(self, _v: &[f64]) -> f64; 
    /// Kendall's tau-b (rank order) correlation
    fn kendalcorr(self, _v: &[f64]) -> f64;
    /// Spearman's rho (rank differences) correlation
    fn spearmancorr(self, _v: &[f64]) -> f64;
    /// Autocorrelation
    fn autocorr(self) -> f64;
    /// Lower triangular part of a covariance matrix for a single f64 vector.
    fn covone(self, m:&[f64]) -> Vec<f64>;
    /// Reconstructs the full symmetric matrix from its lower diagonal compact form
    fn symmatrix(self) -> Vec<Vec<f64>>;
    /// Linear transformation to [0,1]
    fn lintrans(self) -> Vec<f64>;
    // Sort vector in a standard way
    // fn sortf(self) -> Vec<f64>; 
}

/// Some support for Vec<u8> (vector of bytes)
pub trait Vecu8 {
    /// Scalar multiplication of a vector
    fn smult(self, s: f64) -> Vec<f64>;
    /// Scalar addition to vector
    fn sadd(self, s: f64) -> Vec<f64>;
    /// Scalar product of u8 and f64 vectors
    fn dotp(self, v: &[f64]) -> f64;
    /// Scalar product of two u8 vectors -> u64
     fn dotpu8(self, v: &[u8]) -> u64;
    /// Cosine between u8 and f64 vectors
    fn cosine(self, v: &[f64]) -> f64;
    /// Cosine between two u8 vectors
    fn cosineu8(self, v: &[u8]) -> f64;
    /// Vector subtraction
    fn vsub(self, v: &[f64]) -> Vec<f64>;
    /// Vector subtraction
    fn vsubu8(self, v: &[u8]) -> Vec<f64>;
    /// Vector addition
    fn vadd(self, v: &[f64]) -> Vec<f64>;
    /// Vector addition ( converts results to f64, as they can exceed 255 )
    fn vaddu8(self, v: &[u8]) -> Vec<f64>;
    /// Vector magnitude
    fn vmag(self) -> f64;
    /// Vector magnitude squared (sum of squares)
    fn vmagsq(self) -> f64;
    /// Euclidian distance to &[f64]
    fn vdist(self, v:&[f64]) -> f64;
    /// Euclidian distance to &[u8]
    fn vdistu8(self, v:&[u8]) -> f64;
    /// Euclidian distance between byte vectors
    fn vdistsq(self, v: &[u8]) -> u64; 
    /// cityblock distance
    fn cityblockd(self, v:&[f64]) -> f64;
    /// cityblock distance
    fn cityblockdu8(self, v:&[u8]) -> f64;  
    /// Vector similarity S in the interval [0,1]: S = (1+cos(theta))/2
    fn vsim(self, v:&[f64]) -> f64;
    /// We define vector dissimilarity D in the interval [0,1]: D = 1-S = (1-cos(theta))/2
    fn vdisim(self, v:&[f64]) -> f64;   
    /// Area proportional to the swept arc
    fn varc(self, v:&[f64]) -> f64; 
    /// Lower triangular part of a covariance matrix for a single f64 vector.
    fn covone(self, m:&[f64]) -> Vec<f64>;

    /// Probability density function (pdf) of bytes data
    fn pdf(self) -> Vec<f64>;
    /// Information (entropy) in nats of &[u8]
    fn entropy(self) -> f64;
    /// Counts of joint bytes values
    fn jointpdf(self, v:&[u8]) -> Vec<Vec<u32>>;
    /// Joint entropy of &[u8],&[u8] in nats 
    fn jointentropy(self, v:&[u8]) -> f64;
    /// Statistical independence measure based on joint entropy
    fn dependence(self, v:&[u8]) -> f64; 

    /// cast vector of u8s to vector of f64s
    fn vecu8asvecf64(self) -> Vec<f64>;
}

/// Mutable vector operations.
/// Some of the vectors trait methods reimplemented here for efficiency, to mutate in-place
pub trait MutVectors {

    /// mutable multiplication by a scalar
    fn mutsmult(self, _s: f64) where Self: std::marker::Sized {}  
    /// mutable vector subtraction
    fn mutvsub(self, _v: &[f64]) where Self: std::marker::Sized {}
    fn mutvsubu8(self, _v: &[u8]) where Self: std::marker::Sized {} 
    /// mutable vector addition
    fn mutvadd(self, _v: &[f64]) where Self: std::marker::Sized {}
    fn mutvaddu8(self, _v: &[u8]) where Self: std::marker::Sized {}
     /// mutably makes into a unit vector
    fn mutvunit(self) where Self: std::marker::Sized {}
    /// sort in place
    fn mutsortf(self) where Self: std::marker::Sized {} 
}

/// Some support for self argument of Vec<Vec<u8>> type (vector of vectors of bytes)
pub trait VecVecu8 { 
    /// Centroid = euclidian mean of a set of points  
    fn acentroid(self) -> Vec<f64>;
    /// Weighted Centre
    fn wacentroid(self,ws: &[u8]) -> Vec<f64>;
    /// Eccentricity vector added to a non member point,
    fn nxnonmember(self, p:&[f64]) -> Vec<f64>;
    /// Weighted eccentricity vector for a non member point
    fn wnxnonmember(self, ws:&[u8], p:&[f64]) -> Vec<f64>; 
    /// Weighted geometric median, sorted eccentricities magnitudes, cpdf of the weights
    fn gmedian(self, eps:f64) -> Vec<f64>; 
    /// The weighted geometric median
    fn wgmedian(self, ws:&[u8], eps: f64) -> Vec<f64>;
    /// Lower triangular part of a covariance matrix of a Vec of u8 vectors.
    fn covar(self, med:&[f64]) -> Vec<f64>; 
    fn wcovar(self, ws:&[u8], m:&[f64]) -> Vec<f64>;
}

/// Methods applicable to vector of vectors of <f64>
pub trait VecVecf64 {

    /// Arithmetic Centre = euclidian mean of a set of points
    fn acentroid(self) -> Vec<f64>;
    /// Weighted Arithmetic Centre = weighted euclidian mean of a set of points
    fn wacentroid(self,ws: &[f64]) -> Vec<f64>;
    /// Geometric Centroid
    fn gcentroid(self) -> Vec<f64>;
    /// Harmonic Centroid = harmonic mean of a set of points
    fn hcentroid(self) -> Vec<f64>; 
    /// Trend between two sets
    fn trend(self, eps: f64, v: Vec<Vec<f64>>) -> Vec<f64>;
    /// Subtract m from all points - e.g. transform to zero median form
    fn translate(self, m: &[f64]) -> Vec<Vec<f64>>;

    /// Sums of distances from each point to all other points.
     fn distsums(self) -> Vec<f64>;
    /// Fast sums of distances from each point to all other points 
    fn distsuminset(self, indx: usize) -> f64;
    /// Sum of distances from arbitrary point (v) to all the points in self   
    fn distsum(self, v: &[f64]) -> f64;
    /// Individual distances from any point v (typically not in self) to all the points in self.    
    fn dists(self, v: &[f64]) -> Vec<f64>;
    /// Medoid and Outlier (by distance) of a set of points
    fn medoid(self) -> (f64, usize, f64, usize); 
    /// Eccentricity vectors from each point
    fn eccentricities(self) -> Vec<Vec<f64>>;
    /// Exact eccentricity vectors from all member points by first finding the Geometric Median.
    /// As well as being more accurate, it is usually faster than `eccentricities` above, 
    /// especially for large numbers of points.
    fn exacteccs(self, eps: f64) -> Vec<Vec<f64>>;
    /// Returns ( gm, sorted eccentricities magnitudes )
    fn sortedeccs(self, ascending:bool, eps:f64) -> ( Vec<f64>,Vec<f64> );
    /// ( wgm, sorted eccentricities magnitudes, associated cpdf )
    fn wsortedeccs(self, ws: &[f64], eps:f64) -> ( Vec<f64>,Vec<f64>,Vec<f64> ); 
    /// Sorted cosines magnitudes and cpdf, needs central median
    fn wsortedcos(self, medmed: &[f64], med: &[f64], ws: &[f64]) -> ( Vec<f64>,Vec<f64> ); 
    /// Next approx median point from this member point given by its indx
    fn nxmember(self, indx: usize) -> Vec<f64>;
    /// Ecentricity of a member point given by its indx
    fn eccmember(self, indx: usize) -> Vec<f64>;
    /// Next approx median point from this nonmember point
    fn nxnonmember(self, p:&[f64]) -> Vec<f64>;
    /// Error vector, i.e. unscaled eccentricity vector
    fn errorv(self, p:&[f64]) -> Vec<f64>;
    /// Eccentricity vector for a non member point
    fn eccnonmember(self, p:&[f64]) -> Vec<f64>; 
    /// Weighted eccentricity vector for a non member point
    fn wnxnonmember(self, ws:&[f64], p:&[f64]) -> Vec<f64>; 
    /// magnitudes of a set of vectors
    fn mags(self) -> Vec<f64>; 
    /// Median and quartiles of eccentricities (new robust measure of spread of a multivariate sample)
    fn moe(self, eps: f64) -> (MStats,Med);
    /// Medoid and Outlier as defined by eccentricities.
    fn emedoid(self, eps: f64) -> (f64, usize, f64, usize);

    /// Geometric medians of a set
    /// First iteration point for geometric medians
    fn firstpoint(self) -> Vec<f64>;
    /// Improved Weizsfeld's Algorithm for geometric median
    fn nmedian(self, eps: f64) -> Vec<f64>;
    /// New secant algorithm for geometric median
    fn gmedian(self, eps: f64) -> Vec<f64>; 
    /// The weighted geometric median
    fn wgmedian(self, ws: &[f64],eps: f64) -> Vec<f64>; 
    /// Flattened lower triangular part of a covariance matrix of a Vec of f64 vectors.
    fn covar(self, med:&[f64]) -> Vec<f64>; 
    /// Flattened lower triangular part of a covariance matrix for weighted f64 vectors.
    fn wcovar(self, ws:&[f64], m:&[f64]) -> Vec<f64>;
}
