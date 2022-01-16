use super::{Map, Number, Value};
use std::cmp::{Ord, Ordering};

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn resolve_external(v: &Value) -> &Value {
    match v {
        Value::External(inner) => resolve_external(inner.as_ref()),
        _ => v,
    }
}

fn type_rank(v: &Value) -> i64 {
    match v {
        Value::Null => 0,
        Value::Bool(_) => 1,
        Value::Number(_) => 2,
        Value::String(_) => 3,
        Value::Array(_) => 4,
        Value::Object(_) => 5,
        Value::External(inner) => type_rank(inner),
    }
}

fn cmp_number(a: &Number, b: &Number) -> Ordering {
    let (ai, bi) = (a.as_i64().expect(""), b.as_i64().expect(""));
    return ai.cmp(&bi);
}

#[cfg(feature = "preserve_order")]
fn sorted_keys(m: &Map<String, Value>) -> Vec<&String> {
    let mut keys: Vec<&String> = m.keys().collect();
    keys.sort();
    return keys;
}

#[cfg(not(feature = "preserve_order"))]
fn sorted_keys(m: &Map<String, Value>) -> Vec<&String> {
    m.keys().collect()
}

fn cmp_object(a: &Map<String, Value>, b: &Map<String, Value>) -> Ordering {
    let (keys1, keys2) = (sorted_keys(a), sorted_keys(b));

    let c = keys1.cmp(&keys2);
    match c {
        Ordering::Equal => {}
        _ => {
            return c;
        }
    }

    for k in keys1 {
        let (av, bv) = (a.get(k), b.get(k));
        let c = av.cmp(&bv);
        match c {
            Ordering::Equal => {}
            _ => {
                return c;
            }
        };
    }

    return Ordering::Equal;
}

impl Ord for Value {
    fn cmp(&self, other: &Value) -> Ordering {
        let (self_rank, other_rank) = (type_rank(self), type_rank(other));
        if self_rank < other_rank {
            return Ordering::Less;
        } else if self_rank > other_rank {
            return Ordering::Greater;
        } else {
            match resolve_external(self) {
                Value::Null => Ordering::Equal,
                Value::Bool(b) => b.cmp(&other.as_bool().expect("")),
                Value::Number(n1) => match other {
                    Value::Number(n2) => cmp_number(n1, n2),
                    _ => panic!(),
                },
                Value::String(s) => s.as_str().cmp(&other.as_str().expect("")),
                Value::Array(a1) => match other {
                    Value::Array(a2) => a1.cmp(a2),
                    _ => panic!(),
                },
                Value::Object(o1) => match other {
                    Value::Object(o2) => cmp_object(o1, o2),
                    _ => panic!(),
                },
                _ => panic!(),
            }
        }
    }
}

impl PartialOrd for Map<String, Value> {
    fn partial_cmp(&self, other: &Map<String, Value>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Map<String, Value> {
    fn cmp(&self, other: &Map<String, Value>) -> Ordering {
        cmp_object(self, other)
    }
}
