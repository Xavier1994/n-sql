// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ast::*;
use super::Visitor;
use std::result;
use std::fmt::{Write, Error, Result};
use optimizer::Optimizer;

type Formatter = String;

pub trait MySQLGenerator<T>{
    fn to_mysql(&self) -> result::Result<String, Error>;
}

struct InternalGenerator;

impl Visitor for InternalGenerator{
    fn visit_pagination_statement(&self, pagination_statement: &Box<PaginationStatement>, f: &mut Formatter) -> Result {
        self.visit_set_statement(&pagination_statement.set, f)?;

        f.write_char(' ')?;
        f.write_str("limit")?;
        f.write_char(' ')?;

        if let Some(ref limit) = pagination_statement.limit {
            self.visit_expression(limit, f)?;
        }

        if let Some(ref skip) = pagination_statement.skip {
            f.write_str(", ")?;
            self.visit_expression(skip, f)?;
        }
        Ok(())
    }

    fn visit_nvl_fn(&self, function: &Box<NvlFn>, f: &mut Formatter) -> Result {
        f.write_str("ifnull")?;
        f.write_char('(')?;
        self.visit_expression(&function.expr, f)?;
        f.write_str(", ")?;
        self.visit_expression(&function.default, f)?;
        f.write_char(')')
    }
}

impl MySQLGenerator<Expression> for Expression {
    fn to_mysql(&self) -> result::Result<String, Error> {
        let mut s = String::new();
        InternalGenerator.visit_expression(self, &mut s)?;
        Ok(s)
    }
}


impl MySQLGenerator<PredicateExpression> for PredicateExpression {
    fn to_mysql(&self) -> result::Result<String, Error> {
        let mut s = String::new();
        InternalGenerator.visit_predicate(self, &mut s)?;
        Ok(s)
    }
}

impl MySQLGenerator<Statement> for Statement {
    fn to_mysql(&self) -> result::Result<String, Error> {
        let mut s = String::new();
        InternalGenerator.visit_statement(self, &mut s)?;
        Ok(s)
    }
}