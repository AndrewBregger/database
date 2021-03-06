// Copyright 2020 Alex Dukhno
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{
    plan::Plan,
    planner::{Planner, Result},
    SchemaId, SchemaName,
};
use data_manager::DataManager;
use protocol::{results::QueryError, Sender};
use sqlparser::ast::ObjectName;
use std::{convert::TryFrom, sync::Arc};

pub(crate) struct DropSchemaPlanner<'dsp> {
    names: &'dsp [ObjectName],
    cascade: bool,
}

impl DropSchemaPlanner<'_> {
    pub(crate) fn new(names: &[ObjectName], cascade: bool) -> DropSchemaPlanner<'_> {
        DropSchemaPlanner { names, cascade }
    }
}

impl Planner for DropSchemaPlanner<'_> {
    fn plan(self, data_manager: Arc<DataManager>, sender: Arc<dyn Sender>) -> Result<Plan> {
        let mut schemas = Vec::with_capacity(self.names.len());
        for name in self.names {
            match SchemaName::try_from(name) {
                Ok(schema_name) => match data_manager.schema_exists(&schema_name) {
                    None => {
                        sender
                            .send(Err(QueryError::schema_does_not_exist(schema_name)))
                            .expect("To Send Query Result to Client");
                        return Err(());
                    }
                    Some(schema_id) => schemas.push((SchemaId(schema_id), self.cascade)),
                },
                Err(error) => {
                    sender
                        .send(Err(QueryError::syntax_error(error)))
                        .expect("To Send Query Result to Client");
                    return Err(());
                }
            }
        }
        Ok(Plan::DropSchemas(schemas))
    }
}
