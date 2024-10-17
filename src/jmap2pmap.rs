use std::collections::BTreeMap;

use serde_json::Map;
use serde_json::Number;
use serde_json::Value;

use prost_types::value::Kind;
use prost_types::ListValue;
use prost_types::Struct;
use prost_types::Value as PbValue;

pub fn jarr2plist(j: Vec<Value>) -> ListValue {
    ListValue {
        values: j.into_iter().map(jval2pval).collect(),
    }
}

pub fn jnum2pkind(n: Number) -> Option<Kind> {
    n.as_f64()
        .or_else(|| n.as_i64().map(|i| i as f64))
        .or_else(|| n.as_u64().map(|i| i as f64))
        .map(Kind::NumberValue)
}

pub fn jval2kind(j: Value) -> Option<Kind> {
    match j {
        Value::Null => None,
        Value::Bool(b) => Some(Kind::BoolValue(b)),
        Value::Number(num) => jnum2pkind(num),
        Value::String(s) => Some(Kind::StringValue(s)),
        Value::Array(v) => Some(Kind::ListValue(jarr2plist(v))),
        Value::Object(m) => Some(Kind::StructValue(jmap2pmap(m))),
    }
}

pub fn jval2pval(j: Value) -> PbValue {
    PbValue { kind: jval2kind(j) }
}

pub fn jmap2pmap(j: Map<String, Value>) -> Struct {
    let i = j.into_iter();
    Struct {
        fields: BTreeMap::from_iter(i.map(|pair| {
            let (key, val) = pair;
            (key, jval2pval(val))
        })),
    }
}
