use std::arch::x86_64::{__m512, _mm512_fmadd_ps, _mm512_mul_ps, _mm512_sqrt_ps, _mm512_sub_ps};

use rand::Rng;

fn distance(p1: &(f32, f32), p2: &(f32, f32)) -> f32 {
    (p1.0 - p2.0).powi(2) + (p1.1 - p2.1).powi(2)
}

fn distance_fast(x1: __m512, x2: __m512, y1: __m512, y2: __m512) -> __m512 {
    unsafe {
        let subs_1 = _mm512_sub_ps(x1, x2);
        let subs_2 = _mm512_sub_ps(y1, y2);
        let prods = _mm512_fmadd_ps(subs_1, subs_1, _mm512_mul_ps(subs_2, subs_2));
        _mm512_sqrt_ps(prods)
    }
}

fn tsp(points: &[(f32, f32)]) -> (f32, Vec<usize>) {
    let mut visited = vec![false; points.len()];
    let mut path = Vec::with_capacity(points.len() + 1);
    let mut total_dist = 0.0;
    let mut current_idx = 0;

    visited[0] = true;
    path.push(0);

    for _ in 0..points.len() - 1 {
        let mut min_sq_dist = f32::MAX;
        let mut next_idx = 0;

        for (i, point) in points.iter().enumerate() {
            if !visited[i] {
                let d = distance(&points[current_idx], point);
                if d < min_sq_dist {
                    min_sq_dist = d;
                    next_idx = i;
                }
            }
        }

        visited[next_idx] = true;
        path.push(next_idx);
        total_dist += min_sq_dist.sqrt();
        current_idx = next_idx;
    }

    total_dist += distance(&points[current_idx], &points[0]).sqrt();
    path.push(0);

    (total_dist, path)
}

fn main() {
    const NUM_POINTS: usize = 16394;
    let mut points = vec![(0.0, 0.0); NUM_POINTS];
    let mut rng = rand::rng();

    points.iter_mut().for_each(|p| {
        p.0 = rng.random_range(-500.0..=500.0);
        p.1 = rng.random_range(-500.0..=500.0);
    });

    let (dist, path) = tsp(&points);

    for idx in path {
        println!("{} {}", points[idx].0, points[idx].1);
    }
    eprintln!("Final distance: {}", dist);
}
