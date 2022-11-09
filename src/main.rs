mod deep_diffi;
use deep_diffi::mod_deep_diffi::josn_diffi;
use serde_json::json;

fn main() {
    let a_json = json!({
        "key": "value_1",
        "key2": {
            "key_2": "value_2",
            "key2": [
                "value_3",
                "value_4",
                "value_6",
            ],
            "key3": "value_2",
        },
        "key4": [
            {
                "keyj4": {
                    "keyj45": "valuej_2",
                    "keyj462": "valuej_2222222",
                },
                "keyj5": "valuej_2",
                "keyj62": "valuej_22",
            },
            {
                "keyj42": "valuej_22",
                "keyj52": "valuej_22",
            }
        ],
        "key5": {
            "key8": "value_4",
            "key9": [
                "value_3",
                "value_5",
                "value_6",
            ],
        },
    });

    let b_json = json!({
        "key": "value_1",
        "key2": {
            "key_2": "value_4",
            "key2": [
                "value_3",
                "value_5",
                "value_6",
            ],
        },
        "key4": [
            {
                "keyj4": {
                    "keyj45": "valuej_2222",
                    "keyj462": "valuej_22",
                },
                "keyj5": "valuej_2",
                "keyj62": "valuej_22",
            },
            {
                "keyj42": "valuej_22",
                "keyj52": "valuej_22",
            }
        ],
        "key5": {
            "key8": "value_4",
            "key9": [
                "value_3",
                "value_5",
                "value_6",
            ],
        },
    });

    let ignore_list: Vec<&str> = ["key2", "key4.keyj4.keyj462"].to_vec();
    let result = josn_diffi(a_json, b_json, false, ignore_list);
    println!("===> {:#?}", result);
}
