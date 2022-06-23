#![allow(non_snake_case, unused)]

use std::cmp::min;
use std::collections::{BinaryHeap, HashMap, HashSet};
use rand::prelude::SliceRandom;
use proconio::{*, marker::*};

use std::fmt;
use rand_pcg::Mcg128Xsl64;

use std::io::prelude::*;
use std::mem::swap;
use std::time::Instant;
use std::ops::{Index, IndexMut};
use std::process::id;
use itertools::{concat, Itertools};
use petgraph::{Graph, Undirected};
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::visit::{IntoNeighbors, Time};
use rand::Rng;

const EPS: f64 = 1e-9;


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


#[derive(Clone, Debug)]
pub struct Input {
    pub n: usize,
    // node count
    pub m: usize,
    // edge count
    pub edges: Vec<(usize, usize)>,
}

fn parse_input() -> Input {
    input! {
        n:usize,
        m:usize,
        edges:[(usize,usize);m]
    }
    Input { n, m, edges }
}


/// SEQ法 のソルバー
fn solver_SEQ(g: &Graph<(), (), Undirected>) -> Vec<usize> {
    let n = g.node_count();
    let nodes = g.node_indices().collect_vec();
    let mut colors = vec![!0usize; n];

    colors[0] = 1;
    for i in 1..n {
        let mut no = 1;
        let mut used: HashSet<usize> = HashSet::new();
        for v in g.neighbors(nodes[i]) {
            if colors[v.index()] != !0 {
                used.insert(colors[v.index()]);
            }
        }
        for j in 0..used.len() + 1 {
            if !used.contains(&j) {
                colors[i] = j;
                break;
            }
        }
    }
    colors
}


/// DSATUR による解法
fn solver_DSATUR(g: &Graph<(), (), Undirected>) -> Vec<usize> {
    let n = g.node_count();
    let nodes = g.node_indices().collect_vec();
    let edge_cnt = (0..n).map(|i| { g.edges(nodes[i]).count() }).collect_vec();
    let mut colors = vec![!0usize; n];

    // 飽和次数の管理用
    let mut neighbor_colors: Vec<HashSet<usize>> = vec![HashSet::new(); n];
    // // 飽和次数
    // let mut saturation = vec![!0usize; n];

    let mut que = BinaryHeap::new();
    for i in 0..n { que.push((0, 0, i)); }

    // 飽和次数を更新
    let mut update_saturation = |neighbor_colors: &mut Vec<HashSet<usize>>,
                                 que: &mut BinaryHeap<(usize, usize, usize)>,
                                 colors: &Vec<usize>,
                                 u: usize, c: usize| {
        for v in g.neighbors(nodes[u]).filter(|&i| colors[i.index()] == !0) {
            let vi = v.index();
            if !neighbor_colors[vi].contains(&c) {
                neighbor_colors[vi].insert(c);
                // add priority que
                let colored = neighbor_colors[vi].len();
                let uncolored = edge_cnt[vi] - colored;
                que.push((colored, uncolored, v.index()));
            }
        }
    };
    colors[0] = 0;
    update_saturation(&mut neighbor_colors, &mut que, &colors, 0, 0);
    while let Some((_, _, idx)) = que.pop() {
        if colors[idx] != !0 { continue; }
        for i in 0..neighbor_colors[idx].len() + 1 {
            if !neighbor_colors[idx].contains(&i) {
                colors[idx] = i;
                update_saturation(&mut neighbor_colors, &mut que, &colors, idx, i);
                break;
            }
        }
        neighbor_colors[idx].clear();
    }
    colors
}


