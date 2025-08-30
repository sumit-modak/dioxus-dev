use dioxus::logger::tracing::info;
use dioxus::prelude::*;
// use dioxus_primitives::accordion::*;

#[component]
pub fn MyAccordion() -> Element {
    rsx! {
        // Accordion {
        //     allow_multiple_open: false,
        //     horizontal: false,
        //     for i in 0..4 {
        //         AccordionItem {
        //             index: i,
        //             on_change: move |open| {
        //                 info!("{open};");
        //             },
        //             on_trigger_click: move || {
        //                 info!("trigger");
        //             },
        //             AccordionTrigger {
        //                 "the quick brown fox"
        //             }
        //             AccordionContent {
        //                 div { padding_bottom: "1rem",
        //                     p {
        //                         padding: "0",
        //                         "Jumped over the lazy dog."
        //                     }
        //                 }
        //             }
        //         }
        //     }
        // }
        // Accordion {
        //     id: Some("myaccordion".to_string()),
        //     class: Some("accordion".to_string()), // Apply CSS class
        //     AccordionItem {
        //         value: "item-1", // Unique identifier for this item
        //         trigger: rsx! { div { class: "accordion-trigger", "Section 1" } },
        //         content: rsx! { div { class: "accordion-content", "Content for section 1." } },
        //     }
        //     AccordionItem {
        //         value: "item-2",
        //         trigger: rsx! { div { class: "accordion-trigger", "Section 2" } },
        //         content: rsx! { div { class: "accordion-content", "Content for section 2." } },
        //     }
        //     AccordionItem {
        //         value: "item-3",
        //         trigger: rsx! { div { class: "accordion-trigger", "Section 3" } },
        //         content: rsx! { div { class: "accordion-content", "Content for section 3." } },
        //     }
        // }
    }
}
