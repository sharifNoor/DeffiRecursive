pub mod mod_deep_diffi {

    use serde::{Serialize};
    use serde_derive::{Deserialize, Serialize};
    use serde_json::{json, Value};

    #[derive(Debug, Serialize, Deserialize)]
    struct Foo<T>
    where
        T: Serialize,
    {
        foo: T,
    }

    #[derive(Debug, Serialize)]
    struct Bar {
        dictionary_item_removed: serde_json::Value,
    }

    #[derive(Debug, Serialize)]
    struct Bar5 {
        dictionary_item_added: serde_json::Value,
    }

    #[derive(Debug, Serialize)]
    struct Bar2 {
        iterable_item_removed: serde_json::Value,
    }

    #[derive(Debug, Serialize)]
    struct Bar4 {
        iterable_item_added: serde_json::Value,
    }

    #[derive(Debug, Serialize)]
    struct Bar3 {
        value_changed: serde_json::Value,
    }

    #[derive(Debug, Serialize)]
    pub struct Outer {
        #[serde(flatten)]
        field_5: Bar5,
        #[serde(flatten)]
        field_1: Bar,
        #[serde(flatten)]
        field_4: Bar4,
        #[serde(flatten)]
        field_2: Bar2,
        #[serde(flatten)]
        field_3: Bar3,
    }

    static mut DICTIONARY_ITEM_REMOVED_ARRAY: Vec<Value> = Vec::new();
    static mut DICTIONARY_ITEM_ADDED_ARRAY: Vec<Value> = Vec::new();
    static mut VALUE_CHANGED_ARRAY: Vec<Value> = Vec::new();
    static mut ITERABLE_ITEM_ADDED: Vec<Value> = Vec::new();
    static mut ITERABLE_ITEM_REMOVED: Vec<Value> = Vec::new();

    pub fn josn_diffi(a_json: serde_json::Value, b_json: serde_json::Value, order: bool) -> Outer {

        if !order {
            deep_diffi(a_json, b_json, "root[".to_string());
        }
        else {
            deep_diffi_order(a_json, b_json, "root[".to_string());
        }
        
        let dictionary_item_removed: Bar;
        let dictionary_item_added: Bar5;
        let value_changed: Bar3;
        let iterable_item_removed: Bar2;
        let iterable_item_added: Bar4;

        unsafe {
            dictionary_item_removed = Bar {
                dictionary_item_removed: serde_json::Value::Array(
                    DICTIONARY_ITEM_REMOVED_ARRAY.to_owned(),
                ),
            };
            dictionary_item_added = Bar5 {
                dictionary_item_added: serde_json::Value::Array(
                    DICTIONARY_ITEM_ADDED_ARRAY.to_owned(),
                ),
            };
            value_changed = Bar3 {
                value_changed: serde_json::Value::Array(VALUE_CHANGED_ARRAY.to_owned()),
            };
            iterable_item_added = Bar4 {
                iterable_item_added: serde_json::Value::Array(ITERABLE_ITEM_ADDED.to_owned()),
            };
            iterable_item_removed = Bar2 {
                iterable_item_removed: serde_json::Value::Array(ITERABLE_ITEM_REMOVED.to_owned()),
            };
        }

        let o = Outer {
            field_5: dictionary_item_added,
            field_1: dictionary_item_removed,
            field_4: iterable_item_added,
            field_2: iterable_item_removed,
            field_3: value_changed,
        };

        return o;
        
    }

    fn deep_diffi(a_json: serde_json::Value, b_json: serde_json::Value, _init_location: String) {
        let json1: &serde_json::Map<String, serde_json::Value> = a_json.as_object().unwrap();
        let json2: &serde_json::Map<String, serde_json::Value> = b_json.as_object().unwrap();
        let mut _location: String;
        for (json1_key, _json1_value) in json1.iter() {
            _location = _init_location.to_owned() + &json1_key + "]";
            if !json2.contains_key(json1_key) {
                // dictionary_item_removed
                let val = _json1_value.to_owned();
                let formatted = json!({"location": _location, "value": val});
                unsafe {
                    DICTIONARY_ITEM_REMOVED_ARRAY.push(formatted);
                }
            } else {
                // JSON-2 contains Key-1
                if _json1_value.is_object() {
                    // Recursion
                    let json3 = _json1_value;
                    let json4 = json2[json1_key].as_object().unwrap();
                    deep_diffi(json!(json3), json!(json4), String::from(&_location) + "[");
                } else if _json1_value.is_array() {
                    if json2[json1_key].is_array() {
                        for json1_array_val in _json1_value.as_array().unwrap().iter() {
                            if !json2[json1_key]
                                .as_array()
                                .unwrap()
                                .contains(json1_array_val) && !json1_array_val.is_object()
                            {
                                // Iterable Item Removed
                                let val = json1_array_val.to_owned();
                                let formatted = json!({"location": _location, "value": val});
                                unsafe {
                                    ITERABLE_ITEM_REMOVED.push(formatted);
                                }
                            }
                            else if json1_array_val.is_object() {
                                let j1i = _json1_value.as_array().unwrap().iter().position(|x| x == json1_array_val).unwrap();
                                let location1 = String::from(&_location) + "[" + &j1i.to_string() + "][";
                                let arr_1 = json1_array_val;
                                deep_diffi(json!(&arr_1), json!(&json2[json1_key][j1i]), location1.to_string());
                            }
                        }
                        for json2_array_val in json2[json1_key].as_array().unwrap().iter() {
                            if !_json1_value.as_array().unwrap().contains(json2_array_val) && !json2_array_val.is_object() {
                                // Iterable Item Added
                                let val = json2_array_val.to_owned();
                                let formatted = json!({"location": _location, "value": val});
                                unsafe {
                                    ITERABLE_ITEM_ADDED.push(formatted);
                                }
                            }
                        }
                    }
                } else {
                    // JSON-2 contains Key-1 && Value is not an Object or Array
                    if _json1_value != &json2[json1_key] {
                        // value_changed
                        let val = _json1_value.to_owned();
                        let formatted = json!({"location": _location, "value": val});
                        unsafe {
                            VALUE_CHANGED_ARRAY.push(formatted);
                        }
                    }
                }
            }
        }
        for (json2_key, _json2_value) in json2 {
            _location = _init_location.to_owned() + &json2_key + "]";
            if !json1.contains_key(json2_key) {
                // dictionary_item_added
                let val = _json2_value.to_owned();
                let formatted = json!({"location": _location, "value": val});
                unsafe {
                    DICTIONARY_ITEM_ADDED_ARRAY.push(formatted);
                }
            }
        }
    }

    fn deep_diffi_order(a_json: serde_json::Value, b_json: serde_json::Value, _init_location: String) {
        let json1: &serde_json::Map<String, serde_json::Value> = a_json.as_object().unwrap();
        let json2: &serde_json::Map<String, serde_json::Value> = b_json.as_object().unwrap();
        let mut _location: String;

        for (json1_i, (json1_key, _json1_value)) in json1.iter().enumerate() {
            _location = _init_location.to_owned() + &json1_key + "]";
            let json2_key = json2.iter().nth(json1_i);

            if json2_key.is_some() {
                if json1_key == json2_key.unwrap().0 {
                    if !_json1_value.is_object() && !_json1_value.is_array(){
                        if _json1_value != json2_key.unwrap().1 {
                            let val = _json1_value.to_owned();
                            let formatted = json!({"location": _location, "value": val});
                            unsafe {
                                VALUE_CHANGED_ARRAY.push(formatted);
                            }
                        }
                    }
                    else if _json1_value.is_object() {
                        let json3 = _json1_value;
                        let json4 = json2[json1_key].as_object().unwrap();
                        deep_diffi_order(json!(json3), json!(json4), String::from(&_location) + "[");
                    }
                    else if _json1_value.is_array() {
                        if json2[json1_key].is_array() {
                            for (json1_array_val_i, json1_array_val) in _json1_value.as_array().unwrap().iter().enumerate() {
                                let json4_key = &json2[json1_key][json1_array_val_i];
                                
                                if json1_array_val.is_object() {
                                    let j1i = _json1_value.as_array().unwrap().iter().position(|x| x == json1_array_val).unwrap();
                                    let location1 = String::from(&_location) + "[" + &j1i.to_string() + "][";
                                    let arr_1 = json1_array_val;
                                    deep_diffi_order(json!(&arr_1), json!(&json2[json1_key][j1i]), location1.to_string());
                                }
                                else {
                                    if json1_array_val != json4_key {
                                        let j1i = _json1_value.as_array().unwrap().iter().position(|x| x == json1_array_val).unwrap();
                                        let location1 = String::from(&_location) + "[" + &j1i.to_string() + "]";
                                        let val = json1_array_val.to_owned();
                                        let formatted = json!({"location": location1, "value": val});
                                        unsafe {
                                            VALUE_CHANGED_ARRAY.push(formatted);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            else {
                println!("===> Order Compromised!");
                unsafe {
                    VALUE_CHANGED_ARRAY.clear();
                }
                deep_diffi(json!(a_json), json!(b_json), "root[".to_string());
            }
        }
    }
}
