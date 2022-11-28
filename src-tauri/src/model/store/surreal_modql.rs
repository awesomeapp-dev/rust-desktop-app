use std::collections::BTreeMap;

use crate::prelude::*;
use modql::{BoolOpVal, FloatOpVal, IntOpVal, ListOptions, OpVal, OrGroups, StringOpVal};
use surrealdb::sql::Value;

pub(super) fn build_select_query(
	tb: &str,
	or_groups: Option<OrGroups>,
	list_options: ListOptions,
) -> Result<(String, BTreeMap<String, Value>)> {
	let mut sql = String::from("SELECT * FROM type::table($tb)");

	let mut vars = BTreeMap::from([("tb".into(), tb.into())]);

	// --- Apply the filter
	if let Some(or_groups) = or_groups {
		let mut idx = 0;
		sql.push_str(" WHERE");

		// For each OR group
		for (group_idx, filter_nodes) in or_groups.groups().into_iter().enumerate() {
			if group_idx > 0 {
				sql.push_str(" OR");
			}

			// The AND filters
			sql.push_str(" (");
			for (node_idx, filter_node) in filter_nodes.into_iter().enumerate() {
				let key = filter_node.name;
				for opval in filter_node.opvals {
					let var = f!("w{idx}");
					if node_idx > 0 {
						sql.push_str(" AND");
					}
					// fix me, needs to take it from op_val
					let (sql_el, val) = sqlize(opval, &key, &var)?;
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

	// TODO: Apply the offset

	// --- Apply the limit
	if let Some(limit) = list_options.limit {
		sql.push_str(&f!(" LIMIT {limit}"));
	}

	Ok((sql, vars))
}

/// Private helper to sqlize a a OpVal for SurrealDB
fn sqlize(opval: OpVal, prop_name: &str, var_idx: &str) -> Result<(String, Value)> {
	Ok(match opval {
		// Eq
		OpVal::String(StringOpVal::Eq(v)) => (f!("{prop_name} = ${var_idx}"), v.into()),
		OpVal::Int(IntOpVal::Eq(v)) => (f!("{prop_name} = ${var_idx}"), v.into()),
		OpVal::Float(FloatOpVal::Eq(v)) => (f!("{prop_name} = ${var_idx}"), v.into()),
		OpVal::Bool(BoolOpVal::Eq(v)) => (f!("{prop_name} = ${var_idx}"), v.into()),
		// Not
		OpVal::String(StringOpVal::Not(v)) => (f!("{prop_name} != ${var_idx}"), v.into()),
		OpVal::Int(IntOpVal::Not(v)) => (f!("{prop_name} != ${var_idx}"), v.into()),
		OpVal::Float(FloatOpVal::Not(v)) => (f!("{prop_name} != ${var_idx}"), v.into()),
		OpVal::Bool(BoolOpVal::Not(v)) => (f!("{prop_name} != ${var_idx}"), v.into()),
		// <
		OpVal::String(StringOpVal::Lt(v)) => (f!("{prop_name} < ${var_idx}"), v.into()),
		OpVal::Int(IntOpVal::Lt(v)) => (f!("{prop_name} < ${var_idx}"), v.into()),
		OpVal::Float(FloatOpVal::Lt(v)) => (f!("{prop_name} < ${var_idx}"), v.into()),
		// <=
		OpVal::String(StringOpVal::Lte(v)) => (f!("{prop_name} < ${var_idx}"), v.into()),
		OpVal::Int(IntOpVal::Lte(v)) => (f!("{prop_name} < ${var_idx}"), v.into()),
		OpVal::Float(FloatOpVal::Lte(v)) => (f!("{prop_name} < ${var_idx}"), v.into()),
		// >
		OpVal::String(StringOpVal::Gt(v)) => (f!("{prop_name} > ${var_idx}"), v.into()),
		OpVal::Int(IntOpVal::Gt(v)) => (f!("{prop_name} > ${var_idx}"), v.into()),
		OpVal::Float(FloatOpVal::Gt(v)) => (f!("{prop_name} > ${var_idx}"), v.into()),
		// >=
		OpVal::String(StringOpVal::Gte(v)) => (f!("{prop_name} > ${var_idx}"), v.into()),
		OpVal::Int(IntOpVal::Gte(v)) => (f!("{prop_name} > ${var_idx}"), v.into()),
		OpVal::Float(FloatOpVal::Gte(v)) => (f!("{prop_name} > ${var_idx}"), v.into()),

		// contains
		OpVal::String(StringOpVal::Contains(v)) => {
			(f!("{prop_name} CONTAINS ${var_idx}"), v.into())
		}

		// startsWith
		OpVal::String(StringOpVal::StartsWith(v)) => {
			(f!("string::startsWith({prop_name}, ${var_idx}) "), v.into())
		}

		// endsWith
		OpVal::String(StringOpVal::EndsWith(v)) => {
			(f!("string::endsWith({prop_name}, ${var_idx}) "), v.into())
		}

		_ => return Err(Error::ModqlOperatorNotSupported(f!("{opval:?}"))),
	})
}
