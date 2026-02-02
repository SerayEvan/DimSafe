// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

use std::fmt::{Display, Debug};
use std::fmt;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};

use super::super::scope::*;
use super::super::error::collector::*;
use super::super::scope::output::*;

use super::ast_node::*;

#[derive(Debug, Clone, PartialEq)]
pub struct LocationIndex {
    pub index: usize,
}

#[derive(Clone, PartialEq)]
pub struct RangeIndex {
    pub start: LocationIndex,
    pub end: LocationIndex,
}

impl RangeIndex {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start: LocationIndex::from_index(start), end: LocationIndex::from_index(end) }
    }
    pub fn rev_pos(&self, block: usize, lines_index: &[usize]) -> RangeReverseLocation {
        RangeReverseLocation {
            start: self.start.rev_pos(block, lines_index),
            end: self.end.rev_pos(block, lines_index),
        }
    }
}

impl Debug for RangeIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RangeIndex{{ {}..{} }}", self.start.index, self.end.index)
    }
}

#[derive(Debug, Clone)]
pub struct ReverseLocation {
    block: usize,
    line: usize,
    column: usize,
}

impl PartialEq for ReverseLocation {
    fn eq(&self, other: &Self) -> bool {
        self.block == other.block && self.line == other.line && self.column == other.column
    }
}
impl Eq for ReverseLocation {}
impl PartialOrd for ReverseLocation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for ReverseLocation {
    fn cmp(&self, other: &Self) -> Ordering {
        self.block.cmp(&other.block).then(self.line.cmp(&other.line)).then(self.column.cmp(&other.column))
    }
}

impl LocationIndex {
    pub fn from_index(index: usize) -> Self {
        Self { index }
    }
    pub fn rev_pos(&self,block: usize, lines_pos: &[usize]) -> ReverseLocation {
        // find line by dichotomie
        let mut start = 0;
        let mut end = lines_pos.len();
        while start < end {
            let mid = (start + end) / 2;
            if lines_pos[mid] <= self.index {
                start = mid + 1;
            } else {
                end = mid;
            }
        }
        let line = start - 1;
        let column = self.index - lines_pos[line];
        ReverseLocation { block, line, column }
    }
}

impl Display for ReverseLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.block, self.line, self.column)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RangeReverseLocation {
    pub start: ReverseLocation,
    pub end: ReverseLocation,
}

impl Display for RangeReverseLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Spanned<U: AstNode> {
    pub loc_range: RangeIndex,
    pub value: U,
}

impl<U: AstNode> Spanned<U> {

    pub fn new(loc: RangeIndex, value: U) -> Self {
        Self { loc_range: loc, value }
    }

    pub fn map<V: AstNode, F>(self, f: F) -> Spanned<V>
    where F: FnOnce(U) -> V {
        Spanned {
            loc_range: self.loc_range,
            value: f(self.value),
        }
    }
}

impl<U: AstNode> AstNode for Spanned<U> {

    type Output = U::Output;

    fn evaluate(&self, scope: &mut Scope, errors: &mut ErrorCollector, output: &mut OutputCollector) -> U::Output {
        let result = self.value.evaluate(scope, errors, output);
        errors.set_loc_range(&self.loc_range);
        result
    }

    #[cfg(test)]
    fn difference(prefix: &str, a: &Self, b: &Self) -> Vec<String> {
        let mut result = Vec::new();
        if a.loc_range != b.loc_range {
            result.push(format!("{}   - Location mismatch: {:?} != {:?}", prefix, a.loc_range, b.loc_range));
        }
        result.extend(U::difference(prefix, &a.value, &b.value));
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location_index() {

        // just one line
        let location_index = LocationIndex::from_index(0);
        let reverse_location = location_index.rev_pos(0, &vec![0]);
        assert_eq!(reverse_location, ReverseLocation { block: 0, line: 0, column: 0 });

        let location_index2 = LocationIndex::from_index(15);
        let reverse_location = location_index2.rev_pos(0, &vec![0]);
        assert_eq!(reverse_location, ReverseLocation { block: 0, line: 0, column: 15 });

        // many lines
        let location_index = LocationIndex::from_index(17);
        let reverse_location = location_index.rev_pos(0, &vec![0, 15, 20, 60, 100]);
        assert_eq!(reverse_location, ReverseLocation { block: 0, line: 1, column: 2 });
    }
}