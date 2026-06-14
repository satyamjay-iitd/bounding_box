/*!
[`ToBoundingBox`]: crate::ToBoundingBox
[`BoundingBox`]: crate::BoundingBox
[`covers_point`]: crate::BoundingBox::covers_point
[`approx_covers_point`]: crate::BoundingBox::approx_covers_point

A library for rectilinear, 2-dimensional bounding boxes.

 */
#![doc = include_str!("../docs/main.md")]
#![deny(missing_docs)]

#[cfg(feature = "approx")]
use approx::ulps_eq;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "bincode")]
use bincode::{Decode, Encode};

/**
A rectilinear, 2-dimensional [bounding box](https://en.wikipedia.org/wiki/Minimum_bounding_rectangle).

A 2-dimensional rectilinear bounding box is described by four values: minimum
x-value, maximum x-value, minimum y-value and and maximum y-value. This struct
can be created either from any type which implements [`Into<BoundingBox>`] or
from the constructors [`new`](BoundingBox::new) or
[`try_new`](BoundingBox::try_new). The values defining a bounding box (`xmin`,
`xmax`, `ymin`, `ymax`) are called "extremas".

Since a bounding box only consists of four f64 values (32 bytes), it is cheap to
copy, hence it implements the
[`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html) trait.

# Features

This struct can be serialized / deserialized if the `serde` feature is enabled.
 */
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bincode", derive(Encode, Decode))]
pub struct BoundingBox {
    xmin: f64,
    xmax: f64,
    ymin: f64,
    ymax: f64,
}

impl BoundingBox {
    /**
    Generates a bounding box from minimum and maximum x- and y-values.

    # Panics
    Panics if `xmin > xmax` or if `ymin > ymax`.

    # Examples

    ```
    use bounding_box::BoundingBox;

    let _ = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    ```

    This example panics because `xmin > xmax`.

    ```should_panic
    use bounding_box::BoundingBox;

    let _ = BoundingBox::new(2.0, 1.0, 0.0, 1.0);
    ```
     */
    pub fn new(xmin: f64, xmax: f64, ymin: f64, ymax: f64) -> Self {
        return Self::try_new(xmin, xmax, ymin, ymax)
            .expect("one of the conditions xmin <= xmax and ymin <= ymax is not fulfilled");
    }

    /**
    Like [`BoundingBox::new`], but returns `None` instead of panicking if `xmin > xmax` or if `ymin > ymax`.

    # Examples

    ```
    use bounding_box::BoundingBox;

    assert!(BoundingBox::try_new(0.0, 1.0, 0.0, 1.0).is_some());
    assert!(BoundingBox::try_new(2.0, 1.0, 0.0, 1.0).is_none());
    ```
     */
    pub fn try_new(xmin: f64, xmax: f64, ymin: f64, ymax: f64) -> Option<Self> {
        if xmin > xmax || ymin > ymax {
            return None;
        }
        return Some(BoundingBox {
            xmin,
            xmax,
            ymin,
            ymax,
        });
    }

    /**
    Returns the minimum x-value of the bounding box.
     */
    pub fn xmin(&self) -> f64 {
        return self.xmin;
    }

    /**
    Returns the maximum x-value of the bounding box.
     */
    pub fn xmax(&self) -> f64 {
        return self.xmax;
    }

    /**
    Returns the minimum y-value of the bounding box.
     */
    pub fn ymin(&self) -> f64 {
        return self.ymin;
    }

    /**
    Returns the maximum y-value of the bounding box.
     */
    pub fn ymax(&self) -> f64 {
        return self.ymax;
    }

    /**
    Fallible sets a new value for `xmin`. If the new value is bigger than `xmax`, the old value is left unchanged
    and this function returns `false`. Otherwise, it returns `true` and the old value is replaced by the new value.

    # Examples

    ```
    use bounding_box::BoundingBox;

    let mut bb = BoundingBox::new(0.0, 1.0, 0.0, 1.0);

    assert_eq!(bb.xmin(), 0.0);
    assert!(bb.try_set_xmin(0.5));
    assert_eq!(bb.xmin(), 0.5);

    assert!(!bb.try_set_xmin(1.5));
    assert_eq!(bb.xmin(), 0.5);
    ```
     */
    pub fn try_set_xmin(&mut self, val: f64) -> bool {
        if val > self.xmax {
            return false;
        } else {
            self.xmin = val;
            return true;
        }
    }

