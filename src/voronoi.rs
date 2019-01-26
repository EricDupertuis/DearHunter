extern crate rand;
use rand::Rng;

use amethyst::core::nalgebra::{distance, Point2};

pub struct ClearRegion {
    pub center: Point2<f32>,
    pub radius: f32,
}

pub fn generate_voronoi(
    tree_count: usize,
    centroid_count: usize,
    path_width: f32,
    clear_regions: Vec<ClearRegion>,
) -> Vec<Point2<f32>> {
    let mut rng = rand::thread_rng();

    let mut centroids = Vec::with_capacity(centroid_count);
    while centroids.len() < centroid_count {
        let x = rng.gen::<f32>();
        let y = rng.gen::<f32>();
        centroids.push(Point2::new(x, y));
    }

    let mut trees = Vec::with_capacity(tree_count);
    // TODO: Maybe sample the trees on a grid instead ?
    while trees.len() < tree_count {
        let x = rng.gen::<f32>();
        let y = rng.gen::<f32>();

        let point = Point2::new(x, y);

        let mut distances: Vec<f32> = centroids
            .iter()
            .map(|p| distance(&point, &p) as f32)
            .collect::<Vec<f32>>();
        distances.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // If the point is on a voronoi line, ignore it
        if (distances[0] - distances[1]).abs() < path_width {
            continue;
        }

        // If the point is in the clear regions, discard it
        if clear_regions
            .iter()
            .map(|region| distance(&point, &region.center) < region.radius)
            .fold(0, |acc, x| acc + x as i32)
            > 0
        {
            continue;
        }

        trees.push(Point2::new(x, y));
    }

    trees
}
