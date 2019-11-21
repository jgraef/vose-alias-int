#![allow(non_snake_case)]

use std::ops::{Add, Sub, Mul};
use std::cmp::{Ordering, PartialOrd, Ord};
use std::fmt::Debug;

use num_traits::identities::zero;
use num_traits::sign::Unsigned;



pub struct LookupTable<P>
    where P: Copy + Debug + Unsigned + Add<P> + Sub<P> + Mul<P> + Mul<usize, Output=P> + PartialOrd<P> + Ord
{
    /// The total probability - since we work with integers, this is not 1.0, but corresponds to
    /// a the probability 1.0
    T: P,

    /// Number of entries
    n: usize,

    /// Alias table
    K: Vec<usize>,

    /// Probabilities
    U: Vec<P>,
}


impl<P> LookupTable<P>
    where P: Copy + Debug + Unsigned + Add<P> + Sub<P> + Mul<P> + Mul<usize, Output=P> + PartialOrd<P> + Ord
{
    pub fn new<V: AsRef<[P]>>(p: V, T: P) -> Self {
        // The algorithm was roughly taken from
        //
        // * https://en.wikipedia.org/wiki/Alias_method#Table_generation
        // * https://github.com/asmith26/Vose-Alias-Method/blob/master/vose_sampler/vose_sampler.py
        //
        // p - probabilities p_i. We will use this for U as well
        // T - total probability
        // n - numbrt of probabilities

        let p = p.as_ref();
        let n = p.len();

        // Assert properties of input
        Self::assert_input(p, T);

        // Construct scaled probabilities
        let mut U: Vec<P> = p.iter()
            .map(|p| p.mul(n))
            .collect();


        // Construct overfull and underfull stack. These contain only indices into U
        let mut U_underfull = Vec::with_capacity(n);
        let mut U_overfull = Vec::with_capacity(n);

        for (i, p) in p.iter().enumerate() {
            match p.cmp(&T) {
                Ordering::Equal => (),
                Ordering::Greater => U_overfull.push(i),
                Ordering::Less => U_underfull.push(i),
            }
        }
        // I think this improves performance at the lookup phase
        U_underfull.reverse();

        // Construct alias table
        // K - alias table - n entries initialized with
        // NOTE: Entry 0 will never be aliased, since it will be exactly filled in the first
        // iteration of the loop, therefore we can use K_x = 0 as "not aliased".
        let mut K: Vec<usize> = Vec::with_capacity(n);
        K.resize_with(n, Default::default);

        while let (Some(i_u), Some(i_o)) = (U_underfull.pop(), U_overfull.pop()) {
            // Alias overfull into underfull
            K[i_u] = i_o;

            // Remove allocated space from U: U_o += U_u - T
            U[i_o] = U[i_o] - T + U[i_u];

            // Assign entry i_o to the appropriate category base on the new value
            match U[i_o].cmp(&T) {
                Ordering::Equal => (),
                Ordering::Greater => U_overfull.push(i_o),
                Ordering::Less => U_underfull.push(i_o),
            }
        }

        assert!(U_underfull.is_empty() && U_overfull.is_empty());

        Self { T, n, K, U }
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn total(&self) -> P {
        self.T
    }

    /// Sample from the discrete random distribution
    ///
    /// # Arguments
    ///
    /// * `x`: Must be uniformly random between 0 and `n`
    /// * `y`: Must be uniformly random between 0 and `T`
    ///
    /// # Return value
    ///
    /// Returns the index corresponding to the probability in the input `p`.
    ///
    pub fn sample(&self, x: usize, y: P) -> usize {
        let U_x = self.U[x];
        if y < U_x {
            x
        }
        else {
            let K_x = self.K[x];
            assert_ne!(K_x, 0);
            K_x
        }
    }

    fn assert_input(p: &[P], T: P) {
        let mut last: P = zero();
        let mut sum: P = zero();
        for x in p {
            assert!(*x > last);
            last = *x;
            sum = sum + *x;
        }
        assert_eq!(sum, T);
    }
}