    /**
    Fallible sets a new value for `xmax`. If the new value is smaller than `xmin`, the old value is left unchanged
    and this function returns `false`. Otherwise, it returns `true` and the old value is replaced by the new value.

    # Examples

    ```
    use bounding_box::BoundingBox;

    let mut bb = BoundingBox::new(0.0, 1.0, 0.0, 1.0);

    assert_eq!(bb.xmax(), 1.0);
    assert!(bb.try_set_xmax(0.5));
    assert_eq!(bb.xmax(), 0.5);

    assert!(!bb.try_set_xmax(-0.5));
    assert_eq!(bb.xmax(), 0.5);
    ```
     */
    pub fn try_set_xmax(&mut self, val: f64) -> bool {
        if val < self.xmin {
            return false;
        } else {
            self.xmax = val;
            return true;
        }
    }

    /**
    Fallible sets a new value for `ymin`. If the new value is bigger than `ymax`, the old value is left unchanged
    and this function returns `false`. Otherwise, it returns `true` and the old value is replaced by the new value.

    # Examples

    ```
    use bounding_box::BoundingBox;

    let mut bb = BoundingBox::new(0.0, 1.0, 0.0, 1.0);

    assert_eq!(bb.ymin(), 0.0);
    assert!(bb.try_set_ymin(0.5));
    assert_eq!(bb.ymin(), 0.5);

    assert!(!bb.try_set_ymin(1.5));
    assert_eq!(bb.ymin(), 0.5);
    ```
     */
    pub fn try_set_ymin(&mut self, val: f64) -> bool {
        if val > self.ymax {
            return false;
        } else {
            self.ymin = val;
            return true;
        }
    }

    /**
    Fallible sets a new value for `ymax`. If the new value is smaller than `ymin`, the old value is left unchanged
    and this function returns `false`. Otherwise, it returns `true` and the old value is replaced by the new value.

    # Examples

    ```
    use bounding_box::BoundingBox;

    let mut bb = BoundingBox::new(0.0, 1.0, 0.0, 1.0);

    assert_eq!(bb.ymax(), 1.0);
    assert!(bb.try_set_ymax(0.5));
    assert_eq!(bb.ymax(), 0.5);

    assert!(!bb.try_set_ymax(-0.5));
    assert_eq!(bb.ymax(), 0.5);
    ```
     */
    pub fn try_set_ymax(&mut self, val: f64) -> bool {
        if val < self.ymin {
            return false;
        } else {
            self.ymax = val;
            return true;
        }
    }

