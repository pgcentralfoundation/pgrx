use crate::{
    aggregate::{FinalizeModify, ParallelOption},
    datum::sql_entity_graph::{SqlGraphEntity, SqlGraphIdentifier, ToSql},
};
use core::{any::TypeId, cmp::Ordering};
use eyre::eyre as eyre_err;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct AggregateType {
    pub ty_source: &'static str,
    pub ty_id: TypeId,
    pub full_path: &'static str,
    pub name: Option<&'static str>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct MaybeVariadicAggregateType {
    pub agg_ty: AggregateType,
    pub variadic: bool,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct PgAggregateEntity {
    pub full_path: &'static str,
    pub module_path: &'static str,
    pub file: &'static str,
    pub line: u32,
    pub ty_id: TypeId,

    pub name: &'static str,

    /// The `arg_data_type` list.
    ///
    /// Corresponds to `Args` in [`crate::aggregate::Aggregate`].
    pub args: Vec<MaybeVariadicAggregateType>,

    /// The `ORDER BY arg_data_type` list.
    ///
    /// Corresponds to `OrderBy` in [`crate::aggregate::Aggregate`].
    pub order_by: Option<Vec<AggregateType>>,

    /// The `STYPE` and `name` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// The implementor of an [`crate::aggregate::Aggregate`].
    pub stype: AggregateType,

    /// The `SFUNC` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `state` in [`crate::aggregate::Aggregate`].
    pub sfunc: &'static str,

    /// The `FINALFUNC` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `finalize` in [`crate::aggregate::Aggregate`].
    pub finalfunc: Option<&'static str>,

    /// The `FINALFUNC_MODIFY` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `FINALIZE_MODIFY` in [`crate::aggregate::Aggregate`].
    pub finalfunc_modify: Option<FinalizeModify>,

    /// The `COMBINEFUNC` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `combine` in [`crate::aggregate::Aggregate`].
    pub combinefunc: Option<&'static str>,

    /// The `SERIALFUNC` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `serial` in [`crate::aggregate::Aggregate`].
    pub serialfunc: Option<&'static str>,

    /// The `DESERIALFUNC` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `deserial` in [`crate::aggregate::Aggregate`].
    pub deserialfunc: Option<&'static str>,

    /// The `INITCOND` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `INITIAL_CONDITION` in [`crate::aggregate::Aggregate`].
    pub initcond: Option<&'static str>,

    /// The `MSFUNC` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `moving_state` in [`crate::aggregate::Aggregate`].
    pub msfunc: Option<&'static str>,

    /// The `MINVFUNC` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `moving_state_inverse` in [`crate::aggregate::Aggregate`].
    pub minvfunc: Option<&'static str>,

    /// The `MSTYPE` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `MovingState` in [`crate::aggregate::Aggregate`].
    pub mstype: Option<AggregateType>,

    // The `MSSPACE` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    //
    // TODO: Currently unused.
    // pub msspace: &'static str,
    /// The `MFINALFUNC` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `moving_state_finalize` in [`crate::aggregate::Aggregate`].
    pub mfinalfunc: Option<&'static str>,

    /// The `MFINALFUNC_MODIFY` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `MOVING_FINALIZE_MODIFY` in [`crate::aggregate::Aggregate`].
    pub mfinalfunc_modify: Option<FinalizeModify>,

    /// The `MINITCOND` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `MOVING_INITIAL_CONDITION` in [`crate::aggregate::Aggregate`].
    pub minitcond: Option<&'static str>,

    /// The `SORTOP` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `SORT_OPERATOR` in [`crate::aggregate::Aggregate`].
    pub sortop: Option<&'static str>,

    /// The `PARALLEL` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `PARALLEL` in [`crate::aggregate::Aggregate`].
    pub parallel: Option<ParallelOption>,

    /// The `HYPOTHETICAL` parameter for [`CREATE AGGREGATE`](https://www.postgresql.org/docs/current/sql-createaggregate.html)
    ///
    /// Corresponds to `hypothetical` in [`crate::aggregate::Aggregate`].
    pub hypothetical: bool,
}

impl Ord for PgAggregateEntity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.file
            .cmp(other.full_path)
            .then_with(|| self.file.cmp(other.full_path))
    }
}

impl PartialOrd for PgAggregateEntity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Into<SqlGraphEntity> for PgAggregateEntity {
    fn into(self) -> SqlGraphEntity {
        SqlGraphEntity::Aggregate(self)
    }
}

impl SqlGraphIdentifier for PgAggregateEntity {
    fn dot_identifier(&self) -> String {
        format!("aggregate {}", self.full_path)
    }
    fn rust_identifier(&self) -> String {
        self.full_path.to_string()
    }
    fn file(&self) -> Option<&'static str> {
        Some(self.file)
    }
    fn line(&self) -> Option<u32> {
        Some(self.line)
    }
}

