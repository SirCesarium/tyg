use serde_json::Value;

/// Two samples disagreed on the type of the same key path.
#[derive(Debug, PartialEq, Eq)]
pub struct TypeConflict {
    /// Dotted key path (`user.id`), empty at the document root.
    pub path: String,
    /// Type of the value that was replaced.
    pub existing: &'static str,
    /// Type of the incoming value (the one that won).
    pub incoming: &'static str,
}

/// Human-readable type name of a JSON value, for conflict messages.
fn type_name(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

/// Merges `incoming` into `values`, recording type conflicts along the way.
///
/// - Objects: deep-merged key by key, tracked with a dotted `path`.
/// - Arrays: concatenated.
/// - `null`: never replaces a known value (and a known value never becomes
///   `null` again).
/// - Scalars and mixed shapes: `incoming` wins, and a `TypeConflict` is
///   pushed so the caller can warn about the disagreement.
pub fn merge(values: &mut Value, incoming: Value, path: &str, conflicts: &mut Vec<TypeConflict>) {
    match (values, incoming) {
        (Value::Object(existing), Value::Object(map)) => {
            for (k, v) in map {
                let child = if path.is_empty() {
                    k.clone()
                } else {
                    format!("{path}.{k}")
                };

                if let Some(ev) = existing.get_mut(&k) {
                    merge(ev, v, &child, conflicts);
                } else {
                    existing.insert(k, v);
                }
            }
        }
        (Value::Array(existing), Value::Array(arr)) => {
            existing.extend(arr);
        }
        // `null` carries no type information, so it must not erase one.
        (existing, Value::Null) if !matches!(existing, Value::Null) => {}
        (existing, incoming) => {
            if !matches!(existing, Value::Null) && type_name(existing) != type_name(&incoming) {
                conflicts.push(TypeConflict {
                    path: path.to_string(),
                    existing: type_name(existing),
                    incoming: type_name(&incoming),
                });
            }
            *existing = incoming;
        }
    }
}

/// Reduces a list of samples into a single `Value` via `merge`.
///
/// Also returns every type conflict found between samples, so the caller can
/// report them. Returns `Null` for empty input.
pub fn merge_samples(samples: Vec<Value>) -> (Value, Vec<TypeConflict>) {
    let mut conflicts = Vec::new();

    let result = samples.into_iter().reduce(|mut acc, sample| {
        merge(&mut acc, sample, "", &mut conflicts);
        acc
    });

    (result.unwrap_or(Value::Null), conflicts)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn merge_two_objects() {
        let (result, conflicts) = merge_samples(vec![json!({"a": 1}), json!({"b": 2})]);

        assert_eq!(result, json!({"a": 1, "b": 2}));
        assert!(conflicts.is_empty());
    }

    #[test]
    fn merge_overwrites_keys() {
        let (result, conflicts) = merge_samples(vec![json!({"a": 1}), json!({"a": 2})]);

        assert_eq!(result, json!({"a": 2}));
        assert!(conflicts.is_empty());
    }

    #[test]
    fn merge_nested() {
        let (result, _) = merge_samples(vec![json!({"a": {"x": 1}}), json!({"a": {"y": 2}})]);

        assert_eq!(result, json!({"a": {"x": 1, "y": 2}}));
    }

    #[test]
    fn merge_single() {
        let (result, _) = merge_samples(vec![json!({"a": 1})]);

        assert_eq!(result, json!({"a": 1}));
    }

    #[test]
    fn merge_empty() {
        let (result, conflicts) = merge_samples(vec![]);

        assert_eq!(result, Value::Null);
        assert!(conflicts.is_empty());
    }

    #[test]
    fn merge_arrays() {
        let (result, _) = merge_samples(vec![json!([1, 2]), json!([3, 4])]);

        assert_eq!(result, json!([1, 2, 3, 4]));
    }

    #[test]
    fn merge_object_then_array_reports_conflict() {
        let (result, conflicts) = merge_samples(vec![json!({"a": 1}), json!([1, 2, 3])]);

        assert_eq!(result, json!([1, 2, 3]));
        assert_eq!(
            conflicts,
            vec![TypeConflict {
                path: String::new(),
                existing: "object",
                incoming: "array",
            }]
        );
    }

    #[test]
    fn merge_array_then_object_reports_conflict() {
        let (result, conflicts) = merge_samples(vec![json!([1, 2]), json!({"a": 1})]);

        assert_eq!(result, json!({"a": 1}));
        assert_eq!(conflicts.len(), 1);
    }

    #[test]
    fn merge_deep_nested_overwrite() {
        let (result, _) = merge_samples(vec![
            json!({"a": {"b": {"c": 1, "d": 2}}}),
            json!({"a": {"b": {"c": 99, "e": 3}}}),
        ]);
        assert_eq!(result, json!({"a": {"b": {"c": 99, "d": 2, "e": 3}}}));
    }

    #[test]
    fn merge_null_values() {
        let (result, conflicts) = merge_samples(vec![json!({"a": null}), json!({"b": 1})]);

        assert_eq!(result, json!({"a": null, "b": 1}));
        assert!(conflicts.is_empty());
    }

    #[test]
    fn null_does_not_erase_known_value() {
        let (result, conflicts) = merge_samples(vec![
            json!({"a": "keep"}),
            json!({"a": null}),
            json!({"b": 1}),
        ]);

        assert_eq!(result, json!({"a": "keep", "b": 1}));
        assert!(conflicts.is_empty());
    }

    #[test]
    fn known_value_replaces_initial_null() {
        let (result, conflicts) = merge_samples(vec![json!({"a": null}), json!({"a": 7})]);

        assert_eq!(result, json!({"a": 7}));
        assert!(conflicts.is_empty());
    }

    #[test]
    fn scalar_type_conflict_is_reported_with_path() {
        let (result, conflicts) = merge_samples(vec![
            json!({"user": {"id": 1}}),
            json!({"user": {"id": "abc"}}),
        ]);

        assert_eq!(result, json!({"user": {"id": "abc"}}));
        assert_eq!(
            conflicts,
            vec![TypeConflict {
                path: "user.id".to_string(),
                existing: "number",
                incoming: "string",
            }]
        );
    }

    #[test]
    fn scalar_conflict_at_root_is_reported() {
        let (_, conflicts) = merge_samples(vec![json!("text"), json!(42)]);

        assert_eq!(
            conflicts,
            vec![TypeConflict {
                path: String::new(),
                existing: "string",
                incoming: "number",
            }]
        );
    }

    #[test]
    fn scalar_versus_object_reports_conflict() {
        let (_, conflicts) = merge_samples(vec![json!({"a": 1}), json!({"a": {"b": 2}})]);

        assert_eq!(
            conflicts,
            vec![TypeConflict {
                path: "a".to_string(),
                existing: "number",
                incoming: "object",
            }]
        );
    }

    #[test]
    fn equal_scalar_types_do_not_conflict() {
        let (result, conflicts) = merge_samples(vec![
            json!({"s": "one", "n": 1, "b": true}),
            json!({"s": "two", "n": 2, "b": false}),
        ]);

        assert_eq!(result, json!({"s": "two", "n": 2, "b": false}));
        assert!(conflicts.is_empty());
    }
}
