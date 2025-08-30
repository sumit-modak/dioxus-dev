use dioxus::prelude::*;

struct User {
    id: String,
    name: String,
}

impl User {
    fn new(s: &str) -> Self {
        Self {
            id: s.to_string(),
            name: s.to_string(),
        }
    }
}

#[component]
pub fn Iter() -> Element {
    let users = [User::new("feno"), User::new("godl"), User::new("agog")];
    rsx! {
        // iterating users using for loop
        ul {
            for user in users {
                li {
                    key: "{user.id}",
                    "{user.name}"
                }
            }
        }

        // And iterators
        ul {
            {(0..5).map(|i| rsx! { "{i}" br { } })}
        }
    }
}
