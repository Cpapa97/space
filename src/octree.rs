//! Octree types and algorithms.

mod linear;
mod pointer;

pub use self::linear::LinearOctree;
pub use self::pointer::PointerOctree;
pub use self::pointer::ResizingPointerOctree;

use crate::morton::*;
use nalgebra::Vector3;
use num_traits::{Float, FromPrimitive, ToPrimitive};

use serde::{Deserialize, Serialize};

/// Implement this trait to perform a tree fold across the octree.
///
/// This will convert leaf nodes into the internal `Sum` type and then propogate them up to parent regions by
/// calling `fold`.
pub trait Folder<Item, M> {
    /// This is the type that `gather` and `fold` will produce and acts as the accumulator.
    type Sum;

    /// `gather` converts a leaf node into the internal `Sum` type.
    fn gather<'a>(&self, morton: M, item: &'a Item) -> Self::Sum;

    /// `fold` is allowed to assume the `it` gives at least one item and no more than 8 items.
    fn fold<I>(&self, it: I) -> Self::Sum
    where
        I: Iterator<Item = Self::Sum>;
}

impl<Item, M, F> Folder<Item, M> for &F
where
    F: Folder<Item, M>,
{
    type Sum = F::Sum;

    fn gather<'a>(&self, morton: M, item: &'a Item) -> Self::Sum {
        (*self).gather(morton, item)
    }

    fn fold<I>(&self, it: I) -> Self::Sum
    where
        I: Iterator<Item = Self::Sum>,
    {
        (*self).fold(it)
    }
}

macro_rules! tuple_folder {
    ({$($id: ident),* $(,)?}, {$($sm: ident),* $(,)?}, {$($acc: ident),* $(,)?}, {$($item: ident),* $(,)?}) => {
        #[allow(non_snake_case)]
        impl <Item, M, $($id:),*> Folder<Item, M> for ($($id),*)
            where M: Morton, $($id: Folder<Item, M>,)*
        {
            type Sum = ($($id::Sum),*);

            fn gather<'a>(&self, morton: M, item: &'a Item) -> Self::Sum {
                let ($(ref $id),*) = *self;
                ($($id.gather(morton, item)),*)
            }

            fn fold<IT>(&self, it: IT) -> Self::Sum
            where
                IT: Iterator<Item = Self::Sum>,
            {
                let ($($sm),*): ($(smallvec::SmallVec<[$id::Sum; 8]>),*) =
                    it.fold(<($(smallvec::SmallVec<[$id::Sum; 8]>),*)>::default(),
                            |($(mut $acc),*), ($($item),*)| {
                                $($acc.push($item);)*
                                ($($acc),*)
                            });
                let ($(ref $id),*) = *self;
                ($($id.fold($sm.into_iter())),*)
            }
        }
    }
}

tuple_folder!({A, B},
              {A_sm, B_sm},
              {A_acc, B_acc},
              {A_item, B_item});
tuple_folder!({A, B, C},
              {A_sm, B_sm, C_sm},
              {A_acc, B_acc, C_acc},
              {A_item, B_item, C_item});
tuple_folder!({A, B, C, D},
              {A_sm, B_sm, C_sm, D_sm},
              {A_acc, B_acc, C_acc, D_acc},
              {A_item, B_item, C_item, D_item});
tuple_folder!({A, B, C, D, E},
              {A_sm, B_sm, C_sm, D_sm, E_sm},
              {A_acc, B_acc, C_acc, D_acc, E_acc},
              {A_item, B_item, C_item, D_item, E_item});
tuple_folder!({A, B, C, D, E, F},
              {A_sm, B_sm, C_sm, D_sm, E_sm, F_sm},
              {A_acc, B_acc, C_acc, D_acc, E_acc, F_acc},
              {A_item, B_item, C_item, D_item, E_item, F_item});
tuple_folder!({A, B, C, D, E, F, G},
              {A_sm, B_sm, C_sm, D_sm, E_sm, F_sm, G_sm},
              {A_acc, B_acc, C_acc, D_acc, E_acc, F_acc, G_acc},
              {A_item, B_item, C_item, D_item, E_item, F_item, G_item});
