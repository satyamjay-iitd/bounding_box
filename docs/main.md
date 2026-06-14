> **Feedback welcome!**  
> Found a bug, missing docs, or have a feature request?  
> Please open an issue on [GitHub](https://github.com/StefanMathis/bounding_box.git).

A [minimum two-dimensional bounding box / bounding rectangle](https://en.wikipedia.org/wiki/Minimum_bounding_rectangle)
describes the extents of an entity (shape, point set, line, ...) or a collection
thereof in x-y coordinates. 

Bounding boxes are very useful in computational geometry. For example, if the
bounding boxes of two entities don't intersect, the entities themselves also
don't intersect. This property can be used to short-circuit intersection
algorithms. In a similar fashion, they can be used as a first stage of an
algorithm which checks if an entity covers a point. 

Another use case is to find the minimum space required for displaying an entity
on a rectangular monitor. By comparing the bounding box to the actually
available monitor space, scaling factors can be obtained so the entire entity
can be shown on the monitor at once.

This library offers a lightweight struct [`BoundingBox`] (defined by only four
`f64`) which has various methods to e.g. calculate its dimensions, find its
center, transform it, unite it with other [`BoundingBox`] instances, find
intersections between [`BoundingBox`] instances and many more ...

Additionally, the trait [`ToBoundingBox`] is defined as a standardized interface
for deriving a [`BoundingBox`] from a type `T`. Implementing it also provides
a `From<&T> for BoundingBox` implementation.

The following code snippet shows how a [`BoundingBox`] can be used with a
`Circle` type:

```rust
use bounding_box::*;

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

let c1 = Circle {center: [0.0, 0.0], radius: 1.0};
let c2 = Circle {center: [0.0, 3.0], radius: 1.0};
let c3 = Circle {center: [0.0, 2.0], radius: 2.0};

// From(&TY) is auto-implemented
assert_eq!(BoundingBox::from(&c1), c1.bounding_box());

// ===============================================================
// Intersection

/// This is an incomplete example of an intersection algorithm
fn circles_intersect(left: &Circle, right: &Circle) -> &'static str {
    let bb_l: BoundingBox = left.into();
    let bb_r: BoundingBox = right.into();

    // Short-circuit the evaluation here
    if !(bb_l.intersects(&bb_r)) {
        return "The circles definitely don't intersect!";
    }

    // Implement the detailed (expensive) algorithm here
    // ...

    return "The circles might intersect ...";
}

assert_eq!(circles_intersect(&c1, &c2), "The circles definitely don't intersect!");
assert_eq!(circles_intersect(&c1, &c3), "The circles might intersect ...");

// ===============================================================
// Contains a point

/// This is an incomplete example of a containment check algorithm
fn circle_covers_point(c: &Circle, pt: [f64; 2]) -> &'static str {
    if !BoundingBox::from(c).covers_point(pt) {
        return "The point is not within the circle!";
    }

    // Implement the detailed (expensive) algorithm here
    // ...

    return "The point might be within the circle ...";
}

assert_eq!(circle_covers_point(&c1, [5.0, 1.0]), "The point is not within the circle!");
assert_eq!(circle_covers_point(&c1, [0.0, 0.5]), "The point might be within the circle ...");

// ===============================================================
// Find the common bounding box of all circles

// Using an iterator
let bb_common_iter = BoundingBox::from_bounded_entities([&c1, &c2, &c3].into_iter()).expect("iterator has at least one element");
assert_eq!(bb_common_iter.xmin(), -2.0);
assert_eq!(bb_common_iter.xmax(), 2.0);
assert_eq!(bb_common_iter.ymin(), -1.0);
assert_eq!(bb_common_iter.ymax(), 4.0);

// Alternatively, the bounding box could also be found by manually uniting the individual bounding boxes
let bb_common_man = BoundingBox::from(&c1).union(&BoundingBox::from(&c2).union(&BoundingBox::from(&c3)));
assert_eq!(bb_common_man, bb_common_iter);
```

The following code snippet shows how to find the smallest bounding box containing a given vertex set.

```rust
use bounding_box::BoundingBox;

let bb_all_pts = BoundingBox::from_points([[-1.0, -1.0], [1.0, 1.0]].into_iter()).expect("iterator has at least one element");
assert_eq!(bb_all_pts.xmin(), -1.0);
assert_eq!(bb_all_pts.xmax(), 1.0);
assert_eq!(bb_all_pts.ymin(), -1.0);
assert_eq!(bb_all_pts.ymax(), 1.0);
```

# Feature flags

All features are disabled by default.

## Serialization and deserialization

Bounding boxes can be serialized and deserialized using the
[serde](https://crates.io/crates/serde) crate.

This functionality is gated behind the `serde` feature flag.

## Tolerances

Some methods of [`BoundingBox`] are gated behind the  `approx ` feature flag.
Enabling this flag adds the [approxim](https://crates.io/crates/approxim) crate as a
dependency. The gated methods are prefixed with `approx_` and are variants of
other methods which habe absolute and ULPs (units of least precision) tolerances
as additional arguments. For example, [`approx_covers_point`] is the tolerance 
variant of [`covers_point`] and checks if a given point is *approximately*
covered by the bounding box.