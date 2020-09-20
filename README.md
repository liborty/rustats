# Rstats - Rust Stats

![Crates.io](https://img.shields.io/crates/v/rstats?logo=rust) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/liborty/rstats/HEAD?logo=github)

Rstats is aimed at characterization of multidimensional sets of points, with applications to Machine Learning and Data Analysis. It begins with basic statistical measures and vector algebra, which provide self-contained tools for the more interesting algorithms but can also be used in their own right.

Our treatment of multidimensional sets of points is constructed from the first principles. Thus some original concepts, unlikely to be found elsewhere, are introduced and implemented here.

Going beyond one dimension, most authors  cheat by using 'quasi medians' (1-d medians along each axis). Quazi medians may be quicker to compute but they are a poor start to characterising multidimensional clouds of points reliably. *Specifically, all such 1-d measures depend on the choice of axis.* Such dependence has to be later removed by Principle Components Analysis or similar methods. In contradistinction to this, our methods based on the true Geometric Median, (computed here by `nmedian`), are axis (or rotation) independent.

Rstats is a lean minimalistic library that only depends on `anyhow` (for its error handling).

## Trait Stats

One dimensional statistical measures implemented for `&[i64]` and `&[f64]`.
All these methods operate on one vector of data and take no arguments.
For example, `s.amean()` returns the arithmetic mean of slice `s` of either type.
Trait Stats is carefully checked and will report all kinds of errors, such as empty input.

Included are:

* means (arithmetic, geometric and harmonic),
* standard deviations,
* linearly weighted means (useful for time dependent data analysis),
* median and quartiles.

## Trait Vectors

* Vector algebra implemented on one or two `&[f64]` slices of any length (vector dimensionality).
* Autocorrelation, Pearson's, Spearman's and Kendall's correlations.
* Finding minimum and maximum, linear transformation.

Trait Vectors is sometimes unchecked for speed, so some caution with data is advisable.

## Trait MutVectors

Some of the methods are for memory efficiency reasons reimplemented in this trait so that they mutate `self` in place instead of creating a new Vec. They are useful in vector iterative methods. Beware that they do not return anything, so they can not be chained.

## Trait VecVec

* Relationships of one vector to a set of vectors: 
Sums of distances, Centroid, Medoid, Geometric Median, Eccentricity,
Zero median data.
* Relationships between sets of multidimensional vectors: Trend.

Trait VecVec is entirely unchecked, you should  check your data upfront.

## Releases

* **Version 0.5.4** Added `irank,dv,kazutsugi'. 

* **Version 0.5.3** Added `varea` =  magnitude of the cross product. Chenged status of some mathods from 'required' to 'provided'.

* **Version 0.5.2** Renamed trait RStats to Stats, to avoid naming confusion. Separated MutVecs implementations to their own module `mutvecimpls.rs`. Added some more tests. Expanded `moe` to include mean and std of eccentricities.

* **Version 0.5.1** Added scalar addition `sadd` and linear transformation `lintrans` to `Vectors`.

* **Version 0.5.0** Introduces *VecVec* trait for all multi-point methods, now implemented for type `&[Vec<f64>]`. This is a breaking change but it did allow streamlining of the code and a clean separation of the traits. Main benefit to the user is in no longer having to explicitly pass around the dimensionality d.

* **Version 0.4.15** Added `setsub` similar to `mutzeromd` but it subtracts any given vector from a whole set.
Moved all functions into new module `functions.rs`. The other modules are now exclusively trait implementations.

* **Version 0.4.14** Added `mutzeromd` - or mutated zero median data, which transforms mutable self to zero-median form.

* **Version 0.4.13** Added `trend` between two sets of points. More comments, tests and examples.