tuple_folder!({A, B, C, D, E, F, G, H},
              {A_sm, B_sm, C_sm, D_sm, E_sm, F_sm, G_sm, H_sm},
              {A_acc, B_acc, C_acc, D_acc, E_acc, F_acc, G_acc, H_acc},
              {A_item, B_item, C_item, D_item, E_item, F_item, G_item, H_item});
tuple_folder!({A, B, C, D, E, F, G, H, I},
              {A_sm, B_sm, C_sm, D_sm, E_sm, F_sm, G_sm, H_sm, I_sm},
              {A_acc, B_acc, C_acc, D_acc, E_acc, F_acc, G_acc, H_acc, I_acc},
              {A_item, B_item, C_item, D_item, E_item, F_item, G_item, H_item, I_item});
tuple_folder!({A, B, C, D, E, F, G, H, I, J},
              {A_sm, B_sm, C_sm, D_sm, E_sm, F_sm, G_sm, H_sm, I_sm, J_sm},
              {A_acc, B_acc, C_acc, D_acc, E_acc, F_acc, G_acc, H_acc, I_acc, J_acc},
              {A_item, B_item, C_item, D_item, E_item, F_item, G_item, H_item, I_item, J_item});
tuple_folder!({A, B, C, D, E, F, G, H, I, J, K},
              {A_sm, B_sm, C_sm, D_sm, E_sm, F_sm, G_sm, H_sm, I_sm, J_sm, K_sm},
              {A_acc, B_acc, C_acc, D_acc, E_acc, F_acc, G_acc, H_acc, I_acc, J_acc, K_acc},
              {A_item, B_item, C_item, D_item, E_item, F_item, G_item, H_item, I_item, J_item, K_item});
tuple_folder!({A, B, C, D, E, F, G, H, I, J, K, L},
              {A_sm, B_sm, C_sm, D_sm, E_sm, F_sm, G_sm, H_sm, I_sm, J_sm, K_sm, L_sm},
              {A_acc, B_acc, C_acc, D_acc, E_acc, F_acc, G_acc, H_acc, I_acc, J_acc, K_acc, L_acc},
              {A_item, B_item, C_item, D_item, E_item, F_item, G_item, H_item, I_item, J_item, K_item, L_item});

/// Null folder that only produces only tuples.
pub struct NullFolder;

impl<Item, M> Folder<Item, M> for NullFolder {
    type Sum = ();

    fn gather<'a>(&self, _: M, _: &'a Item) -> Self::Sum {}

    fn fold<I>(&self, _: I) -> Self::Sum
    where
        I: Iterator<Item = Self::Sum>,
    {
    }
}

/// This defines a region from [-2**n, 2**n).
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct LeveledRegion(pub i32);

impl LeveledRegion {
    /// This allows the discretization of a `Vector3` `point` to a morton code using the region.
    /// If the point is not in the region it gives back `None`.
    ///
    /// ```
    /// let region = space::LeveledRegion(0);
    /// // This is inside the bounds, so it gives back `Some(morton)`.
    /// let inside_bounds = nalgebra::Vector3::new(0.5, 0.5, 0.5);
    /// assert!(region.discretize::<f32, u64>(inside_bounds).is_some());
    /// // This is outside the bounds, so it gives back `None`.
    /// let outside_bounds = nalgebra::Vector3::new(1.5, 1.5, 1.5);
    /// assert!(region.discretize::<f32, u64>(outside_bounds).is_none());
    /// ```
    pub fn discretize<S, M>(self, point: Vector3<S>) -> Option<M>
    where
        S: Float + ToPrimitive + FromPrimitive + std::fmt::Debug + 'static,
        M: Morton + std::fmt::Debug + 'static,
    {
        let bound = (S::one() + S::one()).powi(self.0);
        if point.iter().any(|n| n.abs() > bound) {
            None
        } else {
            // Convert the point into normalized space.
            let MortonWrapper(m) =
                (point.map(|n| (n + bound) / (S::one() + S::one()).powi(self.0 + 1))).into();
            Some(m)
        }
    }
}

