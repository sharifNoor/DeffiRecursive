mod deep_diffi;
use serde_json::json;
use deep_diffi::mod_deep_diffi::josn_diffi;

fn main() {

    let a_json = json!({
        "key": "value_1",
        "key2": {
            "key_2": "value_2",
            "key2": [
                "value_3",
                "value_4",
            ],
            "key3": "value_2",
        },
        "key4": [{"keyj4": "valuej_222","keyj5": "valuej_2"}, {"keyj42": "valuej_22","keyj52": "valuej_22","keyj62": "valuej_22"}],
    });

    let b_json = json!({
        "key2": {
            "key": "value_2",
            "key2": [
                "value_3",
                "value_5",
                "value_6",
            ],
            "key3": "value_3",
        },
        "key4": [{"keyj4": "valuej_2","keyj5": "valuej_2"}, {"keyj42": "valuej_22","keyj52": "valuej_22"}],
    });

    let o = josn_diffi(a_json, b_json);
    // println!("===> {}", serde_json::to_string(&o).unwrap());
}