use core::f32;
use std::{arch::x86_64::*, time::Instant};

use rand::Rng;

unsafe fn distance_fast(x1: __m512, x2: __m512, y1: __m512, y2: __m512) -> __m512 {
    unsafe {
        let subs_1 = _mm512_sub_ps(x1, x2);
        let subs_2 = _mm512_sub_ps(y1, y2);
        let prods = _mm512_fmadd_ps(subs_1, subs_1, _mm512_mul_ps(subs_2, subs_2));
        _mm512_sqrt_ps(prods)
    }
}

unsafe fn tsp_fast(xs: &mut [f32], ys: &mut [f32]) -> (f32, Vec<usize>) {
    unsafe {
        let n = xs.len();
        let mut path = Vec::with_capacity(n);
        let mut total_dist = 0.0;

        let start_x = xs[0];
        let start_y = ys[0];

        let mut cur_x = start_x;
        let mut cur_y = start_y;
        let mut current_idx;

        path.push(0);

        xs[0] = f32::INFINITY;
        ys[0] = f32::INFINITY;

        let v_increment = _mm512_set1_ps(16.0);
        let v_indices_init = _mm512_set_ps(
            15.0, 14.0, 13.0, 12.0, 11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0, 0.0,
        );

        for _ in 0..n - 1 {
            let v_cx = _mm512_set1_ps(cur_x);
            let v_cy = _mm512_set1_ps(cur_y);

            let mut v_min_dist_sq = _mm512_set1_ps(f32::INFINITY);
            let mut v_best_idx = _mm512_setzero_ps(); // Placeholder
            let mut v_curr_idx_counter = v_indices_init;

            for i in (0..n).step_by(16) {
                let v_x = _mm512_loadu_ps(xs.as_ptr().add(i));
                let v_y = _mm512_loadu_ps(ys.as_ptr().add(i));

                let dist_sq = distance_fast(v_x, v_cx, v_y, v_cy);

                let mask = _mm512_cmplt_ps_mask(dist_sq, v_min_dist_sq);

                if mask != 0 {
                    v_min_dist_sq = _mm512_mask_blend_ps(mask, v_min_dist_sq, dist_sq);
                    v_best_idx = _mm512_mask_blend_ps(mask, v_best_idx, v_curr_idx_counter);
                }

                v_curr_idx_counter = _mm512_add_ps(v_curr_idx_counter, v_increment);
            }

            let mut best_dists_arr = [0.0; 16];
            let mut best_idxs_arr = [0.0; 16];

            _mm512_storeu_ps(best_dists_arr.as_mut_ptr(), v_min_dist_sq);
            _mm512_storeu_ps(best_idxs_arr.as_mut_ptr(), v_best_idx);

            let mut min_sq_val = f32::INFINITY;
            let mut next_idx = 0;

            for k in 0..16 {
                if best_dists_arr[k] < min_sq_val {
                    min_sq_val = best_dists_arr[k];
                    next_idx = best_idxs_arr[k] as usize;
                }
            }

            current_idx = next_idx;

            cur_x = xs[current_idx];
            cur_y = ys[current_idx];

            path.push(current_idx);
            total_dist += min_sq_val.sqrt();

            xs[current_idx] = f32::INFINITY;
            ys[current_idx] = f32::INFINITY;
        }

        let closing_dist = ((start_x - cur_x).powi(2) + (start_y - cur_y).powi(2)).sqrt();

        (total_dist + closing_dist, path)
    }
}
fn main() {
    const NUM_POINTS: usize = 16384;
    let mut x_coords = vec![0.0; NUM_POINTS];
    let mut y_coords = vec![0.0; NUM_POINTS];
    let mut rng = rand::rng();

    for i in 0..NUM_POINTS {
        x_coords[i] = rng.random_range(-500.0..=500.0);
        y_coords[i] = rng.random_range(-500.0..=500.0);
    }

    let start = Instant::now();
    unsafe {
        let (dist, _) = tsp_fast(&mut x_coords, &mut y_coords);
        println!("Elapsed: {:?}", start.elapsed());
        eprintln!("Final distance: {}", dist);
    }
}
