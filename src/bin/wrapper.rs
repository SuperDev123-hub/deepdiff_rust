use pyo3::prelude::*;

fn main() -> PyResult<()> {
    let py_lib = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/bin/pylib/diffwrapper.py"
    ));

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

    let from_python = Python::with_gil(|py| {
        let result = PyModule::from_code(py, py_lib, "", "")
            .unwrap()
            .call_method("diff", (json1, json2, true), None)
            .unwrap();
        println!("{:?}", result.extract::<&str>());
    });

    Ok(())
}
