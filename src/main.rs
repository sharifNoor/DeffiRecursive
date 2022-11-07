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
                "keyj4": "valuej_222",
                "keyj5": "valuej_2",
            },
            {
                "keyj42": "valuej_22",
                "keyj52": "valuej_22",
                "keyj62": "valuej_22",
            }
        ],
        "key5": "value_1",
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
                "keyj4": "valuej_2",
                "keyj5": "valuej_2",
                "keyj62": "valuej_22",
            }, 
            {
                "keyj42": "valuej_22",
                "keyj52": "valuej_22",
            }
        ],
        "key5": "value_2",
    });

    let ign: Vec<&str> = ["key2", "key4.keyj4"].to_vec();
    let o = josn_diffi(a_json, b_json, false, ign);
    // println!("===> {:#?}", o);

}
