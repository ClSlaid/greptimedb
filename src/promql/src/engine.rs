// Copyright 2022 Greptime Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::Arc;

use promql_parser::parser::Value;

use crate::error::Result;

mod evaluator;
mod functions;

pub use evaluator::*;

pub struct Context {}

pub struct Query {}

pub struct Engine {}

impl Engine {
    pub fn exec(_ctx: &Context, _q: Query) -> Result<Arc<dyn Value>> {
        unimplemented!();
    }
}