    /**
    Creates a bounding box from an iterator over vertices.

    If the iterator is empty, this function returns `None`.
    ```
    use bounding_box::BoundingBox;

    let points = vec![
        [1.0, 0.0],
        [-5.0, 2.0],
        [3.0, -12.3],
        [7.0, 0.0],
        [2.0, 11.0],
        [1.0, -6.0],
    ];

     let bb = BoundingBox::from_points(points.into_iter()).expect("iterator yields at least one elment");
     assert_eq!(bb.xmin(), -5.0);
     assert_eq!(bb.xmax(), 7.0);
     assert_eq!(bb.ymin(), -12.3);
     assert_eq!(bb.ymax(), 11.0);
     ```
     */
    pub fn from_points<'a, T: Into<[f64; 2]>, I: Iterator<Item = T>>(
        mut points: I,
    ) -> Option<Self> {
        match points.next() {
            Some(pt) => {
                let pt: [f64; 2] = pt.into();
                let mut xmin = pt[0];
                let mut xmax = pt[0];
                let mut ymin = pt[1];
                let mut ymax = pt[1];
                for pt in points {
                    let pt: [f64; 2] = pt.into();
                    if pt[0] > xmax {
                        xmax = pt[0]
                    }
                    if pt[0] < xmin {
                        xmin = pt[0]
                    }
                    if pt[1] > ymax {
                        ymax = pt[1]
                    }
                    if pt[1] < ymin {
                        ymin = pt[1]
                    }
                }
                return Some(BoundingBox::new(xmin, xmax, ymin, ymax));
            }
            None => return None,
        }
    }

    /**
    Creates a bounding box from an iterator over any types implementing
    [`Into<BoundingBox>`].

    If the iterator is empty, this function returns `None`.

    ```
    use bounding_box::BoundingBox;

    struct Circle {
        center: [f64; 2],
        radius: f64
    }

    impl From<&Circle> for BoundingBox {
        fn from(c: &Circle) -> BoundingBox {
            return BoundingBox::new(c.center[0] - c.radius,
                                    c.center[0] + c.radius,
                                    c.center[1] - c.radius,
                                    c.center[1] + c.radius);
        }
    }

    let c1 = Circle {center: [0.0, 0.0], radius: 1.0};
    let c2 = Circle {center: [0.0, 2.0], radius: 1.0};
    let c3 = Circle {center: [0.0, 2.0], radius: 2.0};

    let bb = BoundingBox::from_bounded_entities([&c1, &c2, &c3].into_iter()).expect("iterator has at least one element");
    assert_eq!(bb.xmin(), -2.0);
    assert_eq!(bb.xmax(), 2.0);
    assert_eq!(bb.ymin(), -1.0);
    assert_eq!(bb.ymax(), 4.0);
     ```
     */
    pub fn from_bounded_entities<T: Into<BoundingBox>, I: Iterator<Item = T>>(
        mut entities: I,
    ) -> Option<Self> {
        let first_bb: BoundingBox = entities.next()?.into();
        let bb = entities.fold(first_bb, |acc, drawable| drawable.into().union(&acc));
        return Some(bb);
    }

    /**
    Creates the union of two bounding boxes.

    The union of two bounding boxes is the minimum bounding box which covers both bounding boxes.

    # Examples

    ```
    use bounding_box::BoundingBox;

    let bb1 = BoundingBox::new(-1.0, 3.5, 2.0, 3.0);
    let bb2 = BoundingBox::new(-5.0, 2.5, -1.0, 5.0);
    let bb = bb1.union(&bb2);

    assert_eq!(bb.xmin(), -5.0);
    assert_eq!(bb.xmax(), 3.5);
    assert_eq!(bb.ymin(), -1.0);
    assert_eq!(bb.ymax(), 5.0);
    ```
    */
    pub fn union(&self, other: &BoundingBox) -> BoundingBox {
        let xmin: f64;
        let xmax: f64;
        let ymin: f64;
        let ymax: f64;
        if self.xmin > other.xmin {
            xmin = other.xmin;
        } else {
            xmin = self.xmin;
        }
        if self.xmax > other.xmax {
            xmax = self.xmax;
        } else {
            xmax = other.xmax;
        }
        if self.ymin > other.ymin {
            ymin = other.ymin;
        } else {
            ymin = self.ymin;
        }
        if self.ymax > other.ymax {
            ymax = self.ymax;
        } else {
            ymax = other.ymax;
        }
        return BoundingBox {
            xmin,
            xmax,
            ymin,
            ymax,
        };
    }

    /**
    Returns true if `self` covers a given point.

    A point is "covered" by the bounding box if it is either within or on the
    boundaries of `self`. Mathematically speaking, the following two
    inequalities must be true:

    `self.xmin() <= point[0] <= self.xmax()`

    `self.ymin() <= point[1] <= self.ymax()`

    If the boundaries should be excluded, use [`BoundingBox::contains_point`]
    instead.

    If the feature flag `approx` is enabled, the method
    [`BoundingBox::approx_covers_point`] is made available, which allows
    providing tolerances for the aforementioned inequalities.

    # Examples
    ```
    use bounding_box::BoundingBox;

    let bb = BoundingBox::new(0.0, 1.0, 0.0, 1.0);

    assert!(bb.covers_point([0.5, 0.5]));
    assert!(bb.covers_point([0.0, 0.0])); // On boundary
    assert!(!bb.covers_point([-1.0, 0.0]));
    assert!(!bb.covers_point([0.0, 2.0]));
     */
    pub fn covers_point<T: Into<[f64; 2]>>(&self, point: T) -> bool {
        let point: [f64; 2] = point.into();
        return self.xmin <= point[0]
            && self.ymin <= point[1]
            && self.xmax >= point[0]
            && self.ymax >= point[1];
    }

    /**
    Like [`BoundingBox::covers_point`], but with absolute and ULPs tolerances.

    This variant of [`BoundingBox::covers_point`] allows specifying an absolute
    and an [ULP](https://en.wikipedia.org/wiki/Unit_in_the_last_place)
    tolerance. These tolerances  are used to check if the given point lies
    "approximately" on an edge of the bounding box.

    # Examples
    ```
    use bounding_box::BoundingBox;

    let bb = BoundingBox::new(0.0, 1.0, 0.0, 1.0);

    // Exact check: Point is outside the bounding box
    assert!(!bb.covers_point([1.0001, 1.0]));

    // Check using tolerances: Point is inside bounding box
    assert!(bb.approx_covers_point([1.0001, 1.0], 1e-3, 0));

    // Check using a finer tolerance: Point is outside the bounding box
    assert!(!bb.approx_covers_point([1.0001, 1.0], 1e-6, 0));
    ```

    # Features

    This function uses the [`ulps_eq`] macro of the [approx] crate, therefore
    the `approx` feature needs to be enabled.
     */
    #[cfg(feature = "approx")]
    pub fn approx_covers_point<T: Into<[f64; 2]>>(
        &self,
        point: T,
        epsilon: f64,
        max_ulps: u32,
    ) -> bool {
        let point: [f64; 2] = point.into();
        return (self.xmin < point[0]
            || ulps_eq!(self.xmin, point[0], epsilon = epsilon, max_ulps = max_ulps))
            && (self.ymin < point[1]
                || ulps_eq!(self.ymin, point[1], epsilon = epsilon, max_ulps = max_ulps))
            && (self.xmax > point[0]
                || ulps_eq!(self.xmax, point[0], epsilon = epsilon, max_ulps = max_ulps))
            && (self.ymax > point[1]
                || ulps_eq!(self.ymax, point[1], epsilon = epsilon, max_ulps = max_ulps));
    }

    /**
    Returns true if `self` contains a given point.

    A point is "contained" by the bounding box if it is within the boundaries of
    `self`. Mathematically speaking, the following two inequalities must be true:

    `self.xmin() < point[0] < self.xmax()`

    `self.ymin() < point[1] < self.ymax()`

    If the boundaries should be included, use [`BoundingBox::covers_point`]
    instead.

    # Examples
    ```
    use bounding_box::BoundingBox;

    let bb = BoundingBox::new(0.0, 1.0, 0.0, 1.0);

    assert!(bb.contains_point([0.5, 0.5]));
    assert!(!bb.contains_point([0.0, 0.0])); // On boundary
    assert!(!bb.contains_point([-1.0, 0.0]));
    assert!(!bb.contains_point([0.0, 2.0]));
     */
    pub fn contains_point<T: Into<[f64; 2]>>(&self, point: T) -> bool {
        let point: [f64; 2] = point.into();
        return self.xmin < point[0]
            && self.ymin < point[1]
            && self.xmax > point[0]
            && self.ymax > point[1];
    }

    /**
    Returns true if `self` covers `other`.

    A bounding box "covers" another bounding box, if every point covered by the
    second box is also covered by the first one according to the definition
    given in [`BoundingBox::covers_point`]:

    `self.xmin() <= other.xmin() <= other.xmax() <= self.xmax()`

    `self.ymin() <= other.ymin() <= other.ymax() <= self.ymax()`

    If the boundaries of `self` should be excluded, use [`BoundingBox::covers`]
    instead.

    If the feature flag `approx` is enabled, the method
    [`BoundingBox::approx_covers] is made available, which allows
    providing tolerances for the aforementioned inequalities.

    # Examples
    ```
    use bounding_box::BoundingBox;

    // bb1 covers bb2, but not the other way around
    let bb1 = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    let bb2 = BoundingBox::new(0.2, 0.8, 0.2, 0.8);
    assert!(bb1.covers(&bb2));
    assert!(!bb2.covers(&bb1));

    // bb1 covers itself
    assert!(bb1.covers(&bb1));

    // bb1 and bb2 share a border
    let bb1 = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    let bb2 = BoundingBox::new(0.2, 1.0, 0.2, 0.8);
    assert!(bb1.covers(&bb2));

    // bb1 and bb2 are separated from each other, and therefore neither one covers the other
    let bb1 = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    let bb2 = BoundingBox::new(2.0, 3.0, 2.0, 3.0);
    assert!(!bb1.covers(&bb2));
    assert!(!bb2.covers(&bb1));
    ```
    */
    pub fn covers(&self, other: &Self) -> bool {
        return self.xmin <= other.xmin
            && self.ymin <= other.ymin
            && self.xmax >= other.xmax
            && self.ymax >= other.ymax;
    }

    /**
    Like [`BoundingBox::covers`], but with absolute and ULPs tolerances.

    This variant of [`BoundingBox::covers`] allows specifying an absolute and
    an [ULP](https://en.wikipedia.org/wiki/Unit_in_the_last_place) tolerance.
    These tolerances are used to check if the extremas of the boxes are
    "approximately" equal.

    ```
    use bounding_box::BoundingBox;

    // bb1 covers bb2 depending on the selected tolerances
    let bb1 = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    let bb2 = BoundingBox::new(0.0, 1.0001, 0.0, 0.5);

    assert!(!bb1.covers(&bb2));
    assert!(bb1.approx_covers(&bb2, 1e-3, 0));
    assert!(!bb1.approx_covers(&bb2, 1e-6, 0));
    ```

    # Features

    This function uses the [`ulps_eq`] macro of the [approx] crate, therefore
    the `approx` feature needs to be enabled.
    */
    #[cfg(feature = "approx")]
    pub fn approx_covers(&self, other: &Self, epsilon: f64, max_ulps: u32) -> bool {
        return (self.xmin < other.xmin
            || ulps_eq!(
                self.xmin,
                other.xmin,
                epsilon = epsilon,
                max_ulps = max_ulps
            ))
            && (self.ymin < other.ymin
                || ulps_eq!(
                    self.ymin,
                    other.ymin,
                    epsilon = epsilon,
                    max_ulps = max_ulps
                ))
            && (self.xmax > other.xmax
                || ulps_eq!(
                    self.xmax,
                    other.xmax,
                    epsilon = epsilon,
                    max_ulps = max_ulps
                ))
            && (self.ymax > other.ymax
                || ulps_eq!(
                    self.ymax,
                    other.ymax,
                    epsilon = epsilon,
                    max_ulps = max_ulps
                ));
    }

    /**
    Returns true if `self` contains `other`.

    A bounding box "contains" another bounding box, if every point covered by
    the second box is also contained within the first one according to the
    definitions given in [`BoundingBox::covers_point`] and
    [`BoundingBox::contains_point`]:

    `self.xmin() < other.xmin() <= other.xmax() < self.xmax()`

    `self.ymin() < other.ymin() <= other.ymax() < self.ymax()`

    If the boundaries of `self` should be excluded, use [`BoundingBox::covers`]
    instead.

    If the feature flag `approx` is enabled, the method
    [`BoundingBox::approx_covers] is made available, which allows
    providing tolerances for the aforementioned inequalities.

    # Examples
    ```
    use bounding_box::BoundingBox;

    // bb1 contains bb2, but not the other way around
    let bb1 = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    let bb2 = BoundingBox::new(0.2, 0.8, 0.2, 0.8);
    assert!(bb1.contains(&bb2));
    assert!(!bb2.contains(&bb1));

    // bb1 does not contain itself
    assert!(!bb1.contains(&bb1));

    // bb1 and bb2 share a border
    let bb1 = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    let bb2 = BoundingBox::new(0.2, 1.0, 0.2, 0.8);
    assert!(!bb1.contains(&bb2));

    // bb1 and bb2 are separated from each other, and therefore neither one contains the other
    let bb1 = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    let bb2 = BoundingBox::new(2.0, 3.0, 2.0, 3.0);
    assert!(!bb1.contains(&bb2));
    assert!(!bb2.contains(&bb1));
    ```
    */
    pub fn contains(&self, other: &Self) -> bool {
        return self.xmin < other.xmin
            && self.ymin < other.ymin
            && self.xmax > other.xmax
            && self.ymax > other.ymax;
    }

    /**
    Check if the two bounding boxes are approximately equal.

    This check is performed using the [`ulps_eq`] macro of the [approx] crate.

    ```
    use bounding_box::BoundingBox;

    // bb1 covers bb2 depending on the selected tolerances
    let bb1 = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    let bb2 = BoundingBox::new(0.0, 1.0001, 0.0, 1.0);

    assert!(!bb1.eq(&bb2));
    assert!(bb1.approx_eq(&bb2, 1e-3, 0));
    assert!(!bb1.approx_eq(&bb2, 1e-6, 0));
    ```

    # Features

    This function uses the [`ulps_eq`](https://docs.rs/approx/latest/approx/macro.ulps_eq.html)
    macro of the [approx] crate, therefore the `approx ` feature needs to be enabled.
     */
    #[cfg(feature = "approx")]
    pub fn approx_eq(&self, other: &Self, epsilon: f64, max_ulps: u32) -> bool {
        return ulps_eq!(
            self.xmin(),
            other.xmin(),
            epsilon = epsilon,
            max_ulps = max_ulps
        ) && ulps_eq!(
            self.xmax(),
            other.xmax(),
            epsilon = epsilon,
            max_ulps = max_ulps
        ) && ulps_eq!(
            self.ymin(),
            other.ymin(),
            epsilon = epsilon,
            max_ulps = max_ulps
        ) && ulps_eq!(
            self.ymax(),
            other.ymax(),
            epsilon = epsilon,
            max_ulps = max_ulps
        );
    }

    /**
    Returns true if the bounding boxes intersect, i.e. if they both
    [cover](BoundingBox::covers) at least one common point.

    The boxes are intersecting if they are just
    [touching](BoundingBox::touches).

    # Examples
    ```
    use bounding_box::BoundingBox;

    // bb1 and bb2 intersect
    let bb1 = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    let bb2 = BoundingBox::new(-1.0, 1.0, 0.2, 0.8);
    assert!(bb1.intersects(&bb2));

     // bb1 and bb2 do not intersect
    let bb1 = BoundingBox::new(-1.0, 3.5, 2.0, 3.0);
    let bb2 = BoundingBox::new(-5.0, 2.5, -1.0, 1.0);
    assert!(!bb1.intersects(&bb2));

    // bb2 is contained in bb1
    let bb1 = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    let bb2 = BoundingBox::new(0.2, 1.0, 0.2, 0.8);
    assert!(bb1.intersects(&bb2));

    // bb1 is contained in bb2
    let bb1 = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    let bb2 = BoundingBox::new(0.2, 1.0, 0.2, 0.8);
    assert!(bb2.intersects(&bb1));

    // bb1 touches bb2 => intersection
    let bb1 = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    let bb2 = BoundingBox::new(1.0, 2.0, 0.0, 1.0);
    assert!(bb2.intersects(&bb1));
    ```
     */
    pub fn intersects(&self, other: &Self) -> bool {
        return self.xmin() <= other.xmax()
            && other.xmin() <= self.xmax()
            && self.ymin() <= other.ymax()
            && other.ymin() <= self.ymax();
    }

    /**
    Returns true if the bounding boxes overlap, i.e. if they both
    [contain](BoundingBox::contains) at least one common point.

    The boxes are NOT intersecting if they are just
    [touching](BoundingBox::touches).

    # Examples
    ```
    use bounding_box::BoundingBox;

    // bb1 and bb2 overlap
    let bb1 = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    let bb2 = BoundingBox::new(-1.0, 1.0, 0.2, 0.8);
    assert!(bb1.overlaps(&bb2));

     // bb1 and bb2 do not overlap
    let bb1 = BoundingBox::new(-1.0, 3.5, 2.0, 3.0);
    let bb2 = BoundingBox::new(-5.0, 2.5, -1.0, 1.0);
    assert!(!bb1.overlaps(&bb2));

    // bb2 is contained in bb1
    let bb1 = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    let bb2 = BoundingBox::new(0.2, 1.0, 0.2, 0.8);
    assert!(bb1.overlaps(&bb2));

    // bb1 is contained in bb2
    let bb1 = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    let bb2 = BoundingBox::new(0.2, 1.0, 0.2, 0.8);
    assert!(bb2.overlaps(&bb1));

    // bb1 touches bb2 => no overlapping
    let bb1 = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    let bb2 = BoundingBox::new(1.0, 2.0, 0.0, 1.0);
    assert!(!bb2.overlaps(&bb1));
    ```
     */
    pub fn overlaps(&self, other: &Self) -> bool {
        return self.xmin() < other.xmax()
            && other.xmin() < self.xmax()
            && self.ymin() < other.ymax()
            && other.ymin() < self.ymax();
    }

    /**
    Check if the bounding boxes are touching.

    The bounding boxes are touching if they share at least one extremum and are
    not intersecting each other.

    # Examples
    ```
    use bounding_box::BoundingBox;

    // bb1 touches bb2 => no intersection
    let bb1 = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    let bb2 = BoundingBox::new(1.0, 2.0, 0.0, 1.0);
    assert!(bb2.touches(&bb1));

    // bb1 is included in bb2 and two edges are touching
    let bb1 = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    let bb2 = BoundingBox::new(0.0, 2.0, 0.0, 1.0);
    assert!(!bb2.touches(&bb1));

    let bb1 = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    let bb2 = BoundingBox::new(1.0001, 2.0, 0.0, 1.0);
    assert!(!bb2.touches(&bb1));
    ```
     */
    pub fn touches(&self, other: &Self) -> bool {
        if self.overlaps(&other) {
            return false;
        } else {
            return self.xmin() == other.xmax()
                || self.xmax() == other.xmin()
                || self.ymin() == other.ymax()
                || self.ymax() == other.ymin();
        }
    }

    /**
    Like [`BoundingBox::touches`], but with absolute and ULPs tolerances.

    This variant of [`BoundingBox::touches`] allows specifying an absolute and
    an [ULP](https://en.wikipedia.org/wiki/Unit_in_the_last_place) tolerance. These tolerances
    are used to check if the boxes share at least one extremas "approximately".
    This check is performed using the [`ulps_eq`](https://docs.rs/approx/latest/approx/macro.ulps_eq.html)
    macro of the [approx] crate. Please see its documentation.

    # Examples
    ```
    use bounding_box::BoundingBox;

    let bb1 = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    let bb2 = BoundingBox::new(1.0001, 2.0, 0.0, 1.0);
    assert!(!bb1.touches(&bb2));
    assert!(bb1.approx_touches(&bb2, 1e-3, 0));
    assert!(!bb1.approx_touches(&bb2, 1e-6, 0));
    ```

    # Features

    This function uses the [`ulps_eq`](https://docs.rs/approx/latest/approx/macro.ulps_eq.html)
    macro of the [approx] crate, therefore the `approx ` feature needs to be enabled.
    */
    #[cfg(feature = "approx")]
    pub fn approx_touches(&self, other: &Self, epsilon: f64, max_ulps: u32) -> bool {
        if self.overlaps(&other) {
            return false;
        } else {
            return ulps_eq!(
                self.xmin(),
                other.xmax(),
                epsilon = epsilon,
                max_ulps = max_ulps
            ) || ulps_eq!(
                self.xmax(),
                other.xmin(),
                epsilon = epsilon,
                max_ulps = max_ulps
            ) || ulps_eq!(
                self.ymin(),
                other.ymax(),
                epsilon = epsilon,
                max_ulps = max_ulps
            ) || ulps_eq!(
                self.ymax(),
                other.ymin(),
                epsilon = epsilon,
                max_ulps = max_ulps
            );
        }
    }

    /**
    Returns the width of the bounding box.

    # Examples
    ```
    use bounding_box::BoundingBox;

    let bb = BoundingBox::new(-1.0, 1.0, 2.0, 7.0);
    assert_eq!(bb.width(), 2.0);
    ```
     */
    pub fn width(&self) -> f64 {
        return self.xmax - self.xmin;
    }

    /**
    Returns the height of the bounding box.

    # Examples
    ```
    use bounding_box::BoundingBox;

    let bb = BoundingBox::new(-1.0, 1.0, 2.0, 7.0);
    assert_eq!(bb.height(), 5.0);
    ```
     */
    pub fn height(&self) -> f64 {
        return self.ymax - self.ymin;
    }

    /**
    Returns the area of the bounding box.

    This is the product of [`BoundingBox::height`] times [`BoundingBox::width`].

    # Examples
    ```
    use bounding_box::BoundingBox;

    let bb = BoundingBox::new(-1.0, 1.0, 2.0, 7.0);
    assert_eq!(bb.area(), 10.0);
    assert_eq!(bb.height() * bb.width(), bb.area());
    ```
     */
    pub fn area(&self) -> f64 {
        return self.height() * self.width();
    }

    /**
    Returns the center of the bounding box.

    # Examples
    ```
    use bounding_box::BoundingBox;

    let bb = BoundingBox::new(-1.0, 1.0, 2.0, 7.0);
    assert_eq!(bb.center(), [0.0, 4.5]);
    ```
     */
    pub fn center(&self) -> [f64; 2] {
        let x = 0.5 * (self.xmax + self.xmin);
        let y = 0.5 * (self.ymax + self.ymin);
        return [x, y];
    }

    /**
    Translates the bounding box by the given `shift`.

    # Examples
    ```
    use bounding_box::BoundingBox;

    let mut bb = BoundingBox::new(0.0, 1.0, 1.0, 2.0);
    bb.translate([1.0, -1.0]);
    assert_eq!(bb.xmin(), 1.0);
    assert_eq!(bb.xmax(), 2.0);
    assert_eq!(bb.ymin(), 0.0);
    assert_eq!(bb.ymax(), 1.0);
    ```
     */
    pub fn translate<T: Into<[f64; 2]>>(&mut self, shift: T) -> () {
        let shift: [f64; 2] = shift.into();
        self.xmin += shift[0];
        self.xmax += shift[0];
        self.ymin += shift[1];
        self.ymax += shift[1];
    }

    /**
    Scales the width and height of `self` while keeping the center fixed.

    The bounding box is scaled by multiplying its width and height by the factor
    and then recalculating the extremas by adding / subtracting half the width / height
    from the center.

    # Examples
    ```
    use bounding_box::BoundingBox;

    let mut bb = BoundingBox::new(0.0, 1.0, 2.0, 4.0);

    assert_eq!(bb.center(), [0.5, 3.0]);
    assert_eq!(bb.width(), 1.0);
    assert_eq!(bb.height(), 2.0);

    bb.scale(2.0);

    assert_eq!(bb.center(), [0.5, 3.0]);
    assert_eq!(bb.width(), 2.0);
    assert_eq!(bb.height(), 4.0);

    assert_eq!(bb.xmin(), -0.5);
    assert_eq!(bb.xmax(), 1.5);
    assert_eq!(bb.ymin(), 1.0);
    assert_eq!(bb.ymax(), 5.0);
    ```
     */
    pub fn scale(&mut self, factor: f64) -> () {
        let dw = 0.5 * (factor - 1.0) * self.width();
        let dh = 0.5 * (factor - 1.0) * self.height();
        self.xmin = self.xmin - dw;
        self.xmax = self.xmax + dw;
        self.ymin = self.ymin - dh;
        self.ymax = self.ymax + dh;
    }

    /**
    Remove any singular dimensions by "buffering" them with `add_to_extr`.

    The value `add_to_extr` is applied to both the minimum and the maximum value of a singular dimension.
    For example, if the bounding box width is 0 (`xmin` = `xmax`) and `add_to_extr = 1.0`, the new width will be 2.0.

    # Examples
    ```
    use bounding_box::BoundingBox;

    let mut bb = BoundingBox::new(0.0, 0.0, -1.0, 1.0);
    assert_eq!(bb.width(), 0.0);

    bb.remove_singular_dimensions(1.0);
    assert_eq!(bb.width(), 2.0);
    assert_eq!(bb.xmin(), -1.0);
    assert_eq!(bb.xmax(), 1.0);

    // =================================================

    let mut bb = BoundingBox::new(-1.0, 1.0, 2.0, 2.0);
    assert_eq!(bb.height(), 0.0);

    bb.remove_singular_dimensions(3.0);
    assert_eq!(bb.height(), 6.0);
    assert_eq!(bb.ymin(), -1.0);
    assert_eq!(bb.ymax(), 5.0);
    ```
     */
    pub fn remove_singular_dimensions(&mut self, add_to_extr: f64) {
        if self.width() == 0.0 {
            self.xmin -= add_to_extr;
            self.xmax += add_to_extr;
        }
        if self.height() == 0.0 {
            self.ymin -= add_to_extr;
            self.ymax += add_to_extr;
        }
    }

    /**
    Returns true if the bounding box is finite.

    # Examples
    ```
    use std::f64::INFINITY;
    use bounding_box::BoundingBox;

    assert!(BoundingBox::new(0.0, 1.0, 0.0, 1.0).is_finite());
    assert!(!BoundingBox::new(0.0, INFINITY, 0.0, 1.0).is_finite());
    ```
     */
    pub fn is_finite(&self) -> bool {
        return self.xmin.is_finite()
            && self.xmax.is_finite()
            && self.ymin.is_finite()
            && self.ymax.is_finite();
    }
}

