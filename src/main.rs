mod cli;
mod codegen;
mod error;
mod format;
mod input;

use clap::Parser;
use serde_json::Value;

fn merge(values: &mut Value, incoming: Value) {
    match (values, incoming) {
        (Value::Object(existing), Value::Object(map)) => {
            for (k, v) in map {
                if let Some(ev) = existing.get_mut(&k) {
                    merge(ev, v);
                } else {
                    existing.insert(k, v);
                }
            }
        }
        (Value::Array(existing), Value::Array(arr)) => {
            existing.extend(arr);
        }
        (existing, incoming) => *existing = incoming,
    }
}

fn merge_samples(mut samples: Vec<Value>) -> Value {
    if samples.is_empty() {
        return Value::Null;
    }
    let mut result = samples.remove(0);
    for sample in samples {
        merge(&mut result, sample);
    }
    result
}

fn main() -> miette::Result<()> {
    let cli = cli::Cli::parse();

    let samples = input::collect_samples(&cli)?;
    if samples.is_empty() {
        return Ok(());
    }

    let merged = merge_samples(samples);

    let code = codegen::generate(&cli.name, &merged, cli.lang)?;
    println!("{code}");

    Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn merge_two_objects() {
        let result = merge_samples(vec![json!({"a": 1}), json!({"b": 2})]);
        assert_eq!(result, json!({"a": 1, "b": 2}));
    }

    #[test]
    fn merge_overwrites_keys() {
        let result = merge_samples(vec![json!({"a": 1}), json!({"a": 2})]);
        assert_eq!(result, json!({"a": 2}));
    }

    #[test]
    fn merge_nested() {
        let result = merge_samples(vec![json!({"a": {"x": 1}}), json!({"a": {"y": 2}})]);
        assert_eq!(result, json!({"a": {"x": 1, "y": 2}}));
    }

    #[test]
    fn merge_single() {
        let result = merge_samples(vec![json!({"a": 1})]);
        assert_eq!(result, json!({"a": 1}));
    }

    #[test]
    fn merge_empty() {
        let result = merge_samples(vec![]);
        assert_eq!(result, Value::Null);
    }

    #[test]
    fn merge_arrays() {
        let result = merge_samples(vec![json!([1, 2]), json!([3, 4])]);
        assert_eq!(result, json!([1, 2, 3, 4]));
    }

    #[test]
    fn merge_object_then_array_overwrites() {
        let result = merge_samples(vec![json!({"a": 1}), json!([1, 2, 3])]);
        assert_eq!(result, json!([1, 2, 3]));
    }

    #[test]
    fn merge_array_then_object_overwrites() {
        let result = merge_samples(vec![json!([1, 2]), json!({"a": 1})]);
        assert_eq!(result, json!({"a": 1}));
    }

    #[test]
    fn merge_deep_nested_overwrite() {
        let result = merge_samples(vec![
            json!({"a": {"b": {"c": 1, "d": 2}}}),
            json!({"a": {"b": {"c": 99, "e": 3}}}),
        ]);
        assert_eq!(result, json!({"a": {"b": {"c": 99, "d": 2, "e": 3}}}));
    }

    #[test]
    fn merge_null_values() {
        let result = merge_samples(vec![json!({"a": null}), json!({"b": 1})]);
        assert_eq!(result, json!({"a": null, "b": 1}));
    }
}
