extern crate convex_hull_filtering;
use convex_hull_filtering::convex_hull::ConvexHull;
use convex_hull_filtering::ConvexHulls;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filepath: String = args[1].parse().unwrap();
    let file = std::fs::File::open(filepath).unwrap();
    let convex_hulls: ConvexHulls = serde_json::from_reader(&file).unwrap();
    //println!("{:?}", convex_hulls.convex_hulls);
    let convex_hulls = convex_hulls.convex_hulls;
    for (index1, convex_hull1) in convex_hulls.iter().enumerate() {
        for convex_hull2 in convex_hulls.iter().skip(index1 + 1) {
            let res = ConvexHull::intersection(&convex_hull1, &convex_hull2);
            println!("{:?}", res);
        }
    }
}
