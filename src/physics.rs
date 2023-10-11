use ggez::glam::Vec2;

#[derive(Clone, Debug)]
pub struct Polygon2D {
    pub verts: Vec<Vec2>,
    pub normals: Vec<Vec2>,
    pub is_trigger: bool,
    trigger_function: Option<fn(&mut Polygon2D, &mut Polygon2D)>, // TODO: should this be a `Box<dyn FnMut>` or something else?
                                                                  // Referenced: https://stackoverflow.com/questions/27831944/how-do-i-store-a-closure-in-a-struct-in-rust/27832320
                                                                  // Picked the simplest but least performant (I think) option
                                                                  // Note: this feels like anti-functional programming with all the potential side-effects...
}

impl Polygon2D {
    pub fn new(verts: impl IntoIterator<Item = Vec2>) -> Polygon2D {
        let verts = verts.into_iter().collect::<Vec<_>>();
        assert!(
            verts.len() >= 3,
            "polygon must consist of at least 3 vertices"
        );

        let normals = verts
            .windows(2)
            .map(|pair| [pair[0], pair[1]])
            .chain([[*verts.first().unwrap(), *verts.last().unwrap()]])
            .map(|[a, b]| {
                let edge = a - b;
                Vec2::new(-edge.y, edge.x)
                    .try_normalize()
                    .expect("failed to normalize normal")
            })
            .collect::<Vec<_>>();

        Polygon2D {
            verts,
            normals,
            is_trigger: false,
            trigger_function: None,
        }
    }

    pub fn set_trigger(
        &self,
        is_trigger: bool,
        trigger_function: fn(&mut Polygon2D, &mut Polygon2D),
    ) -> Polygon2D {
        Polygon2D {
            verts: self.verts.clone(),
            normals: self.normals.clone(),
            is_trigger,
            trigger_function: Some(trigger_function),
        }
    }

    pub fn new_line(start: Vec2, end: Vec2, thickness: f32) -> Polygon2D {
        let start_to_end = (end - start)
            .try_normalize()
            .expect("lines are generally not points");

        let p1 = (-start_to_end + Vec2::new(-start_to_end.y, start_to_end.x)) * thickness + start;
        let p2 = (-start_to_end + Vec2::new(start_to_end.y, -start_to_end.x)) * thickness + start;

        let p3 = (start_to_end + Vec2::new(start_to_end.y, -start_to_end.x)) * thickness + end;
        let p4 = (start_to_end + Vec2::new(-start_to_end.y, start_to_end.x)) * thickness + end;

        Polygon2D::new([p1, p2, p3, p4])
    }
}

pub struct PhysicsWorld {
    positions: Vec<Vec2>,
    colliders: Vec<Polygon2D>,
    //
    // FIXME: no broadphase
}

impl PhysicsWorld {
    pub fn new() -> PhysicsWorld {
        PhysicsWorld {
            positions: vec![],
            colliders: vec![],
        }
    }

    pub fn new_entity(&mut self, pos: Vec2, collider: Polygon2D) -> usize {
        let id = self.positions.len();
        self.positions.push(pos);
        self.colliders.push(collider);
        id
    }

    pub fn position(&self, entity: usize) -> Vec2 {
        self.positions[entity]
    }

    pub fn move_entity_to(&mut self, entity: usize, p1: Vec2) {
        // let c1 = &self.colliders[entity];

        let collision = check_entity(p1, entity, &self.positions, &mut self.colliders);
        match collision {
            CollisionResult::NoCollision => self.positions[entity] = p1,
            CollisionResult::Ya(mtv) => self.positions[entity] = p1 + mtv,
            CollisionResult::Reset => (),
        }
    }

    pub fn move_entity_by(&mut self, entity: usize, delta: Vec2) {
        let to = self.positions[entity] + delta;
        self.move_entity_to(entity, to);
    }
}

enum CollisionResult {
    NoCollision,
    Ya(Vec2),
    Reset,
}

