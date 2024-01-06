use crate::edge::Edge;
use crate::point::Point;
use serde::Deserialize;
const EPSILON: f64 = 1e-6;

#[derive(Copy)]
pub enum Direction {
    Clockwise = 1,
    CounterClockwise = -1,
}

impl Clone for Direction {
    fn clone(&self) -> Self {
        *self
    }
}

enum WhichToAdvance {
    NotInit,
    PPoly,
    QPoly,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ConvexHull {
    #[serde(alias = "ID")]
    pub id: u32,
    pub apexes: Vec<Point>,
}

impl ConvexHull {
    fn get_point(&self, idx: i32) -> &Point {
        let n = self.apexes.len() as i32;
        let idx = ((idx % n) + n) % n;
        &self.apexes[idx as usize]
    }

    pub fn is_point_inside(&self, pt: &Point) -> bool {
        let mut sum_angles = 0.0f64;
        let nb_points_p = self.apexes.len();
        let mut iter = ConvexHullIterator::new(self, Direction::Clockwise);
        for _ in 0..nb_points_p {
            let p_dot = iter.next().unwrap();
            sum_angles += p_dot.get_angle(&pt);
        }
        // if the sum of the angles is not zero then the point is outside
        return f64::abs(sum_angles) > EPSILON;
    }

    pub fn get_direction(&self) -> Direction {
        let edge = Edge(&self.apexes[0], &self.apexes[1]);
        if edge.belong_to_half_plane(&self.apexes[2]) {
            Direction::Clockwise
        } else {
            Direction::CounterClockwise
        }
    }

    pub fn intersection(cvx_p: &ConvexHull, cvx_q: &ConvexHull) -> (bool, ConvexHull) {
        let n_p = cvx_p.apexes.len();
        let n_q = cvx_q.apexes.len();
        let mut iter_p = ConvexHullIterator::new(&cvx_p, cvx_p.get_direction());
        let mut iter_q = ConvexHullIterator::new(&cvx_q, cvx_q.get_direction());
        let mut p_dot = iter_p.next().unwrap();
        let mut q_dot = iter_q.next().unwrap();
        let mut cvx_inter: Vec<Point> = Vec::new();
        let mut inside = WhichToAdvance::NotInit;
        let mut first_inter: Option<Point> = None;
        for _ in 0..(2 * (n_p + n_q)) {
            match p_dot.intersection(&q_dot) {
                Some(iter) => {
                    if let Some(pt) = first_inter {
                        if iter == pt {
                            return (
                                true,
                                ConvexHull {
                                    id: 0,
                                    apexes: cvx_inter,
                                },
                            );
                        }
                    }
                    cvx_inter.push(iter);
                    if q_dot.belong_to_half_plane(p_dot.1) {
                        inside = WhichToAdvance::PPoly;
                    } else {
                        inside = WhichToAdvance::QPoly;
                    }
                    if let None = first_inter {
                        first_inter = Some(iter);
                    }
                }
                None => (),
            };
            let advance;
            if q_dot.cross_prod_z(&p_dot) >= 0.0f64 {
                if q_dot.belong_to_half_plane(p_dot.1) {
                    advance = WhichToAdvance::QPoly;
                } else {
                    advance = WhichToAdvance::PPoly;
                }
            } else {
                if p_dot.belong_to_half_plane(q_dot.1) {
                    advance = WhichToAdvance::PPoly;
                } else {
                    advance = WhichToAdvance::QPoly;
                }
            }
            match advance {
                WhichToAdvance::NotInit => panic!("Case that shouldn't happen"),
                WhichToAdvance::PPoly => {
                    match inside {
                        WhichToAdvance::PPoly => cvx_inter.push(*p_dot.1),
                        _ => (),
                    };
                    p_dot = iter_p.next().unwrap();
                }
                WhichToAdvance::QPoly => {
                    match inside {
                        WhichToAdvance::QPoly => cvx_inter.push(*q_dot.1),
                        _ => (),
                    };
                    q_dot = iter_q.next().unwrap();
                }
            }
        }
        if cvx_q.is_point_inside(&cvx_p.apexes[0]) {
            (true, cvx_p.clone())
        } else if cvx_p.is_point_inside(&cvx_q.apexes[0]) {
            (true, cvx_q.clone())
        } else {
            (
                false,
                ConvexHull {
                    id: 0,
                    apexes: cvx_inter,
                },
            )
        }
    }
}

pub struct ConvexHullIterator<'a> {
    pub convex_hull: &'a ConvexHull,
    pub idx: i32,
    pub direction: Direction,
}

impl<'a> ConvexHullIterator<'a> {
    pub fn new(convex_hull: &'a ConvexHull, direction: Direction) -> Self {
        ConvexHullIterator {
            convex_hull,
            idx: 0,
            direction,
        }
    }
}

impl<'a> Iterator for ConvexHullIterator<'a> {
    type Item = Edge<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let cur_pt = self.convex_hull.get_point(self.idx);
        let prev_pt = self.convex_hull.get_point(self.idx - self.direction as i32);
        self.idx += self.direction as i32;
        Some(Edge(prev_pt, cur_pt))
    }
}
