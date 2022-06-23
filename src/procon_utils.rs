#![allow(non_snake_case, unused)]

use std::cmp::min;
use std::collections::HashMap;
use std::fmt;
use std::io::prelude::*;
use std::mem::swap;
use std::ops::{Index, IndexMut};
use std::time::Instant;

use itertools::{concat, Itertools};
use petgraph::visit::Time;
use proconio::{*, marker::*};
use rand::prelude::SliceRandom;
use rand::Rng;
use rand_pcg::Mcg128Xsl64;

/// chmin, chmax 関数
pub trait SetMinMax {
    fn chmin(&mut self, v: Self) -> bool;
    fn chmax(&mut self, v: Self) -> bool;
}

impl<T> SetMinMax for T where T: PartialOrd {
    fn chmin(&mut self, v: T) -> bool {
        *self > v && {
            *self = v;
            true
        }
    }
    fn chmax(&mut self, v: T) -> bool {
        *self < v && {
            *self = v;
            true
        }
    }
}

/* timer
------------------------ */
pub struct Timer {
    since: Instant,
    duration: f64,
}

impl Timer {
    fn new(duration: f64) -> Timer {
        Timer {
            since: Instant::now(),
            duration,
        }
    }
    fn t(&self) -> f64 {
        (Instant::now() - self.since).as_secs_f64() * (1.0 / self.duration)
    }

    /*
     * 経過時間取得(sec)
     * 実行経過時間測定用
     * 実行直後に1度コールする。2回目以降は1度目のコールからの経過時間を返す
     *
     */
    fn get_time() -> f64 {
        static mut STIME: f64 = -1.0;
        let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();
        let ms = t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9;
        unsafe {
            if STIME < 0.0 {
                STIME = ms;
            }
            ms - STIME
        }
    }
}

/// hash 関数
/// https://codeforces.com/blog/entry/95477
// fn ksm(a: usize, b: usize) {
pub fn ksm(a: usize) -> usize {
    let b = 114514usize;
    const MOD: usize = 998244353;
    let mut a = a;
    let mut b = b;

    let mut ans = 1usize;
    while b > 0 {
        if b & 1 == 1 {
            ans = a * ans % MOD;
        }
        a = a * a % MOD;
        b >>= 1;
    }
    ans
}
