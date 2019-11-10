#![feature(proc_macro_hygiene)]

use jsx::jsx;

fn main() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    jsx!(document => {
        <div class="baz">
            <br foo={ 3 + 7 } />
        </div>
    });
}
