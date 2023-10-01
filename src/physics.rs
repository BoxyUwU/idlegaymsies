use ggez::glam::Vec2;

#[derive(Clone, Debug)]
pub struct Polygon2D {
    pub verts: Vec<Vec2>,
    pub normals: Vec<Vec2>,
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

        Polygon2D { verts, normals }
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
        self.positions[entity] = p1;
        let c1 = &self.colliders[entity];

        let collision = check_entity(p1, c1, entity, &self.positions, &self.colliders);
        if let Some(mtv) = collision {
            self.positions[entity] += mtv;
        }
    }

    pub fn move_entity_by(&mut self, entity: usize, delta: Vec2) {
        let to = self.positions[entity] + delta;
        self.move_entity_to(entity, to);
    }
}

fn check_entity(
    p1: Vec2,
    c1: &Polygon2D,
    skip: usize,
    positions: &[Vec2],
    colliders: &[Polygon2D],
) -> Option<Vec2> {
    for (n, c2) in colliders.iter().enumerate() {
        if n == skip {
            continue;
        }

        let p2 = positions[n];

        if let Some(collision) = check_pair(p1, c1, p2, c2) {
            // FIXME: this only handles ONE collision but there could be infinitely many, especially after applying the resulting mtv
            return Some(collision);
        }
    }

    None
}

fn check_pair(p1: Vec2, c1: &Polygon2D, p2: Vec2, c2: &Polygon2D) -> Option<Vec2> {
    let mut smallest_intersect = None;

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

    Some(smallest_intersect.expect("no mtv generated even though intersection should have occured"))
}