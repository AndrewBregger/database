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

use std::sync::Arc;

use data_manager::DataManager;
use kernel::SystemResult;
use protocol::{results::QueryEvent, Sender};
use query_planner::plan::SchemaCreationInfo;

pub(crate) struct CreateSchemaCommand {
    schema_info: SchemaCreationInfo,
    data_manager: Arc<DataManager>,
    sender: Arc<dyn Sender>,
}

impl CreateSchemaCommand {
    pub(crate) fn new(
        schema_info: SchemaCreationInfo,
        data_manager: Arc<DataManager>,
        sender: Arc<dyn Sender>,
    ) -> CreateSchemaCommand {
        CreateSchemaCommand {
            schema_info,
            data_manager,
            sender,
        }
    }

    pub(crate) fn execute(&mut self) -> SystemResult<()> {
        let schema_name = &self.schema_info.schema_name;
        match self.data_manager.create_schema(schema_name) {
            Err(error) => Err(error),
            Ok(_schema_id) => {
                self.sender
                    .send(Ok(QueryEvent::SchemaCreated))
                    .expect("To Send Query Result to Client");
                Ok(())
            }
        }
    }
}
