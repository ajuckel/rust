// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

trait MyEq {
    fn eq(&self, other: &Self) -> bool;
}

struct A {
    x: int
}

impl MyEq for int {
    fn eq(&self, other: &int) -> bool { *self == *other }
}

impl MyEq for A;  //~ ERROR missing method

fn main() {
}

