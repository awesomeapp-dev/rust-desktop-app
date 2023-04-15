//! ModQL implementation for the surrealdb store.
//!
//! For now the following is implemented:
//!
//! - FilterNodes with FilterGroups
//! - ListOptions.offset
//! - ListOptions.limit
//! - ListOptions.order_by
//!
//! TODO: Implements the IncludeNodes when available in ModQL.
//!

use crate::prelude::*;
use crate::{Error, Result};
use modql::filter::{FilterGroups, OpVal, OpValBool, OpValFloat64, OpValInt64, OpValString};
use modql::ListOptions;
use std::collections::BTreeMap;
use surrealdb::sql::Value;

pub(super) fn build_select_query(
	tb: &str,
	or_groups: Option<FilterGroups>,
	list_options: ListOptions,
) -> Result<(String, BTreeMap<String, Value>)> {
	let mut sql = String::from("SELECT * FROM type::table($tb)");

	let mut vars = BTreeMap::from([("tb".into(), tb.into())]);

	// --- Apply the filter
	if let Some(or_groups) = or_groups {
		let mut idx = 0;
		sql.push_str(" WHERE");

		// For each OR group
		for (group_idx, filter_nodes) in or_groups.groups().iter().enumerate() {
			if group_idx > 0 {
				sql.push_str(" OR");
			}

			// The AND filters
			sql.push_str(" (");
			for (node_idx, filter_node) in filter_nodes.nodes().iter().enumerate() {
				let key = &filter_node.name;
				for opval in &filter_node.opvals {
					let var = f!("w{idx}");
					if node_idx > 0 {
						sql.push_str(" AND");
					}
					// fix me, needs to take it from op_val
					let (sql_el, val) = sqlize(opval.clone(), key, &var)?;
					sql.push_str(&f!(" {sql_el}"));
					vars.insert(var, val);

					idx += 1;
				}
			}
			sql.push_str(" )");
		}
	}

	// --- Apply the orderby
	if let Some(order_bys) = list_options.order_bys {
		sql.push_str(" ORDER BY ");
		let obs = order_bys
			.order_bys()
			.into_iter()
			.map(|o| o.to_string())
			.collect::<Vec<String>>();
		let obs = obs.join(",");
		sql.push_str(&obs);
	}

	// --- Apply the limit
	if let Some(limit) = list_options.limit {
		sql.push_str(&f!(" LIMIT {limit}"));
	}

	// --- Apply the offset
	if let Some(offset) = list_options.offset {
		sql.push_str(&f!(" START {offset}"));
	}

	Ok((sql, vars))
}

/// Private helper to sqlize a a OpVal for SurrealDB
///
/// #
fn sqlize(opval: OpVal, prop_name: &str, var_idx: &str) -> Result<(String, Value)> {
	Ok(match opval {
		// Eq
		OpVal::String(OpValString::Eq(v)) => (f!("{prop_name} = ${var_idx}"), v.into()),
		OpVal::Int64(OpValInt64::Eq(v)) => (f!("{prop_name} = ${var_idx}"), v.into()),
		OpVal::Float64(OpValFloat64::Eq(v)) => (f!("{prop_name} = ${var_idx}"), v.into()),
		OpVal::Bool(OpValBool::Eq(v)) => (f!("{prop_name} = ${var_idx}"), v.into()),
		// Not
		OpVal::String(OpValString::Not(v)) => (f!("{prop_name} != ${var_idx}"), v.into()),
		OpVal::Int64(OpValInt64::Not(v)) => (f!("{prop_name} != ${var_idx}"), v.into()),
		OpVal::Float64(OpValFloat64::Not(v)) => (f!("{prop_name} != ${var_idx}"), v.into()),
		OpVal::Bool(OpValBool::Not(v)) => (f!("{prop_name} != ${var_idx}"), v.into()),
		// <
		OpVal::String(OpValString::Lt(v)) => (f!("{prop_name} < ${var_idx}"), v.into()),
		OpVal::Int64(OpValInt64::Lt(v)) => (f!("{prop_name} < ${var_idx}"), v.into()),
		OpVal::Float64(OpValFloat64::Lt(v)) => (f!("{prop_name} < ${var_idx}"), v.into()),
		// <=
		OpVal::String(OpValString::Lte(v)) => (f!("{prop_name} < ${var_idx}"), v.into()),
		OpVal::Int64(OpValInt64::Lte(v)) => (f!("{prop_name} < ${var_idx}"), v.into()),
		OpVal::Float64(OpValFloat64::Lte(v)) => (f!("{prop_name} < ${var_idx}"), v.into()),
		// >
		OpVal::String(OpValString::Gt(v)) => (f!("{prop_name} > ${var_idx}"), v.into()),
		OpVal::Int64(OpValInt64::Gt(v)) => (f!("{prop_name} > ${var_idx}"), v.into()),
		OpVal::Float64(OpValFloat64::Gt(v)) => (f!("{prop_name} > ${var_idx}"), v.into()),
		// >=
		OpVal::String(OpValString::Gte(v)) => (f!("{prop_name} > ${var_idx}"), v.into()),
		OpVal::Int64(OpValInt64::Gte(v)) => (f!("{prop_name} > ${var_idx}"), v.into()),
		OpVal::Float64(OpValFloat64::Gte(v)) => (f!("{prop_name} > ${var_idx}"), v.into()),

		// contains
		OpVal::String(OpValString::Contains(v)) => {
			(f!("{prop_name} CONTAINS ${var_idx}"), v.into())
		}

		// startsWith
		OpVal::String(OpValString::StartsWith(v)) => {
			(f!("string::startsWith({prop_name}, ${var_idx}) "), v.into())
		}

		// endsWith
		OpVal::String(OpValString::EndsWith(v)) => {
			(f!("string::endsWith({prop_name}, ${var_idx}) "), v.into())
		}

		_ => return Err(Error::ModqlOperatorNotSupported(f!("{opval:?}"))),
	})
}
