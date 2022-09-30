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

//! This module defines the most basic data types that are used throughout all
//! the code of our library (both at the abstraction and implementation levels).
//! These are also the types your client library is likely to work with.

// ----------------------------------------------------------------------------
// --- VARIABLE ---------------------------------------------------------------
// ----------------------------------------------------------------------------
/// This type denotes a variable from the optimization problem at hand.
/// In this case, each variable is assumed to be identified with an integer
/// ranging from 0 until `problem.nb_vars()`
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Variable(pub usize);
impl Variable {
    #[inline]
    /// This function retruns the id (numeric value) of the variable.
    ///
    /// # Examples:
    /// ```
    /// # use ddo::Variable;
    /// assert_eq!(0, Variable(0).id());
    /// assert_eq!(1, Variable(1).id());
    /// assert_eq!(2, Variable(2).id());
    /// assert_eq!(3, Variable(3).id());
    /// ```
    pub fn id(self) -> usize {
        self.0
    }
}

// ----------------------------------------------------------------------------
// --- DECISION ---------------------------------------------------------------
// ----------------------------------------------------------------------------
/// This denotes a decision that was made during the search. It affects a given
/// `value` to the specified `variable`. Any given `Decision` should be
/// understood as ```[[ variable = value ]]````
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Decision {
    pub variable : Variable,
    pub value    : isize
}

// ----------------------------------------------------------------------------
// --- Results ----------------------------------------------------------------
// ----------------------------------------------------------------------------
/// A reason explaining why the mdd stopped developing
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Reason {
    /// It stopped because the configured cutoff criterion was met
    CutoffOccurred
}

/// The outcome of an mdd development
#[derive(Debug, Clone)]
pub struct Completion {
    /// is the given solution exact (proved optimal for the given [sub-]problem)?
    /// or is it an approximation ?
    pub is_exact: bool,
    /// if present the value of the best solution derived from this mdd
    pub best_value: Option<isize>,
}


// ############################################################################
// #### TESTS #################################################################
// ############################################################################

#[cfg(test)]
mod test_var {
    use crate::Variable;

    #[test]
    fn test_var_id() {
        assert_eq!(0, Variable(0).id());
        assert_eq!(1, Variable(1).id());
        assert_eq!(2, Variable(2).id());
        assert_eq!(3, Variable(3).id());
    }
}