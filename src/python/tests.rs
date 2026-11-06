use std::rc::Rc;

use pyo3::ffi::c_str;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use serde::Serialize;

use crate::{Message, Process, ProcessState, System};

use crate::python::{get_size_fun, PyProcessFactory};

fn build_system() -> (System, Rc<dyn ProcessState>) {
    let mut sys = System::new(0);
    sys.add_node("node");
    let proc_f = PyProcessFactory::new("tests/python/process.py", "TestProcess");
    let process = proc_f.build((), 1);
    let state = process.state().unwrap();
    sys.add_process("proc", Box::new(process), "node");
    (sys, state)
}

#[derive(Serialize)]
struct EmptyMessage {}

#[test]
fn test_set_state() {
    let (mut sys, proc_state) = build_system();

    // check that process stores valid data
    // and update inner state (with `tmp_value` member)
    sys.send_local_message("proc", Message::json("CHECK_STATE", &EmptyMessage {}));
    sys.step_until_no_events();

    // reset process state to initial
    sys.get_mut_node("node").unwrap().set_process_state("proc", proc_state);
    // check that process stores the same data as right after initialization
    sys.send_local_message("proc", Message::json("CHECK_STATE", &EmptyMessage {}));
    sys.step_until_no_events();
}

#[test]
fn test_embedded_module_is_reused() {
    Python::attach(|py| {
        let first = PyProcessFactory::new("tests/python/process.py", "TestProcess");
        let second = PyProcessFactory::new("tests/python/process.py", "TestProcess");

        assert_eq!(first.msg_class.bind(py).as_ptr(), second.msg_class.bind(py).as_ptr());
        assert_eq!(first.ctx_class.bind(py).as_ptr(), second.ctx_class.bind(py).as_ptr());

        let fake_module = PyModule::new(py, "anysystem").unwrap();
        PyModule::import(py, "sys")
            .unwrap()
            .getattr("modules")
            .unwrap()
            .set_item("anysystem", fake_module)
            .unwrap();
        let third = PyProcessFactory::new("tests/python/process.py", "TestProcess");

        let module = PyModule::import(py, "anysystem").unwrap();
        assert_eq!(
            first.msg_class.bind(py).as_ptr(),
            module.getattr("Message").unwrap().as_ptr()
        );
        assert_eq!(
            first.ctx_class.bind(py).as_ptr(),
            module.getattr("Context").unwrap().as_ptr()
        );
        assert_eq!(first.msg_class.bind(py).as_ptr(), third.msg_class.bind(py).as_ptr());
        assert_eq!(first.ctx_class.bind(py).as_ptr(), third.ctx_class.bind(py).as_ptr());
    });
}

fn check_memory_size(test: &str) {
    Python::attach(|py| {
        let module = PyModule::from_code(
            py,
            c_str!(include_str!("../../tests/python/memory.py")),
            c"memory.py",
            c"memory_tests",
        )
        .unwrap();
        module.getattr(test).unwrap().call1((get_size_fun(py),)).unwrap();
    });
}

#[test]
fn test_array_memory_size() {
    check_memory_size("check_array_storage");
}

#[test]
fn test_array_memory_references() {
    check_memory_size("check_array_references");
}

#[test]
fn test_array_subclass_memory_size() {
    check_memory_size("check_array_subclasses");
}

#[test]
fn test_array_process_max_size() {
    let factory = PyProcessFactory::new("tests/python/array_process.py", "ArrayProcess");
    let mut process = factory.build((), 1);
    process.set_max_size_freq(1);
    let initial_size = process.max_size();
    let expected_growth = Python::attach(|py| {
        let module = PyModule::from_code(
            py,
            c_str!(include_str!("../../tests/python/memory.py")),
            c"memory.py",
            c"memory_tests",
        )
        .unwrap();
        module
            .getattr("array_growth")
            .unwrap()
            .call0()
            .unwrap()
            .extract::<u64>()
            .unwrap()
    });

    let mut sys = System::new(0);
    sys.add_node("node");
    sys.add_process("proc", Box::new(process), "node");
    sys.send_local_message("proc", Message::json("FILL", &EmptyMessage {}));
    sys.step_until_no_events();
    assert_eq!(sys.max_size("proc"), initial_size + expected_growth);

    sys.send_local_message("proc", Message::json("CLEAR", &EmptyMessage {}));
    sys.step_until_no_events();
    assert_eq!(sys.max_size("proc"), initial_size + expected_growth);
}
