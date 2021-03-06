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

use std::convert::TryFrom;

use sql_model::Id;
use sqlparser::ast::ObjectName;
use std::fmt::{self, Display, Formatter};

use sql_model::sql_types::SqlType;

///! Module for representing how a query will be parameters bound, executed and
///! values represented during runtime.
pub mod plan;
pub mod planner;

/// represents a schema uniquely by its id
#[derive(PartialEq, Debug, Clone)]
pub struct SchemaId(Id);

impl AsRef<Id> for SchemaId {
    fn as_ref(&self) -> &Id {
        &self.0
    }
}

/// represents a schema uniquely by its name
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct SchemaName(String);

impl AsRef<str> for SchemaName {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl TryFrom<&ObjectName> for SchemaName {
    type Error = SchemaNamingError;

    fn try_from(object: &ObjectName) -> Result<Self, Self::Error> {
        if object.0.len() != 1 {
            Err(SchemaNamingError(object.to_string()))
        } else {
            Ok(SchemaName(object.to_string()))
        }
    }
}

impl Display for SchemaName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct SchemaNamingError(String);

impl Display for SchemaNamingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "only unqualified schema names are supported, '{}'", self.0)
    }
}

/// represents a table uniquely
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct FullTableName(SchemaName, String);

impl FullTableName {
    fn as_tuple(&self) -> (&str, &str) {
        (&self.0.as_ref(), &self.1)
    }
}

impl Display for FullTableName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.0.as_ref(), self.1)
    }
}

impl TryFrom<&ObjectName> for FullTableName {
    type Error = TableNamingError;

    fn try_from(object: &ObjectName) -> Result<Self, Self::Error> {
        if object.0.len() == 1 {
            Err(TableNamingError::Unqualified(object.to_string()))
        } else if object.0.len() != 2 {
            Err(TableNamingError::NotProcessed(object.to_string()))
        } else {
            let table_name = object.0.last().unwrap().value.clone();
            let schema_name = object.0.first().unwrap().value.clone();
            Ok(FullTableName(SchemaName(schema_name), table_name))
        }
    }
}

pub enum TableNamingError {
    Unqualified(String),
    NotProcessed(String),
}

impl Display for TableNamingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TableNamingError::Unqualified(table_name) => write!(
                f,
                "unsupported table name '{}'. All table names must be qualified",
                table_name
            ),
            TableNamingError::NotProcessed(table_name) => write!(f, "unable to process table name '{}'", table_name),
        }
    }
}

/// A type of a column
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColumnType {
    #[allow(dead_code)]
    nullable: bool,
    sql_type: SqlType,
}

/// represents a table uniquely
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct TableId((Id, Id));

impl AsRef<(Id, Id)> for TableId {
    fn as_ref(&self) -> &(Id, Id) {
        &self.0
    }
}

#[cfg(test)]
mod tests;
