// Copyright 2020 Xavier Gillard
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

//! This module defines traits for implementations of an MDD.
use crate::core::common::{Decision, Node, NodeInfo};

/// This enumeration characterizes the kind of MDD being generated. It can
/// either be
/// * `Exact` if it is a true account of the problem state space.
/// * `Restricted` if it is an under approximation of the problem state space.
/// * `Relaxed` if it is an over approximation of the problem state space.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MDDType {
    Relaxed,
    Restricted,
    Exact
}

/// This trait describes an MDD
///
/// # Type param
/// The type parameter `<T>` denotes the type of the state defined/manipulated
/// by the `Problem` definition.
pub trait MDD<T> {
    /// Tells whether this MDD is exact, relaxed, or restricted.
    fn mdd_type(&self) -> MDDType;

    /// Generates the root node of the problem
    fn root(&self) -> Node<T>;

    /// Expands this MDD into  an exact MDD
    fn exact(&mut self, root: &Node<T>, best_lb : i32);
    /// Expands this MDD into a restricted (lower bound approximation)
    /// version of the exact MDD.
    fn restricted(&mut self, root: &Node<T>, best_lb : i32);
    /// Expands this MDD into a relaxed (upper bound approximation)
    /// version of the exact MDD.
    fn relaxed(&mut self, root: &Node<T>, best_lb : i32);

    /// Consumes (removes) all nodes from the cutset of this mdd ands applies
    /// the given function `f` to each pair of `(state, node_info)` present in
    /// this mdd.
    ///
    /// # Note:
    /// Because the nodes are consumed, they are no longer available for use
    /// after a call to this method completes.
    ///
    /// All nodes from the cutset are considered to be used even though the
    /// function may decide to skip them. Hence, calling `for_each_cutset_node`
    /// after a call to this method completes will have absolutely no effect.
    fn consume_cutset<F>(&mut self, f: F) where F: FnMut(T, NodeInfo);

    /// Return true iff this `MDD` is exact. That is to say, it returns true if
    /// no nodes have been merged (because of relaxation) or suppressed (because
    /// of restriction).
    fn is_exact(&self) -> bool;
    /// Returns the length of the longest path between the root and the terminal
    /// node of this `MDD`.
    fn best_value(&self) -> i32;
    /// Returns the terminal node having the longest associated path in this `MDD`.
    fn best_node(&self) -> &Option<NodeInfo>;
    /// Returns the list of decisions along the longest path between the
    /// root node and the best terminal node of this `MDD`.
    fn longest_path(&self) -> Vec<Decision>;
}