impl From<[f64; 2]> for BoundingBox {
    fn from(v: [f64; 2]) -> Self {
        return (&v).into();
    }
}

impl From<&'_ [f64; 2]> for BoundingBox {
    fn from(v: &'_ [f64; 2]) -> Self {
        return BoundingBox::new(v[0], v[0], v[1], v[1]);
    }
}

/**
This trait provides a standardized way of deriving a [`BoundingBox`] from
another type `T `with the [`bounding_box`](ToBoundingBox::bounding_box) method.

Implementing [`ToBoundingBox`] also auto-implements a [`From<&T>`] implementation
for [`BoundingBox`].

```
use bounding_box::{BoundingBox, ToBoundingBox};

struct Circle {
    center: [f64; 2],
    radius: f64
}

impl ToBoundingBox for Circle {
    fn bounding_box(&self) -> BoundingBox {
        return BoundingBox::new(self.center[0] - self.radius,
                                self.center[0] + self.radius,
                                self.center[1] - self.radius,
                                self.center[1] + self.radius);
    }
}

let c = Circle {center: [0.0, 0.0], radius: 1.0};
assert_eq!(c.bounding_box(), BoundingBox::from(&c));
```
 */
pub trait ToBoundingBox {
    /**
    Returns a bounding box for the implementor.

    # Example

    ```
    use bounding_box::{BoundingBox, ToBoundingBox};

    struct Circle {
        center: [f64; 2],
        radius: f64
    }

    impl ToBoundingBox for Circle {
        fn bounding_box(&self) -> BoundingBox {
            return BoundingBox::new(self.center[0] - self.radius,
                                    self.center[0] + self.radius,
                                    self.center[1] - self.radius,
                                    self.center[1] + self.radius);
        }
    }

    let c = Circle {center: [0.0, 0.0], radius: 1.0};
    let bb = c.bounding_box();
    assert_eq!(bb.xmin(), -1.0);
    assert_eq!(bb.ymin(), -1.0);
    assert_eq!(bb.xmax(), 1.0);
    assert_eq!(bb.ymax(), 1.0);
    ```
     */
    fn bounding_box(&self) -> BoundingBox;
}

impl<T: ToBoundingBox> From<&T> for BoundingBox {
    fn from(value: &T) -> Self {
        value.bounding_box()
    }
}