/// Given a collider c1 moving to position p1, returns any collisions with other valid colliders moving to new positions.
///
/// skip: sets the entity to which c1 belongs, to skip over in calculations
fn check_entity(
    mut p1: Vec2,
    skip: usize,
    positions: &[Vec2],
    colliders: &mut [Polygon2D],
) -> CollisionResult {
    let orig_p1 = p1;

    let mut recheck_collisions = true;
    let mut iterations = 0;
    'outer: while recheck_collisions {
        recheck_collisions = false;

        if iterations >= 5 {
            return CollisionResult::Reset;
        }
        iterations += 1;

        for (n, c2) in colliders.iter_mut().enumerate() {
            if n == skip {
                continue;
            }
            let c1 = &mut colliders[skip];

            let p2 = positions[n];

            if let Some(collision) = check_pair(p1, c1, p2, c2) {
                if c1.is_trigger {
                    match c1.trigger_function {
                        Some(f) => {
                            f(&mut c1, &mut c2); // TODO: here's the problem - allowing for mutating state isn't permitted currently
                        }
                        None => (),
                    }
                }
                p1 += collision;
                recheck_collisions = true;
                continue 'outer;
            }
        }
    }
    let c1 = &mut colliders[skip];
    if c1.is_trigger {
        CollisionResult::NoCollision
    } else if orig_p1 == p1 {
        CollisionResult::NoCollision
    } else {
        CollisionResult::Ya(p1 - orig_p1)
    }
}

/// Finds the smallest overlapping vector between
fn check_pair(p1: Vec2, c1: &Polygon2D, p2: Vec2, c2: &Polygon2D) -> Option<Vec2> {
    // If the latter is a trigger, nothing needs to be done
    if c2.is_trigger {
        return None; // c2.trigger_function will be called when its collision checks are done
    }

    // The smallest overlapping vector between the two polygons
    let mut smallest_intersect = None;

    // Collect the normals of both polygons, then iterate through the list
    // This finds the smallest intersecting vector
    for &axis in c1.normals.iter().chain(c2.normals.iter()) {
        let calc_min_max = |position: Vec2, collider: &Polygon2D| {
            let (mut min, mut max) = {
                let first = (collider.verts[0] + position).dot(axis);
                (first, first)
            };

            for &v in &collider.verts {
                let projected = (v + position).dot(axis);
                if projected < min {
                    min = projected;
                }
                if projected > max {
                    max = projected;
                }
            }

            (min, max)
        };

        let (min1, max1) = calc_min_max(p1, c1);
        let (min2, max2) = calc_min_max(p2, c2);

        // line x line collision check

        // fast path for not intersecting
        if max1 <= min2 || min1 >= max2 {
            return None;
        }

        let mtv = if min1 >= min2 && min1 <= max2 && max1 >= min2 && max1 <= max2 {
            // `c1` is entirely within `c2
            // .mn2   .mn1   .mx1  .mx2
            let left = min2 - max1;
            let right = max2 - min1;

            use std::cmp::Ordering;
            match left.abs().total_cmp(&right) {
                Ordering::Less | Ordering::Equal => left,
                Ordering::Greater => right,
            }
        } else if max2 <= max1 && max2 >= min1 && min2 >= min1 && min2 <= max1 {
            // `c2` is entirely within `c1
            // .mn1 .mn2 .mx2 .mx1
            let left = min1 - max2;
            let right = max1 - min2;

            use std::cmp::Ordering;
            match left.abs().total_cmp(&right) {
                Ordering::Less | Ordering::Equal => left,
                Ordering::Greater => right,
            }
        } else if max1 <= max2 && max1 >= min2 {
            // `max1` is inside of `c2`
            // `c1` being entirely inside `c2` is handled above
            // `max1` is inside of `c2` and `min1` is outside
            // .mn1   .mn2   .mx1  .mx2
            min2 - max1
        } else if min1 <= max2 && min1 >= min2 {
            // `min1` is inside of `c2`
            // `c1` being entirely inside `c2` is handled above
            // `min1` is inside of `c2` and `max1` is outside
            // .mn2   .mn1   .mx2  .mx1
            max2 - min1
        } else {
            unreachable!("collider projections intersecting even though no intersection detected");
        };

        match smallest_intersect {
            None => smallest_intersect = Some(mtv * axis),
            Some(real_mtv) if mtv.abs() <= real_mtv.length() => {
                smallest_intersect = Some(mtv * axis)
            }
            Some(_) => (),
        };
    }

    // Returns the smallest intersection, or panics if no intersection was found
    Some(smallest_intersect.expect("no mtv generated even though intersection should have occured"))
}
