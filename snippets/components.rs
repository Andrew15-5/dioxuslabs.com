//! Encapsulate state in components.
//!
//! Use structs or autodderive props with `#[component]`

#[derive(Props, PartialEq)]
struct PropBased {
    name: String,
    age: String,
}

fn Stateful(cx: Scope<PropBased>) -> Element {
    render!("Hello {cx.name}, you are {cx.age} years old!")
}
