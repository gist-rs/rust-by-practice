use std::fmt::Display;

fn foobar_1(thing: &dyn Display) {}

fn foobar_2(thing: Box<dyn Display>) {}

fn main() {}
