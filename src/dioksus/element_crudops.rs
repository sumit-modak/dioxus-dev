use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(Clone, PartialEq)]
struct Todo {
    id: u32,
    text: String,
    completed: bool,
}

#[component]
pub fn TodoFn() -> Element {
    // State for todos (stored as a HashMap for easy CRUD operations)
    let mut todolist = use_signal(|| HashMap::<u32, Todo>::new());
    let mut next_id = use_signal(|| 1u32);
    let mut new_todo_text = use_signal(|| String::new());
    let mut editing_id = use_signal(|| None::<u32>);
    let mut edit_text = use_signal(|| String::new());

    // Create: Handle form submission to add a new todo
    let on_submit = move |event: FormEvent| {
        let text = new_todo_text.read().trim().to_string();
        if !text.is_empty() {
            let mut todos = todolist.write();
            todos.insert(
                *next_id.read(),
                Todo {
                    id: *next_id.read(),
                    text,
                    completed: false,
                },
            );
            next_id += 1;
            new_todo_text.set(String::new());
        }
    };

    // Update: Handle toggle completion
    let mut on_toggle = move |id: u32| {
        let mut todos = todolist.write();
        if let Some(todo) = todos.get_mut(&id) {
            todo.completed = !todo.completed;
        }
    };

    // Update: Start editing a todo
    let mut on_start_edit = move |(id, text): (u32, String)| {
        editing_id.set(Some(id));
        edit_text.set(text);
    };

    // Update: Save edited todo
    let mut on_save_edit = use_callback(move |id: u32| {
        let text = edit_text.read().trim().to_string();
        if !text.is_empty() {
            let mut todos = todolist.write();
            if let Some(todo) = todos.get_mut(&id) {
                std::mem::replace(&mut todo.text, text);
            }
            editing_id.set(None);
            edit_text.set(String::new());
        }
    });

    // Delete: Remove a todo
    let mut on_delete = move |id: u32| {
        let mut todos = todolist.write();
        todos.remove(&id);
    };

    rsx! {
        div {
            h1 { "Todo List" }
            // Create: Input form for new todos
            form {
                onsubmit: on_submit,
                input {
                    value: "{new_todo_text}",
                    oninput: move |event| new_todo_text.set(event.value()),
                    placeholder: "Enter a new todo",
                }
                button { r#type: "submit", "Add Todo" }
            }
            // Read: Display todos
            ul {
                for (id, todo) in todolist.read().clone().into_iter() {
                    li {
                        key: "{todo.id}",
                        // Update: Show input field if editing this todo
                        if *editing_id.read() == Some(id) {
                            input {
                                value: "{edit_text}",
                                oninput: move |event| edit_text.set(event.value()),
                            }
                            button {
                                onclick: move |_| on_save_edit(id),
                                "Save"
                            }
                        } else {
                            // Read: Display todo text and completion status
                            span {
                                style: if todo.completed { "text-decoration: line-through" } else { "" },
                                "{todo.text}"
                            }
                            // Update: Toggle completion
                            button {
                                onclick: move |_| on_toggle(id),
                                if todo.completed { "Undo" } else { "Complete" }
                            }
                            // Update: Start editing
                            button {
                                onclick: move |_| on_start_edit((id, todo.text.clone())),
                                "Edit"
                            }
                            // Delete: Remove todo
                            button {
                                onclick: move |_| on_delete(id),
                                "Delete"
                            }
                        }
                    }
                }
            }
        }
    }
}
