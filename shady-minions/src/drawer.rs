use gloo::events::EventListener;
use web_sys::{wasm_bindgen::JsCast, Element, KeyboardEvent, MouseEvent};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct LeftDrawerProps {
    pub children: Children,
    pub is_open: UseStateHandle<bool>,
}

#[function_component(LeftDrawer)]
pub fn left_drawer(props: &LeftDrawerProps) -> Html {
    let is_open = props.is_open.clone();
    let drawer_ref = use_node_ref();

    let close = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(false))
    };

    let mut drawer_class = classes!(
        "fixed",
        "left-0",
        "top-0",
        "z-50",
        "h-full",
        "w-fit",
        "max-w-3/4",
        "md:max-w-1/3",
        "transform",
        "bg-background",
        "shadow-lg",
        "transition-transform",
        "duration-300",
        "ease-in-out"
    );
    let mut overlay_class = classes!(
        "fixed",
        "inset-0",
        "z-40",
        "bg-black/50",
        "transition-opacity",
        "duration-300"
    );
    if *is_open {
        drawer_class.push("translate-x-0");
        overlay_class.push("opacity-100");
    } else {
        drawer_class.push("-translate-x-full");
        overlay_class.push("pointer-events-none");
        overlay_class.push("opacity-0");
    }

    {
        let drawer_ref = drawer_ref.clone();

        use_effect_with(is_open, move |is_open| {
            let document = web_sys::Document::new().expect("Document should be available");

            // Set up click outside listener
            //
            let is_open_clone = is_open.clone();
            let click_callback = Callback::from(move |event: MouseEvent| {
                if *is_open_clone {
                    if let Some(drawer_element) = drawer_ref.cast::<Element>() {
                        let target = event.target().unwrap();
                        let target_element = target.dyn_ref::<Element>().unwrap();

                        if !drawer_element.contains(Some(target_element)) {
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
                ref={drawer_ref}
                class={drawer_class}
                aria-label="Drawer"
            >
                <div class="flex h-full flex-col">
                    <div class="flex-1 overflow-auto p-4">
                        { for props.children.iter() }
                    </div>
                </div>
            </div>
        </>
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct BottomDrawerProps {
    pub children: Children,
    pub is_open: UseStateHandle<bool>,
}

#[function_component(BottomDrawer)]
pub fn left_drawer(props: &BottomDrawerProps) -> Html {
    let is_open = props.is_open.clone();
    let drawer_ref = use_node_ref();

    let close = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(false))
    };

    let mut drawer_class = classes!(
        "fixed",
        "left-0",
        "bottom-0",
        "z-50",
        "w-full",
        "h-fit",
        "rounded-t-xl",
        "bg-white",
        "transform",
        "transition-transform",
        "duration-300",
        "ease-in-out"
    );
    let mut overlay_class = classes!(
        "fixed",
        "inset-0",
        "z-40",
        "bg-black/50",
        "transition-opacity",
        "duration-300"
    );
    if *is_open {
        drawer_class.push("translate-y-0");
        overlay_class.push("opacity-100");
    } else {
        drawer_class.push("translate-y-full");
        overlay_class.push("pointer-events-none");
    }

    {
        let drawer_ref = drawer_ref.clone();

        use_effect_with(is_open, move |is_open| {
            let document = web_sys::Document::new().expect("Document should be available");

            // Set up click outside listener
            //
            let is_open_clone = is_open.clone();
            let click_callback = Callback::from(move |event: MouseEvent| {
                if *is_open_clone {
                    if let Some(drawer_element) = drawer_ref.cast::<Element>() {
                        let target = event.target().unwrap();
                        let target_element = target.dyn_ref::<Element>().unwrap();

                        if !drawer_element.contains(Some(target_element)) {
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
                ref={drawer_ref}
                class={drawer_class}
                aria-label="Drawer"
            >
                { for props.children.iter() }
            </div>
        </>
    }
}
