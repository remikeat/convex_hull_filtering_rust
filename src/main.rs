extern crate convex_hull_filtering;
use convex_hull_filtering::convex_hull::ConvexHull;
use convex_hull_filtering::ConvexHulls;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filepath: String = args[1].parse().unwrap();
    let file = std::fs::File::open(filepath).unwrap();
    let convex_hulls: ConvexHulls = serde_json::from_reader(&file).unwrap();
    //println!("{:?}", convex_hulls.convex_hulls);
    let res =
        ConvexHull::intersection(&convex_hulls.convex_hulls[7], &convex_hulls.convex_hulls[8]);
    println!("{:?}", res);
}
