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

fn validate(g: &Graph<(), (), Undirected>, colors: &Vec<usize>) -> bool {
    let n = g.node_count();
    let nodes = g.node_indices().collect_vec();
    for i in 1..n {
        let c = colors[i];
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

    let input = parse_input();
    let mut g = Graph::<(), (), Undirected>::new_undirected();
    let nodes: Vec<_> = (0..input.n).map(|_| g.add_node(())).collect();
    g.extend_with_edges(input.edges.iter().map(|&x| (nodes[x.0], nodes[x.1])));


    let res = solver_DSATUR(&g);
    // println!("{:?}", res);
    validate(&g, &res);
    println!("color count: {:?}", get_result(&res));

    let res = solver_SEQ(&g);
    // println!("{:?}", res);
    validate(&g, &res);
    println!("color count: {:?}", get_result(&res));
    println!("test {} {}", input.n, g.edge_count());
    //
    // assert_eq!((input.n, input.edges.len()), (g.node_count(), g.edge_count()));

    // let n = g.node_count();
    // let nodes =g.node_indices().collect_vec();
    // for v in g.neighbors(nodes[0]){
    //     println!("n, {:?}",v.index());
    // }
    // let a=ksm(10);
    // let b=ksm(1002);
    // let c = a^b;
    // println!("{} {}",a,b);
    // println!("{} {} {}",c^a,c^b,c^a^b);
}
