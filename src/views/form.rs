use dioxus::prelude::*;

#[component]
pub fn Form() -> Element {
    rsx! {
        form {
            method: "POST",
            enctype: "multipart/form-data",

            // username
            label { r#for: "username", "username:" }
            input {
                type: "text",
                id: "username",
                required: true,
                minlength: 6,
                maxlength: 14,
                placeholder: "Enter a username",
            }
            br {}

            // password
            label { r#for: "password", "password:" }
            input {
                type: "password",
                id: "password",
                required: true,
                minlength: 8,
                placeholder: "Enter a password",
            }
            br {}

            // email
            label { for: "email", "email:" }
            input { type: "email", id: "email", required: true, placeholder: "johndoe@example.com", }
            br {}

            // phone
            label { r#for: "phone", "phone:" }
            input { type: "tel", id: "phone", placeholder: "123-456-7890", pattern: "[0-9]{10}" }
            br {}

            // birth date
            label { r#for: "dob", "birth date:", }
            input { type: "date", id: "dob", required: true, }
            br {}

            // number
            label { for: "quantity", "quantity", }
            input { type: "number", id: "quantity", min: 0, max: 99, value: 1, } // value = default value
            br {}

            // radio buttons
            label { for: "title", "title:" }
            // ... for mister
            label { for: "Mr.", "Mr." }
            input { type: "radio", id: "Mr.", name: "title", }
            // ... for miss
            label { for: "Ms.", "Ms." }
            input { type: "radio", id: "Ms.", name: "title", }
            // ... for doctor
            label { for: "PhD.", "PhD." }
            input { type: "radio", id: "PhD.", name: "title", }
            br {}

            // dropdown menu
            label { for: "payment", "payment:" }
            select {
                id: "payment",
                option { value: "visa", "visa" }
                option { value: "mastercard", "mastercard" }
                option { value: "giftcard", "giftcard" }
            }
            br {}

            // checkbox
            label { for: "subscribe", "subscribe:" }
            input { type: "checkbox", id: "subscribe", }
            br {}

            // text area
            label { for: "comment", "comment:" }
            textarea { id: "comment", rows: 3, cols: 25, }
            br {}

            // file submission
            label { for: "file", "profile picture:" }
            input { type: "file", id: "file", accept: "image/png, image/jpeg"}
            br {}

            // buttons
            input {
                type: "reset",
            }
            input {
                type: "submit",
            }
        }
    }
}