/// RLF の解法
fn solver_RLF(g: &Graph<(), (), Undirected>) -> Vec<usize> {
    let n = g.node_count();
    let nodes = g.node_indices().collect_vec();
    let edge_cnt = (0..n).map(|i| { g.edges(nodes[i]).count() }).collect_vec();
    let mut colors = vec![!0usize; n];

    let mut no: i32 = 0;

    // 集合 U の管理
    let mut u_nodes = vec![-1; n];
    loop {
        // 色 no で塗れる頂点集合
        let mut v_nodes = vec![];

        // 未彩色頂点の取得
        for i in 0..n {
            if colors[i] == !0 {
                let cnt = g.neighbors(nodes[i]).filter(|v| colors[v.index()] == !0).count();
                v_nodes.push((i, cnt));
            }
        }
        if v_nodes.len() == 0 { break; }

        v_nodes.sort_by_key(|(_, cnt)| -(*cnt as i32));

        // 各頂点ごとの U に登録されている隣接頂点数
        let mut nu_cnt = vec![0; n];

        let mut v = v_nodes[0].0;
        loop {
            colors[v] = no as usize;

            // v の隣接頂点を 集合 U に登録
            let tmp = g.neighbors(nodes[v]).filter(|v| colors[v.index()] == !0 && u_nodes[v.index()] < no).collect_vec();
            for u in tmp {
                u_nodes[u.index()] = no;
                for u2 in g.neighbors(u).filter(|v| colors[v.index()] == !0) {
                    nu_cnt[u2.index()] += 1;
                }
            }

            let mut nxt_nodes = vec![];
            for (idx, cnt) in v_nodes.iter() {
                if colors[*idx] == !0 && u_nodes[*idx] < no {
                    nxt_nodes.push((*idx, *cnt));
                }
            }
            if nxt_nodes.len() == 0 {
                break;
            }

            // 集合 U に含まれる隣接頂点が多いものを次の処理対象に
            let mut nxt_v = !0usize;
            let mut max_nucnt = -1;
            for &(idx, cnt) in nxt_nodes.iter() {
                if max_nucnt.chmax(nu_cnt[idx] as i32) {
                    nxt_v = idx;
                }
            }
            v = nxt_v;
            v_nodes = nxt_nodes;
        }
        no += 1;
    }
    colors
}

fn validate(g: &Graph<(), (), Undirected>, colors: &Vec<usize>) -> bool {
    let n = g.node_count();
    let nodes = g.node_indices().collect_vec();
    for i in 1..n {
        let c = colors[i];
        if c == !0 {
            eprintln!("no color is assigned! node:{}", i);
            return false;
        }
        for v in g.neighbors(nodes[i]) {
            if colors[v.index()] == c {
                eprintln!("nodes {} - {} are same color({})", i, v.index(), c);
                return false;
            }
        }
    }
    true
}

fn get_result(colors: &Vec<usize>) -> usize {
    let mut used: HashSet<usize> = HashSet::new();
    for i in 1..colors.len() {
        let c = colors[i];
        used.insert(c);
    }
    used.len()
}

fn main() {
    // read input, build graph
    let input = parse_input();
    let mut g = Graph::<(), (), Undirected>::new_undirected();
    let nodes: Vec<_> = (0..input.n).map(|_| g.add_node(())).collect();
    g.extend_with_edges(input.edges.iter().map(|&x| (nodes[x.0], nodes[x.1])));
    println!("total node:{} total edge:{}", input.n, g.edge_count());

    let mut t = Timer::get_time();
    let res = solver_SEQ(&g);
    validate(&g, &res);
    println!("SEQ color count: {:?}", get_result(&res));
    println!("SEQ time: {:?}", Timer::get_time() - t);
    println!();

    let mut t = Timer::get_time();
    let res = solver_DSATUR(&g);
    validate(&g, &res);
    println!("DSATUR color count: {:?}", get_result(&res));
    println!("DSATUR time: {:?}", Timer::get_time() - t);
    println!();

    let mut t = Timer::get_time();
    let res = solver_RLF(&g);
    validate(&g, &res);
    println!("RLF color count: {:?}", get_result(&res));
    println!("RLF time: {:?}", Timer::get_time() - t);
    println!();
}
