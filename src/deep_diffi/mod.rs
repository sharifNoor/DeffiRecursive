pub mod mod_deep_diffi {

    use serde_derive::{Deserialize, Serialize};
    use serde_json::{json};

    // #[derive(Debug, Serialize, Deserialize, Clone)]
    // enum Res {
    //     DictionaryItemRemoved(serde_json::Value),
    //     DictionaryItemAdded(serde_json::Value),
    //     ValueChanged(serde_json::Value),
    //     IterableItemAdded(serde_json::Value),
    //     IterableItemRemoved(serde_json::Value),
    // }
    #[derive(Debug, Serialize, Clone)]
    pub struct Result {
        #[serde(flatten)]
        dictionary_item_removed: Vec<serde_json::Value>,
        #[serde(flatten)]
        dictionary_item_added: Vec<serde_json::Value>,
        #[serde(flatten)]
        value_changed: Vec<serde_json::Value>,
        #[serde(flatten)]
        iterable_item_added: Vec<serde_json::Value>,
        #[serde(flatten)]
        iterable_item_removed: Vec<serde_json::Value>,
    }

    pub fn josn_diffi(a_json: serde_json::Value, b_json: serde_json::Value, order: bool, ignore: Vec<&str>) -> Result {
       
        let init_result: Result = Result {
            dictionary_item_removed: Vec::<serde_json::Value>::new(),
            dictionary_item_added: Vec::<serde_json::Value>::new(),
            value_changed: Vec::<serde_json::Value>::new(),
            iterable_item_added: Vec::<serde_json::Value>::new(),
            iterable_item_removed: Vec::<serde_json::Value>::new(),
        };

        let ig: Vec<&str> = ignore[1].split('.').collect();
        println!("--> {:?}", ig);


        if !order {
            let result = deep_diffi(a_json, b_json, "root[".to_string(), &init_result, &ignore);
            result
        }
        else {
            let result = deep_diffi_order(a_json, b_json, "root[".to_string(), &init_result, &ignore);
            result
        }
    }

    fn check_ignore(ignore: &Vec<&str>, json1_key: &String, json1: &serde_json::Map<String, serde_json::Value>) -> bool {

        for ig in ignore.iter() {
            if ig.contains('.') {
                let iig: Vec<&str> = ig.split('.').collect();
                if iig[0] == json1_key {
                    if json1[json1_key].as_object().unwrap().contains_key(iig[1]) {
                        return false;
                    }
                }

            }
            else {
                if json1_key == ig {
                    return false;
                }
            }
        }
        true
    } 
    fn deep_diffi(a_json: serde_json::Value, b_json: serde_json::Value, _init_location: String, init_result: &Result, ignore: &Vec<&str>) -> Result {
        let json1: &serde_json::Map<String, serde_json::Value> = a_json.as_object().unwrap();
        let json2: &serde_json::Map<String, serde_json::Value> = b_json.as_object().unwrap();
        let mut _location: String;
        let mut sub_result: Result = init_result.to_owned();
        for (json1_key, _json1_value) in json1.iter() {
            _location = _init_location.to_owned() + &json1_key + "]";
            let chk = check_ignore(ignore, json1_key, json1);
            println!("{} --> {}", json1_key, chk);
            if !json2.contains_key(json1_key) {
                // dictionary_item_removed
                let val = _json1_value.to_owned();
                let formatted = json!({"location": _location, "value": val});
                sub_result.dictionary_item_removed.push(formatted);
            } else {
                // JSON-2 contains Key-1
                if _json1_value.is_object() {
                    // Recursion
                    let json3 = _json1_value;
                    let json4 = json2[json1_key].as_object().unwrap();
                    sub_result = deep_diffi(json!(json3), json!(json4), String::from(&_location) + "[", &sub_result, &ignore);
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
                                sub_result.iterable_item_removed.push(formatted);
                            }
                            else if json1_array_val.is_object() {
                                let j1i = _json1_value.as_array().unwrap().iter().position(|x| x == json1_array_val).unwrap();
                                let location1 = String::from(&_location) + "[" + &j1i.to_string() + "][";
                                let arr_1 = json1_array_val;
                                sub_result = deep_diffi(json!(&arr_1), json!(&json2[json1_key][j1i]), location1.to_string(), &sub_result, &ignore);
                            }
                        }
                        for json2_array_val in json2[json1_key].as_array().unwrap().iter() {
                            if !_json1_value.as_array().unwrap().contains(json2_array_val) && !json2_array_val.is_object() {
                                // Iterable Item Added
                                let val = json2_array_val.to_owned();
                                let formatted = json!({"location": _location, "value": val});
                                sub_result.iterable_item_added.push(formatted);
                            }
                        }
                    }
                } else {
                    // JSON-2 contains Key-1 && Value is not an Object or Array
                    if _json1_value != &json2[json1_key] {
                        // value_changed
                        let val = _json1_value.to_owned();
                        let formatted = json!({"location": _location, "value": val});
                        sub_result.value_changed.push(formatted);
                    }
                }
            }
        }
        for (json2_key, _json2_value) in json2.iter() {
            _location = _init_location.to_owned() + &json2_key + "]";
            if !json1.contains_key(json2_key) {
                // dictionary_item_added
                let val = _json2_value.to_owned();
                let formatted = json!({"location": _location, "value": val});
                sub_result.dictionary_item_added.push(formatted);
            }
        }

        sub_result

        // println!("{}", serde_json::to_string(&result).unwrap())

    }

    fn deep_diffi_order(a_json: serde_json::Value, b_json: serde_json::Value, _init_location: String, init_result: &Result, ignore: &Vec<&str>) -> Result {
        let json1: &serde_json::Map<String, serde_json::Value> = a_json.as_object().unwrap();
        let json2: &serde_json::Map<String, serde_json::Value> = b_json.as_object().unwrap();
        let mut _location: String;
        let mut sub_result: Result = init_result.to_owned();
        for (json1_i, (json1_key, _json1_value)) in json1.iter().enumerate() {
            _location = _init_location.to_owned() + &json1_key + "]";
            let json2_key = json2.iter().nth(json1_i);

            if json2_key.is_some() {
                if json1_key == json2_key.unwrap().0 {
                    if !_json1_value.is_object() && !_json1_value.is_array(){
                        if _json1_value != json2_key.unwrap().1 {
                            let val = _json1_value.to_owned();
                            let formatted = json!({"location": _location, "value": val});
                            sub_result.value_changed.push(formatted);
                        }
                    }
                    else if _json1_value.is_object() {
                        let json3 = _json1_value;
                        let json4 = json2[json1_key].as_object().unwrap();
                        sub_result = deep_diffi_order(json!(json3), json!(json4), String::from(&_location) + "[", &sub_result, &ignore);
                    }
                    else if _json1_value.is_array() {
                        if json2[json1_key].is_array() {
                            for (json1_array_val_i, json1_array_val) in _json1_value.as_array().unwrap().iter().enumerate() {
                                let json4_key = &json2[json1_key][json1_array_val_i];
                                
                                if json1_array_val.is_object() {
                                    let j1i = _json1_value.as_array().unwrap().iter().position(|x| x == json1_array_val).unwrap();
                                    let location1 = String::from(&_location) + "[" + &j1i.to_string() + "][";
                                    let arr_1 = json1_array_val;
                                    sub_result = deep_diffi_order(json!(&arr_1), json!(&json2[json1_key][j1i]), location1.to_string(), &sub_result, &ignore);
                                }
                                else {
                                    if json1_array_val != json4_key {
                                        let j1i = _json1_value.as_array().unwrap().iter().position(|x| x == json1_array_val).unwrap();
                                        let location1 = String::from(&_location) + "[" + &j1i.to_string() + "]";
                                        let val = json1_array_val.to_owned();
                                        let formatted = json!({"location": location1, "value": val});
                                        sub_result.value_changed.push(formatted);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            else {
                println!("===> Order Compromised!");
                sub_result.value_changed.clear();
                sub_result = deep_diffi(json!(a_json), json!(b_json), "root[".to_string(), &sub_result, &ignore);
            }
        }
        sub_result
    }
}
