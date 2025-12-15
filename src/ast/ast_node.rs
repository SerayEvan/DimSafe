// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

use std::ops::Deref;
use std::fmt::Debug;

use crate::error::*;
use crate::scope::*;

pub trait AstNode {
    type Output: Clone;
    fn rev_location(&mut self, block: usize, lines_index: &[usize]);
    fn evaluate(&self, scope: &mut Scope) -> Result<Self::Output, Error>;

    #[cfg(test)]
    fn difference(prefix: &str, a: &Self, b: &Self) -> Vec<String>;
}

impl<U: AstNode> AstNode for Vec<U> {
    type Output = Vec<U::Output>;
    fn rev_location(&mut self, block: usize, lines_index: &[usize]) {
        for item in self {
            item.rev_location(block, lines_index);
        }
    }
    fn evaluate(&self, scope: &mut Scope) -> Result<Vec<U::Output>, Error> {
        let mut result = Vec::new();
        for item in self {
            result.push(item.evaluate(scope)?);
        }
        Ok(result)
    }

    #[cfg(test)]
    fn difference(prefix: &str, a: &Self, b: &Self) -> Vec<String> {
        if a.len() != b.len() {
            return vec![format!("{}   - Length mismatch: {} != {}", prefix, a.len(), b.len())];
        }
        let mut result = Vec::new();
        for (i, item) in a.iter().enumerate() {
            result.extend(U::difference(format!("{}[{}]", prefix, i).as_str(), item, &b[i]));
        }
        result
    }
}

impl<U: AstNode> AstNode for Option<U> {
    type Output = Option<U::Output>;
    
    fn rev_location(&mut self, block: usize, lines_index: &[usize]) {
        if let Some(value) = self {
            value.rev_location(block, lines_index);
        }
    }
    
    fn evaluate(&self, scope: &mut Scope) -> Result<Option<U::Output>, Error> {
        match self {
            Some(value) => Ok(Some(value.evaluate(scope)?)),
            None => Ok(None),
        }
    }

    #[cfg(test)]
    fn difference(prefix: &str, a: &Self, b: &Self) -> Vec<String> {
        match (a, b) {
            (Some(a), Some(b)) => U::difference(prefix, a, b),
            (Some(_), None) => vec![format!("{}   - Branch mismatch: Some != None", prefix)],
            (None, Some(_)) => vec![format!("{}   - Branch mismatch: None != Some", prefix)],
            (None, None) => vec![],
        }
    }
}

impl<U: AstNode, V: AstNode> AstNode for (U, V) {
    type Output = (U::Output, V::Output);
    
    fn rev_location(&mut self, block: usize, lines_index: &[usize]) {
        self.0.rev_location(block, lines_index);
        self.1.rev_location(block, lines_index);
    }
    
    fn evaluate(&self, scope: &mut Scope) -> Result<(U::Output, V::Output), Error> {
        Ok((self.0.evaluate(scope)?, self.1.evaluate(scope)?))
    }

    #[cfg(test)]
    fn difference(prefix: &str, a: &Self, b: &Self) -> Vec<String> {
        let mut result = Vec::new();
        result.extend(U::difference(format!("{}:0", prefix).as_str(), &a.0, &b.0));
        result.extend(V::difference(format!("{}:1", prefix).as_str(), &a.1, &b.1));
        result
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Leaf<T: Clone + PartialEq + Debug> {
    pub value: T,
}

impl<T: Clone + PartialEq + Debug> From<T> for Leaf<T> {
    fn from(value: T) -> Self {
        Self { value }
    }
}

impl<T: Clone + PartialEq + Debug> AstNode for Leaf<T> {
    type Output = T;
    
    fn rev_location(&mut self, _block: usize, _lines_index: &[usize]) {}
    
    fn evaluate(&self, _scope: &mut Scope) -> Result<T, Error> {
        Ok(self.value.clone())
    }
    
    #[cfg(test)]
    fn difference(prefix: &str, a: &Self, b: &Self) -> Vec<String> {
        if a.value != b.value {
            return vec![format!("{}   - Value mismatch: {:?} != {:?}", prefix, a.value, b.value)];
        }
        vec![]
    }
}

impl<T: Clone + PartialEq + Debug> Deref for Leaf<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl AstNode for () {
    type Output = ();

    fn rev_location(&mut self, _block: usize, _lines_index: &[usize]) {
        ()
    }

    fn evaluate(&self, _scope: &mut Scope) -> Result<(), Error> {
        Ok(())
    }
    
    #[cfg(test)]
    fn difference(_prefix: &str, _a: &Self, _b: &Self) -> Vec<String> {
        vec![]
    }
}