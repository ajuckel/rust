// xfail-test
// Reason: ICE with explicit self

// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

trait Positioned {
  fn SetX(&mut self, int);
  fn X(&self) -> int;
}

#[allow(default_methods)]
trait Movable: Positioned {
  fn translate(&mut self, dx: int) {
    let x = self.X();
    self.SetX(x + dx);
  }
}

struct Point { x: int, y: int }

impl Positioned for Point {
    fn SetX(&mut self, x: int) {
        self.x = x;
    }
    fn X(&self) -> int {
        self.x
    }
}

impl Movable for Point;

pub fn main() {
    let mut p = Point{ x: 1, y: 2};
    p.translate(3);
    assert!(p.X() == 4);
}