/// Defines a ```LeveledRegion``` from [-2^n, 2^n) shifted so that it is centered at ```center```.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct CenteredLeveledRegion<S>
where
    S: Float + ToPrimitive + FromPrimitive + PartialOrd + std::fmt::Debug + 'static,
{
    /// Represents a region from [-2**n, 2**n) shifted by center
    pub leveled_region: LeveledRegion,

    /// Represents the center of the CenteredLeveledRegion
    #[serde(bound(
        serialize = "Vector3<S>: Serialize",
        deserialize = "Vector3<S>: Deserialize<'de>"
    ))]
    pub center: Vector3<S>,
}

impl<S> CenteredLeveledRegion<S>
where
    S: Float + ToPrimitive + FromPrimitive + PartialOrd + std::fmt::Debug + Copy + 'static,
{
    /// Return octant where old points should be placed upon resizing
    /// based upon the the position of the new point
    pub fn expand_loc(&self, point: Vector3<S>) -> Option<u8> {
        let radius: S = S::from(2.0.powi(self.leveled_region.0) as f64)
            .expect("space::CenteredLeveledRegion::expand_loc: Unable to convert f64 to S");
        let lower_bound: Vector3<S> = self.center.map(|p| p - radius);
        let upper_bound: Vector3<S> = self.center.map(|p| p + radius);

        // Octant where the old region should lie based upon the new point
        // (-1 if within), note 1 more positive, 0 more negative
        // [0] x, [1] y, [2] z
        let new_octant: Vec<i32> = (0..3)
            .map(
                |i| match (point[i] < lower_bound[i], point[i] >= upper_bound[i]) {
                    (true, _) => 1, // new point is less than existing region
                    (_, true) => 0, // new point is greater than existing region
                    _ => -1,        // new point is within existing region
                },
            )
            .collect();

        if (new_octant[0] == -1) && (new_octant[1] == -1) && (new_octant[2] == -1) {
            None
        } else {
            let preferred_octant: Vec<i32> = (0..3)
                .map(|i| match (new_octant[i], point[i] < self.center[i]) {
                    (-1, true) => 1,
                    (-1, false) => 0,
                    (x, _) => x,
                })
                .collect();
            Some(
                // zyx format
                ((preferred_octant[2] << 2) | (preferred_octant[1] << 1) | preferred_octant[0])
                    as u8,
            )
        }
    }

    /// Allows discretization of a point, taking into account the shifted center of the
    /// ```CenteredLeveledRegion```.
    pub fn discretize<M>(self, point: Vector3<S>) -> Option<M>
    where
        M: Morton + std::fmt::Debug + 'static,
        S: nalgebra::base::Scalar + alga::general::ClosedSub,
    {
        self.leveled_region.discretize(point - self.center)
    }

    /// Expand the ```CenteredLeveledRegion``` by one "notch" (1 level of the ```LeveledRegion```)
    /// The octant represents the octant where the old points should be moved
    /// (as in the ```expand_loc``` function)
    pub fn expand(&mut self, octant: u8)
    where
        S: std::ops::AddAssign,
    {
        // Adjust center
        let center_adjust: Vector3<S> = Vector3::from_iterator((0..3).map(|i| {
            // New octant is in the positive half, so the center is shifted left (negative)
            if octant & (1 << i) != 0 {
                S::from_f64(-(2.0.powi(self.leveled_region.0)) as f64)
                    .expect("space::CenteredLeveledRegion::expand: Unable to convert f64 to S")
            } else {
                S::from_f64(2.0.powi(self.leveled_region.0) as f64)
                    .expect("space::CenteredLeveledRegion::expand: Unable to convert f64 to S")
            }
        }));

        self.center += center_adjust;
        self.leveled_region.0 += 1;
    }
}
