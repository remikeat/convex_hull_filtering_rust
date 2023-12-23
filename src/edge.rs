use crate::point::Point;
use std::f64::EPSILON;

#[derive(Debug)]
pub struct Edge<'a>(pub &'a Point, pub &'a Point);

impl<'a> Edge<'a> {
    pub fn dot(&self, edge_b: &Edge) -> f64 {
        let Edge(em, e) = self;
        let Edge(bm, b) = edge_b;
        return (e.x - em.x) * (b.x - bm.x) + (e.y - em.y) * (b.y - bm.y);
    }

    pub fn cross_prod_z(&self, edge_b: &Edge) -> f64 {
        let Edge(em, e) = self;
        let Edge(bm, b) = edge_b;
        return (e.x - em.x) * (b.y - bm.y) - (e.y - em.y) * (b.x - bm.x);
    }

    pub fn get_angle(&self, pt: &Point) -> f64 {
        let a = Edge(pt, self.0);
        let b = Edge(pt, self.1);
        let cross = a.cross_prod_z(&b);
        let dot = a.dot(&b);
        return cross.atan2(dot);
    }

    pub fn belong_to_half_plane(&self, pt: &Point) -> bool {
        let Edge(em, _) = self;
        return self.cross_prod_z(&Edge(em, pt)) >= 0.0f64;
    }

    pub fn intersection(&self, q_dot: &Edge) -> Option<Point> {
        let Edge(em, e) = self;
        let Edge(qm, q) = q_dot;
        // Solve for em + ke * (e - em) == qm + kq * (q - qm)
        // ie ke * (e - em) - kq * (q - qm) == qm - em
        // ie ke * (e.x - em.x) - kq * (q.x - qm.x) == qm.x - em.x
        //    ke * (e.y - em.y) - kq * (q.y - qm.y) == qm.y - em.y
        // This system of equations can be solved with Cramer's Rule
        let det = self.cross_prod_z(q_dot);
        if f64::abs(det) > EPSILON {
            let rhsx = qm.x - em.x;
            let rhsy = qm.y - em.y;
            let ke = (rhsx * (q.y - qm.y) - rhsy * (q.x - qm.x)) / det;
            let kq = -((e.x - em.x) * rhsy - (e.y - em.y) * rhsx) / det;
            if 0.0f64 <= ke && ke <= 1.0f64 && 0.0f64 <= kq && kq <= 1.0f64 {
                return Some(Point {
                    x: em.x + ke * (e.x - em.x),
                    y: em.y + ke * (e.y - em.y),
                });
            }
        }
        None
    }
}
