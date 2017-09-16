// Copyright 2016 bluss and ndarray developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Constructors for randomized arrays. `rand` integration for `ndarray`.
//!
//! See [**`RandomExt`**](trait.RandomExt.html) for usage examples.
extern crate rand;
extern crate ndarray;

use rand::Rng;
use rand::distributions::Sample;
use rand::distributions::IndependentSample;

use ndarray::{ArrayBase, Dimension, DataOwned};
use ndarray::ShapeBuilder;

/// Constructors for n-dimensional arrays with random elements.
///
/// This trait extends ndarray’s `ArrayBase` and can not be implemented
/// for other types.
///
/// The default Rng is a fast automatically seeded rng (currently `rand::weak_rng`).
pub trait RandomExt<S, D>
where
    S: DataOwned,
    D: Dimension,
{
    /// Create an array with shape `dim` with elements drawn from
    /// `distribution`  using the default rng.
    ///
    /// ***Panics*** if the number of elements overflows usize.
    ///
    /// ```
    /// extern crate rand;
    /// extern crate ndarray;
    /// extern crate ndarray_rand;
    ///
    /// use rand::distributions::Range;
    /// use ndarray::Array;
    /// use ndarray_rand::RandomExt;
    ///
    /// # fn main() {
    /// let a = Array::random((2, 5), Range::new(0., 10.));
    /// println!("{:8.4}", a);
    /// // Example Output:
    /// // [[  8.6900,   6.9824,   3.8922,   6.5861,   2.4890],
    /// //  [  0.0914,   5.5186,   5.8135,   5.2361,   3.1879]]
    /// # }
    fn random<Sh, IdS>(shape: Sh, distribution: IdS) -> ArrayBase<S, D>
    where
        IdS: IndependentSample<S::Elem>,
        Sh: ShapeBuilder<Dim = D>;

    /// Create an array with shape `dim` with elements drawn from
    /// `distribution`, using a specific Rng `rng`.
    ///
    /// ***Panics*** if the number of elements overflows usize.
    fn random_using<Sh, IdS, R>(shape: Sh, distribution: IdS, rng: &mut R) -> ArrayBase<S, D>
    where
        IdS: IndependentSample<S::Elem>,
        R: Rng,
        Sh: ShapeBuilder<Dim = D>;
}

impl<S, D> RandomExt<S, D> for ArrayBase<S, D>
where
    S: DataOwned,
    D: Dimension,
{
    fn random<Sh, IdS>(shape: Sh, dist: IdS) -> ArrayBase<S, D>
    where
        IdS: IndependentSample<S::Elem>,
        Sh: ShapeBuilder<Dim = D>,
    {
        Self::random_using(shape, dist, &mut rand::weak_rng())
    }

    fn random_using<Sh, IdS, R>(shape: Sh, dist: IdS, rng: &mut R) -> ArrayBase<S, D>
    where
        IdS: IndependentSample<S::Elem>,
        R: Rng,
        Sh: ShapeBuilder<Dim = D>,
    {
        Self::from_shape_fn(shape, |_| dist.ind_sample(rng))
    }
}

/// A wrapper type that allows casting f64 distributions to f32
///
/// ```
/// extern crate rand;
/// extern crate ndarray;
/// extern crate ndarray_rand;
///
/// use rand::distributions::Normal;
/// use ndarray::Array;
/// use ndarray_rand::{RandomExt, F32};
///
/// # fn main() {
/// let a = Array::random((2, 5), F32(Normal::new(0., 1.)));
/// println!("{:8.4}", a);
/// // Example Output:
/// // [[ -0.6910,   1.1730,   1.0902,  -0.4092,  -1.7340],
/// //  [ -0.6810,   0.1678,  -0.9487,   0.3150,   1.2981]]
/// # }
#[derive(Copy, Clone, Debug)]
pub struct F32<S>(pub S);

impl<S> Sample<f32> for F32<S>
where
    S: Sample<f64>,
{
    fn sample<R>(&mut self, rng: &mut R) -> f32
    where
        R: Rng,
    {
        self.0.sample(rng) as f32
    }
}

impl<S> IndependentSample<f32> for F32<S>
where
    S: IndependentSample<f64>,
{
    fn ind_sample<R>(&self, rng: &mut R) -> f32
    where
        R: Rng,
    {
        self.0.ind_sample(rng) as f32
    }
}