impl ToSql for PgAggregateEntity {
    #[tracing::instrument(level = "debug", err, skip(self, context), fields(identifier = %self.rust_identifier()))]
    fn to_sql(&self, context: &super::PgxSql) -> eyre::Result<String> {
        let self_index = context.aggregates[self];
        let mut optional_attributes = Vec::new();
        let schema = context.schema_prefix_for(&self_index);

        if let Some(value) = self.finalfunc {
            optional_attributes.push((
                format!("\tFINALFUNC = {}\"{}\"", schema, value),
                format!("/* {}::final */", self.full_path),
            ));
        }
        if let Some(value) = self.finalfunc_modify {
            optional_attributes.push((
                format!("\tFINALFUNC_MODIFY = {}", value.to_sql(context)?),
                format!("/* {}::FINALIZE_MODIFY */", self.full_path),
            ));
        }
        if let Some(value) = self.combinefunc {
            optional_attributes.push((
                format!("\tCOMBINEFUNC = {}\"{}\"", schema, value),
                format!("/* {}::combine */", self.full_path),
            ));
        }
        if let Some(value) = self.serialfunc {
            optional_attributes.push((
                format!("\tSERIALFUNC = {}\"{}\"", schema, value),
                format!("/* {}::serial */", self.full_path),
            ));
        }
        if let Some(value) = self.deserialfunc {
            optional_attributes.push((
                format!("\tDESERIALFUNC ={} \"{}\"", schema, value),
                format!("/* {}::deserial */", self.full_path),
            ));
        }
        if let Some(value) = self.initcond {
            optional_attributes.push((
                format!("\tINITCOND = '{}'", value),
                format!("/* {}::INITIAL_CONDITION */", self.full_path),
            ));
        }
        if let Some(value) = self.msfunc {
            optional_attributes.push((
                format!("\tMSFUNC = {}\"{}\"", schema, value),
                format!("/* {}::moving_state */", self.full_path),
            ));
        }
        if let Some(value) = self.minvfunc {
            optional_attributes.push((
                format!("\tMINVFUNC = {}\"{}\"", schema, value),
                format!("/* {}::moving_state_inverse */", self.full_path),
            ));
        }
        if let Some(value) = self.mfinalfunc {
            optional_attributes.push((
                format!("\tMFINALFUNC = {}\"{}\"", schema, value),
                format!("/* {}::moving_state_finalize */", self.full_path),
            ));
        }
        if let Some(value) = self.mfinalfunc_modify {
            optional_attributes.push((
                format!("\tMFINALFUNC_MODIFY = {}", value.to_sql(context)?),
                format!("/* {}::MOVING_FINALIZE_MODIFY */", self.full_path),
            ));
        }
        if let Some(value) = self.minitcond {
            optional_attributes.push((
                format!("\tMINITCOND = '{}'", value),
                format!("/* {}::MOVING_INITIAL_CONDITION */", self.full_path),
            ));
        }
        if let Some(value) = self.sortop {
            optional_attributes.push((
                format!("\tSORTOP = \"{}\"", value),
                format!("/* {}::SORT_OPERATOR */", self.full_path),
            ));
        }
        if let Some(value) = self.parallel {
            optional_attributes.push((
                format!("\tPARALLEL = {}", value.to_sql(context)?),
                format!("/* {}::PARALLEL */", self.full_path),
            ));
        }
        if self.hypothetical {
            optional_attributes.push((
                String::from("\tHYPOTHETICAL"),
                format!("/* {}::hypothetical */", self.full_path),
            ))
        }

        let stype_sql = context
            .rust_to_sql(self.stype.ty_id, self.stype.ty_source, self.stype.full_path)
            .ok_or_else(|| {
                eyre_err!(
                    "Failed to map moving state type `{}` to SQL type while building aggregate `{}`.",
                    self.stype.full_path,
                    self.name
                )
            })?;

        if let Some(value) = &self.mstype {
            let sql = context
                .rust_to_sql(value.ty_id, value.ty_source, value.full_path)
                .ok_or_else(|| {
                    eyre_err!(
                        "Failed to map moving state type `{}` to SQL type while building aggregate `{}`.",
                        value.full_path,
                        self.name
                    )
                })?;
            optional_attributes.push((
                format!("\tMSTYPE = {}", sql),
                format!("/* {}::MovingState = {} */", self.full_path, value.full_path),
            ));
        }

        let mut optional_attributes_string = String::new();
        for (index, (optional_attribute, comment)) in optional_attributes.iter().enumerate() {
            let optional_attribute_string = format!("{optional_attribute}{maybe_comma} {comment}{maybe_newline}",
                optional_attribute = optional_attribute,
                maybe_comma = if index == optional_attributes.len() -1 {
                    ""
                } else { "," },
                comment = comment,
                maybe_newline = if index == optional_attributes.len() -1 {
                    ""
                } else { "\n" }
            );
            optional_attributes_string += &optional_attribute_string;
        }

        let sql = format!(
            "\n\
                -- {file}:{line}\n\
                -- {full_path}\n\
                CREATE AGGREGATE {schema}{name} ({args}{maybe_order_by})\n\
                (\n\
                    \tSFUNC = {schema}\"{sfunc}\", /* {full_path}::state */\n\
                    \tSTYPE = {schema}{stype}{maybe_comma_after_stype} /* {stype_full_path} */\
                    {optional_attributes}\
                );\
            ",
            schema = schema,
            name = self.name,
            full_path = self.full_path,
            file = self.file,
            line = self.line,
            sfunc = self.sfunc,
            stype = stype_sql,
            stype_full_path = self.stype.full_path,
            maybe_comma_after_stype = if optional_attributes.len() == 0 {
                ""
            } else {
                ","
            },
            args = {
                let mut args = Vec::new();
                for (idx, arg) in self.args.iter().enumerate() {
                    let graph_index = context
                        .graph
                        .neighbors_undirected(self_index)
                        .find(|neighbor| match &context.graph[*neighbor] {
                            SqlGraphEntity::Type(ty) => ty.id_matches(&arg.agg_ty.ty_id),
                            SqlGraphEntity::Enum(en) => en.id_matches(&arg.agg_ty.ty_id),
                            SqlGraphEntity::BuiltinType(defined) => {
                                defined == &arg.agg_ty.full_path
                            }
                            _ => false,
                        })
                        .ok_or_else(|| {
                            eyre_err!("Could not find arg type in graph. Got: {:?}", arg.agg_ty)
                        })?;
                    let needs_comma = idx < (self.args.len() - 1);
                    let buf = format!("\
                           \t{name}{variadic}{schema_prefix}{sql_type}{maybe_comma}/* {full_path} */\
                       ",
                           schema_prefix = context.schema_prefix_for(&graph_index),
                           // First try to match on [`TypeId`] since it's most reliable.
                           sql_type = context.rust_to_sql(arg.agg_ty.ty_id, arg.agg_ty.ty_source, arg.agg_ty.full_path).ok_or_else(|| eyre_err!(
                               "Failed to map argument type `{}` to SQL type while building aggregate `{}`.",
                               arg.agg_ty.full_path,
                               self.name
                           ))?,
                           variadic = if arg.variadic { "VARIADIC " } else { "" },
                           maybe_comma = if needs_comma { ", " } else { " " },
                           full_path = arg.agg_ty.full_path,
                           name = if let Some(name) = arg.agg_ty.name {
                               format!(r#""{}" "#, name)
                           } else { "".to_string() },
                    );
                    args.push(buf);
                }
                String::from("\n")
                    + &args.join("\n")
                    + if self.order_by.is_none() { "\n" } else { "" }
            },
            maybe_order_by = if let Some(order_by) = &self.order_by {
                let mut args = Vec::new();
                for (idx, arg) in order_by.iter().enumerate() {
                    let graph_index = context
                        .graph
                        .neighbors_undirected(self_index)
                        .find(|neighbor| match &context.graph[*neighbor] {
                            SqlGraphEntity::Type(ty) => ty.id_matches(&arg.ty_id),
                            SqlGraphEntity::Enum(en) => en.id_matches(&arg.ty_id),
                            SqlGraphEntity::BuiltinType(defined) => defined == &arg.full_path,
                            _ => false,
                        })
                        .ok_or_else(|| {
                            eyre_err!("Could not find arg type in graph. Got: {:?}", arg)
                        })?;
                    let needs_comma = idx < (order_by.len() - 1);
                    let buf = format!("\
                           {schema_prefix}{sql_type}{maybe_comma}/* {full_path} */\
                       ",
                           schema_prefix = context.schema_prefix_for(&graph_index),
                           // First try to match on [`TypeId`] since it's most reliable.
                           sql_type = context.rust_to_sql(arg.ty_id, arg.ty_source, arg.full_path).ok_or_else(|| eyre_err!(
                               "Failed to map argument type `{}` to SQL type while building aggregate `{}`.",
                               arg.full_path,
                               self.name
                           ))?,
                           maybe_comma = if needs_comma { ", " } else { " " },
                           full_path = arg.full_path,
                    );
                    args.push(buf);
                }
                String::from("\n\tORDER BY ") + &args.join("\n,") + "\n"
            } else {
                String::default()
            },
            optional_attributes = if optional_attributes.len() == 0 {
                String::from("")
            } else {
                String::from("\n")
            } + &optional_attributes_string
                + if optional_attributes.len() == 0 {
                    ""
                } else {
                    "\n"
                },
        );
        tracing::debug!(%sql);
        Ok(sql)
    }
}
