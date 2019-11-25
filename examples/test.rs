#![feature(proc_macro_hygiene)]

use domsl::jsx;

fn main() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let node = jsx!(document => {
        <p>"Hello"</p>
    });
    let s1 = "hello";
    let s2 = String::from("hello");
    let v = vec![
        jsx!(document => { <p>"p1"</p> }),
        jsx!(document => { <p>"p2"</p> }),
    ];

    jsx!(document => {
        <div class="baz">
            { node }
            <div>{ s1 }</div>
            <div>{ s2 }</div>
            <div>{ v }</div>
            <div>{ vec!["hi"] }</div>
            <div>{ vec![1, 2, 3] }</div>
        </div>
    });
}
