use cpython::{py_fn, ObjectProtocol, PyDict, PyResult, PyString, Python};
use std::process::Command;
fn main() {
    let gil = Python::acquire_gil();
    let py_interface = gil.python();

    Command::new("pip")
        .arg("install")
        .arg("deepdiff")
        .status()
        .unwrap();

    let deepdiff = py_interface.import("deepdiff").unwrap();

    let json1: &str = r#"[
        {
            "a1": "87",
            "a2": "57",
            "a3": "23"
        },
        {
            "key1": "12",
            "key2": "3223",
            "key3": "233"
        },
        {
            "lastkey1": "87",
            "lastkey2": "57",
            "lastkey35": "22"
        }    
    ]"#;

    let json2: &str = r#"[
        {
            "key1": "12",
            "key2": "3223",
            "key3": "233"
        },
        {
            "lastkey1": "1",
            "lastkey2": "2",
            "lastkey3": "3",
            "lastkey4": "4"
        }    
    ]"#;

    let json1_str = PyString::new(py_interface, json1);
    let json2_str = PyString::new(py_interface, json2);
    // let res: String = deepdiff
    //     .call(py_interface, "DeepDiff", (json1_str, json2_str), None)
    //     .unwrap()
    //     .extract(py_interface)
    //     .unwrap();

    let diff_fn = deepdiff.get(py_interface, "DeepDiff").unwrap();
    let ret = diff_fn
        .call(py_interface, (json1_str, json2_str), None)
        .unwrap();

    let rest_str: PyString = ret
        .call_method(py_interface, "__str__", cpython::NoArgs, None)
        .unwrap()
        .extract(py_interface)
        .unwrap();
    println!(
        "Hello, world! {}",
        rest_str.to_string(py_interface).unwrap()
    );
}
