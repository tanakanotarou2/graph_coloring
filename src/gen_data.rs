#![allow(non_snake_case, unused)]

use std::cmp::min;
use std::collections::HashMap;
use rand::prelude::SliceRandom;
use proconio::{*, marker::*};

use std::fmt;
use rand_pcg::Mcg128Xsl64;

use std::io::prelude::*;
use std::mem::swap;
use std::time::Instant;
use std::ops::{Index, IndexMut};
use itertools::{concat, Itertools};
use petgraph::visit::Time;
use rand::Rng;


/// 座標を表す構造体
// #[derive_readable] // proconio で input できるようにする
#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
struct P(pub f64, pub f64);

impl P {
    fn dist(self, rhs: P) -> f64 {
        let y = self.0 - rhs.0;
        let x = self.1 - rhs.1;
        return (y * y + x * x).sqrt();
    }
}

const EPS: f64 = 1e-9;

/// グラフ作成
/// - n: 頂点数
/// - d: 距離が d 以下の 2 頂点間に辺を結ぶ
fn gen_random_data(seed: u128, n: usize, d: f64) -> (Vec<P>, Vec<(usize, usize)>) {
    let mut rng: rand_pcg::Pcg64Mcg = rand_pcg::Pcg64Mcg::new(seed);
    let pos = (0..n).map(|i|
        P(rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0))
    ).collect_vec();
    let mut edges = vec![];

    for i in 0..n {
        for j in i + 1..n {
            if pos[i].dist(pos[j]) < d + EPS { edges.push((i, j)); }
        }
    }
    return (pos, edges);
}

/// 各頂点の平均次数が Nπd^2 となることが知られている
fn calc_avg_jisu(n: usize, d: f64) -> f64 {
    const PI: f64 = std::f64::consts::PI;
    (n as f64) * PI * d * d
}

use average::Mean;
use average::{MeanWithError, Estimate};
fn main() {
    let n = 1000; // 頂点数
    const AVG_JISU: f64 = 10.0; // 平均次数

    let d;
    {
        let mut l = 0.0;
        let mut r = 1.0;
        for _ in 0..100 {
            let m = (l + r) / 2.0;
            let v = calc_avg_jisu(n, m);
            if v < AVG_JISU + EPS {
                l = m;
            } else {
                r = m;
            }
        }
        d = l;
    }

    let (pos, edges) = gen_random_data(128, n, d);
    println!("{} {}",n, edges.len());
    for e in edges.iter() {
        println!("{} {}", e.0, e.1);
    }

    let mut cnt = vec![0; n];
    edges.iter().for_each(|(u, v)| {
        cnt[*u] += 1;
        cnt[*v] += 1;
    });

    let avg_total: Mean = cnt.iter().map(|v| f64::from(*v)).collect();
    eprintln!("{:?}", avg_total);
}
