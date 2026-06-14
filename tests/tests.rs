use bounding_box::*;

#[test]
fn test_intersects() {
    let bb1 = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    let bb2 = BoundingBox::new(-0.5, 0.5, -0.5, 1.5);
    assert!(bb1.intersects(&bb2));

    let bb1 = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    let bb2 = BoundingBox::new(-0.5, 0.5, -0.5, 1.5);
    assert!(bb1.intersects(&bb2));

    let bb1 = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    let bb2 = BoundingBox::new(0.2, 0.8, std::f64::NEG_INFINITY, 0.5);
    assert!(bb1.intersects(&bb2));

    let bb1 = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    let bb2 = BoundingBox::new(0.2, 0.8, 0.5, std::f64::INFINITY);
    assert!(bb1.intersects(&bb2));

    let bb1 = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    let bb2 = BoundingBox::new(0.2, 0.8, std::f64::NEG_INFINITY, std::f64::INFINITY);
    assert!(bb1.intersects(&bb2));

    let bb1 = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    let bb2 = BoundingBox::new(std::f64::NEG_INFINITY, std::f64::INFINITY, 0.2, 0.8);
    assert!(bb1.intersects(&bb2));
}

#[test]
fn test_covers() {
    let bb1 = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    let bb2 = BoundingBox::new(-0.5, 1.5, -0.5, 1.5);
    assert!(bb2.covers(&bb1));
    assert!(!bb1.covers(&bb2));

    let bb1 = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    let bb2 = BoundingBox::new(0.2, 1.0, 0.2, 0.8);
    assert!(bb1.covers(&bb2));
}

#[test]
fn test_touches() {
    // bb1 touches bb2
    let bb1 = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    let bb2: BoundingBox = BoundingBox::new(1.0, 2.0, 0.0, 1.0);
    assert!(bb2.touches(&bb1));
    assert!(bb1.touches(&bb2));

    // bb1 intersects bb2
    let bb1 = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    let bb2: BoundingBox = BoundingBox::new(0.8, 2.0, 0.0, 1.0);
    assert!(!bb2.touches(&bb1));
    assert!(!bb1.touches(&bb2));
}

#[test]
fn test_is_finite() {
    let bb = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    assert!(bb.is_finite());

    let bb = BoundingBox::new(0.0, 1e14, 0.0, 1.0);
    assert!(bb.is_finite());

    let bb = BoundingBox::new(0.0, std::f64::INFINITY, 0.0, 1.0);
    assert!(!bb.is_finite());

    let bb = BoundingBox::new(std::f64::NEG_INFINITY, 1.0, 0.0, 1.0);
    assert!(!bb.is_finite());

    let bb = BoundingBox::new(-10.0, 1.0, std::f64::NEG_INFINITY, 1.0);
    assert!(!bb.is_finite());

    let bb = BoundingBox::new(-10.0, 1.0, 2.0, std::f64::INFINITY);
    assert!(!bb.is_finite());
}

#[test]
fn test_impl_to_bounding_box() {
    struct Dummy;

    impl From<&Dummy> for BoundingBox {
        fn from(_: &Dummy) -> Self {
            return BoundingBox::new(0.0, 1.0, 0.0, 1.0);
        }
    }

    let dummy = Dummy {};
    let test_bb = BoundingBox::new(0.0, 1.0, 0.0, 1.0);

    let bb: BoundingBox = (&dummy).into();
    assert_eq!(bb, test_bb);

    // Use reference
    let bb = BoundingBox::from(&dummy);
    assert_eq!(bb, test_bb);

    // Consuming
    let bb = BoundingBox::from(&dummy);
    assert_eq!(bb, test_bb);
}

#[test]
fn test_covers_point() {
    let bb = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    assert!(bb.covers_point([0.0, 0.0]));
    assert!(!bb.covers_point([2.0, 0.0]));
    assert!(!bb.covers_point([2.0, 2.0]));
    assert!(bb.covers_point([0.7, 0.9]));
}

#[test]
fn test_from_nalgebra() {
    use nalgebra::{Point2, Vector2};

    let bb = BoundingBox::new(0.0, 1.0, 0.0, 1.0);
    assert!(bb.covers_point(Point2::new(0.0, 0.0)));
    assert!(bb.covers_point(Vector2::new(0.0, 0.0)));
}

#[test]
fn test_from_bounded_entities() {
    struct Box {
        xmin: f64,
        xmax: f64,
        ymin: f64,
        ymax: f64,
    }

    impl From<&Box> for BoundingBox {
        fn from(value: &Box) -> Self {
            return BoundingBox::new(value.xmin, value.xmax, value.ymin, value.ymax);
        }
    }

    {
        let box1 = Box {
            xmin: 0.0,
            xmax: 1.0,
            ymin: 0.0,
            ymax: 1.0,
        };
        let box2 = Box {
            xmin: 0.5,
            xmax: 2.0,
            ymin: 0.5,
            ymax: 2.0,
        };

        let bb = BoundingBox::from_bounded_entities([&box1, &box2].into_iter()).unwrap();
        assert_eq!(bb.xmin(), 0.0);
        assert_eq!(bb.xmax(), 2.0);
        assert_eq!(bb.ymin(), 0.0);
        assert_eq!(bb.ymax(), 2.0);
    }
}

#[test]
fn test_to_bounding_box() {
    struct Circle {
        center: [f64; 2],
        radius: f64,
    }

    impl ToBoundingBox for Circle {
        fn bounding_box(&self) -> BoundingBox {
            return BoundingBox::new(
                self.center[0] - self.radius,
                self.center[0] + self.radius,
                self.center[1] - self.radius,
                self.center[1] + self.radius,
            );
        }
    }

    let c = Circle {
        center: [2.0, 2.0],
        radius: 2.0,
    };
    let bb = c.bounding_box();
    assert_eq!(bb.xmin(), 0.0);
    assert_eq!(bb.ymin(), 0.0);
    assert_eq!(bb.xmax(), 4.0);
    assert_eq!(bb.ymax(), 4.0);
}
