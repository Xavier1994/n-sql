// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
mod common;

test_init!();

#[theory]
#[case("cast(a as text)",NSQL,"a::text")]
#[case("a::text",NSQL,"a::text")]
fn test(left: &str, database_type: DatabaseType, right: &str){
    test_expression(database_type, left, right);
}