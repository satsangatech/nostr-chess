use gloo::events::EventListener;
use web_sys::{wasm_bindgen::JsCast, Element, KeyboardEvent, MouseEvent};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ModalProps {
    pub children: Children,
    pub is_open: UseStateHandle<bool>,
}

#[function_component(Modal)]
pub fn left_modal(props: &ModalProps) -> Html {
    let is_open = props.is_open.clone();
    let modal_ref = use_node_ref();

    let close = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(false))
    };

    let modal_class = if *is_open {
        "fixed top-0 left-0 w-screen h-screen flex justify-center items-center z-70 transition-transform duration-300 ease-in-out scale-100 pointer-events-none"
    } else {
        "fixed top-0 left-0 w-screen h-screen flex justify-center items-center z-70 transform transition-transform duration-300 ease-in-out scale-0 pointer-events-none"
    };

    let overlay_class = if *is_open {
        "fixed top-0 left-0 w-screen h-screen z-60 bg-black/50 transition-opacity duration-300 opacity-100"
    } else {
        "fixed top-0 left-0 w-screen h-screen z-60 bg-black/50 transition-opacity duration-300 pointer-events-none opacity-0"
    };

    {
        let modal_ref = modal_ref.clone();

        use_effect_with(is_open, move |is_open| {
            let document = gloo::utils::document();

            // Set up click outside listener
            //
            let is_open_clone = is_open.clone();
            let click_callback = Callback::from(move |event: MouseEvent| {
                if *is_open_clone {
                    if let Some(modal_element) = modal_ref.cast::<Element>() {
                        let target = event.target().unwrap();
                        let target_element = target.dyn_ref::<Element>().unwrap();

                        if !modal_element.contains(Some(target_element)) {
                            is_open_clone.set(false);
                        }
                    }
                }
            });

            let click_listener = EventListener::new(&document, "mousedown", move |event| {
                click_callback.emit(event.dyn_ref::<MouseEvent>().unwrap().clone());
            });

            // Set up keydown listener
            let is_open = is_open.clone();
            let keydown_callback = Callback::from(move |event: KeyboardEvent| {
                if *is_open && event.key() == "Escape" {
                    is_open.set(false);
                }
            });

            let keydown_listener = EventListener::new(&document, "keydown", move |event| {
                keydown_callback.emit(event.dyn_ref::<KeyboardEvent>().unwrap().clone());
            });

            move || {
                drop(click_listener);
                drop(keydown_listener);
            }
        });
    }

    html! {
        <>
            // Overlay
            <div onclick={close}
                class={overlay_class}
                aria-hidden="true"
            />

            // Drawer
            <div
                ref={modal_ref}
                class={modal_class}
                aria-label="Drawer"
            >
                <div class="max-w-sm h-fit max-h-sm mx-auto pointer-events-auto">
                { for props.children.iter() }
                </div>
            </div>
        </>
    }
}


