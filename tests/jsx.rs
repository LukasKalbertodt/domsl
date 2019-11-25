#![feature(proc_macro_hygiene)]

use domsl::jsx;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::Document;

wasm_bindgen_test_configure!(run_in_browser);


// ===== Helper functions and macros =========================================

fn doc() -> Document {
    web_sys::window().unwrap().document().unwrap()
}

macro_rules! assert_only_has_text {
    ($n:ident, $t:literal) => {
        assert_eq!($n.inner_text(), $t);
        assert_eq!($n.inner_html(), $t);
        assert_eq!($n.children().length(), 0);
    };
}

macro_rules! assert_cast {
    ($n:ident, $ty:ident) => {
        match $n.dyn_into::<web_sys::$ty>() {
            Ok(v) => v,
            Err(n) => {
                panic!(
                    "expected '{}', but value is actually: {:#?}",
                    stringify!($ty),
                    n,
                );
            }
        }
    };
}

macro_rules! assert_into_children {
    ($n:ident, [$($i:expr),*]) => {{
        if $n.children().length() != [$($i),*].len() as u32 {
            panic!(
                "expected '{}' to have {} children, but it has {} children",
                stringify!($n),
                $n.children().length(),
                [$($i),*].len(),
            );
        }

        [$(
            $n.children().item($i).unwrap()
        ),*]
    }};
}



// ===== Actual tests ========================================================

#[wasm_bindgen_test]
fn simple_div() {
    let d = doc();
    let n: web_sys::HtmlDivElement = jsx!(d => { <div></div> });
    assert_only_has_text!(n, "");
}

#[wasm_bindgen_test]
fn span_child() {
    let d = doc();
    let n: web_sys::HtmlDivElement = jsx!(d => {
        <div><span>"Hello"</span></div>
    });

    assert!(!n.inner_text().is_empty());
    assert!(!n.inner_html().is_empty());
    let [span] = assert_into_children!(n, [0]);

    let span = assert_cast!(span, HtmlSpanElement);
    assert_only_has_text!(span, "Hello");
}

#[wasm_bindgen_test]
fn all_into_node() {
    let d = doc();

    // We test all implementations that can convert a value into a node.
    let p_node = jsx!(d => {
        <p>"Hello p"</p>
    });
    let s1 = "s1";
    let s2 = String::from("s2");
    let node_vec = vec![
        jsx!(d => { <span>"span1"</span> }),
        jsx!(d => { <span>"span2"</span> }),
    ];

    let out: web_sys::HtmlDivElement = jsx!(d => {
        <div class="baz">
            { p_node }
            <div>{ s1 }</div>
            <div>{ s2 }</div>
            <div>{ 27 }</div>
            <div>{ node_vec }</div>
            <div>{ vec!["hi", "yo"] }</div>
            <div>{ &[1, 2, 3] }</div>
        </div>
    });

    let [c0, c1, c2, c3, c4, c5, c6] = assert_into_children!(out, [0, 1, 2, 3, 4, 5, 6]);

    let n = assert_cast!(c0, HtmlParagraphElement);
    assert_only_has_text!(n, "Hello p");
    let n = assert_cast!(c1, HtmlDivElement);
    assert_only_has_text!(n, "s1");
    let n = assert_cast!(c2, HtmlDivElement);
    assert_only_has_text!(n, "s2");
    let n = assert_cast!(c3, HtmlDivElement);
    assert_only_has_text!(n, "27");

    let n = assert_cast!(c5, HtmlDivElement);
    assert_only_has_text!(n, "hiyo");
    let n = assert_cast!(c6, HtmlDivElement);
    assert_only_has_text!(n, "123");

    assert_eq!(c4.children().length(), 2);
    let cc0 = c4.children().item(0).unwrap();
    let cc1 = c4.children().item(1).unwrap();
    let n = assert_cast!(cc0, HtmlSpanElement);
    assert_only_has_text!(n, "span1");
    let n = assert_cast!(cc1, HtmlSpanElement);
    assert_only_has_text!(n, "span2");
}
