//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use bindgen::callbacks::{MacroParsingBehavior, ParseCallbacks};
use pgrx_c_macros::{MacroInventory, MacroScanner};
use std::cell::{Cell, RefCell};
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

fn fixture() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/definitions.h")
}

fn assert_inventory(inventory: &MacroInventory) {
    for name in ["FUNCTION", "INCLUDED_VALUE"] {
        assert!(inventory.macros.iter().any(|definition| definition.name == name));
    }
}

fn assert_runtime(expected: &Arc<clang_sys::SharedLibrary>) {
    let current = clang_sys::get_library().expect("the Clang runtime must remain loaded");
    assert!(Arc::ptr_eq(expected, &current), "the original Clang runtime must be retained");
}

#[derive(Debug)]
struct ScanCallback {
    called: Rc<Cell<bool>>,
    existing: Option<Rc<RefCell<Option<MacroScanner>>>>,
}

impl ParseCallbacks for ScanCallback {
    fn will_parse_macro(&self, name: &str) -> MacroParsingBehavior {
        if name != "CALLBACK_VALUE" {
            return MacroParsingBehavior::Default;
        }
        assert!(!self.called.replace(true), "the test macro must be processed once");
        let runtime = clang_sys::get_library().expect("bindgen must have loaded Clang");
        if let Some(existing) = &self.existing {
            // This scanner loaded Clang before bindgen created its translation unit.
            let scanner = existing.borrow_mut().take().expect("the scanner must still be alive");
            drop(scanner);
        } else {
            // This scanner is created while bindgen's translation unit is already alive.
            let scanner = MacroScanner::new().expect("the scanner must share bindgen's runtime");
            assert_runtime(&runtime);
            let inventory = scanner.scan(&fixture(), &[]).expect("the fixture must parse");
            drop(scanner);
            assert_inventory(&inventory);
        }
        assert_runtime(&runtime);
        MacroParsingBehavior::Default
    }
}

fn generate(callback: ScanCallback) -> String {
    bindgen::Builder::default()
        .header_contents(
            "pgrx-c-macros-runtime-test.h",
            "#define CALLBACK_VALUE 37\n\
             typedef struct BindingValue { int value; } BindingValue;\n\
             int callback_function(BindingValue *value);\n",
        )
        .clang_args(["-x", "c"])
        .allowlist_var("CALLBACK_VALUE")
        .allowlist_type("BindingValue")
        .allowlist_function("callback_function")
        .parse_callbacks(Box::new(callback))
        .layout_tests(false)
        .formatter(bindgen::Formatter::None)
        .generate()
        .expect("bindgen must complete after the scanner is dropped")
        .to_string()
}

fn assert_bindings(bindings: &str) {
    for declaration in
        ["pub const CALLBACK_VALUE", "pub struct BindingValue", "pub fn callback_function"]
    {
        assert!(bindings.contains(declaration), "missing {declaration}: {bindings}");
    }
}

// One test covers both construction orders without concurrent clang wrapper handles.
#[test]
fn discovery_preserves_bindgens_runtime_in_both_construction_orders() {
    assert!(clang_sys::get_library().is_none(), "the test starts before Clang initialization");
    let scanner = MacroScanner::new().expect("libclang must be available for this test");
    let inventory = scanner.scan(&fixture(), &[]).expect("the fixture must parse");
    assert_inventory(&inventory);
    let runtime = clang_sys::get_library().expect("discovery must have loaded Clang");
    let existing = Rc::new(RefCell::new(Some(scanner)));
    let called = Rc::new(Cell::new(false));
    let bindings =
        generate(ScanCallback { called: Rc::clone(&called), existing: Some(Rc::clone(&existing)) });
    assert!(called.get(), "bindgen must invoke its macro callback");
    assert!(existing.borrow().is_none(), "the callback must drop the original scanner");
    assert_bindings(&bindings);
    assert_runtime(&runtime);

    let called = Rc::new(Cell::new(false));
    let bindings = generate(ScanCallback { called: Rc::clone(&called), existing: None });
    assert!(called.get(), "bindgen must invoke its macro callback");
    assert_bindings(&bindings);
    assert_runtime(&runtime);

    // Ordinary discovery also remains usable after bindgen has completed.
    let scanner = MacroScanner::new().expect("the scanner must reuse the existing runtime");
    assert_runtime(&runtime);
    let inventory = scanner.scan(&fixture(), &[]).expect("the fixture must parse");
    drop(scanner);
    assert_inventory(&inventory);
    assert_runtime(&runtime);
}
