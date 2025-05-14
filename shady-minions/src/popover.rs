use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct PopoverContext {
    pub is_open: bool,
    pub toggle: Callback<()>,
    pub close: Callback<()>,
}

#[derive(Properties, PartialEq)]
pub struct PopoverProviderProps {
    #[prop_or_default]
    pub children: Children,
    pub context: PopoverContext,
}

#[function_component(PopoverProvider)]
pub fn popover_provider(props: &PopoverProviderProps) -> Html {
    html! {
        <ContextProvider<PopoverContext> context={props.context.clone()}>
            { for props.children.iter() }
        </ContextProvider<PopoverContext>>
    }
}

#[function_component(Popover)]
pub fn popover(props: &yew::html::ChildrenProps) -> Html {
    let is_open = use_state(|| false);
    let listener_ref = use_mut_ref(|| None);

    let toggle = {
        let is_open = is_open.clone();
        Callback::from(move |()| is_open.set(!*is_open))
    };

    let close = {
        let is_open = is_open.clone();
        Callback::from(move |()| is_open.set(false))
    };

    {
        use_effect_with(is_open.clone(), move |is_open| {
            if **is_open {
                let document = web_sys::window().unwrap().document().unwrap();
                let is_open = is_open.clone();
                let listener = gloo::events::EventListener::new(&document, "click", move |_| {
                    is_open.set(false);
                });

                // Store listener so it doesn't drop
                *listener_ref.borrow_mut() = Some(listener);
            } else {
                // Drop the existing listener if it's there
                *listener_ref.borrow_mut() = None;
            }

            || {}
        });
    }

    let context = PopoverContext {
        is_open: *is_open,
        toggle,
        close,
    };

    html! {
        <div class="relative inline-block text-left">
            <PopoverProvider context={context}>
                { props.children.clone() }
            </PopoverProvider>
        </div>
    }
}

#[function_component(PopoverTrigger)]
pub fn popover_trigger(props: &yew::html::ChildrenProps) -> Html {
    let ctx = use_context::<PopoverContext>().expect("PopoverTrigger must be used inside Popover");

    let onclick = {
        let toggle = ctx.toggle;
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            toggle.emit(());
        })
    };

    html! {
        <div onclick={onclick} class="cursor-pointer inline-block">
            { props.children.clone() }
        </div>
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum PopoverPosition {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum PopoverAlign {
    #[default]
    Start,
    Center,
    End,
}

#[derive(Properties, PartialEq)]
pub struct PopoverContentProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub position: PopoverPosition,
    #[prop_or_default]
    pub align: PopoverAlign,
}
#[function_component(PopoverContent)]
pub fn popover_content(props: &PopoverContentProps) -> Html {
    let ctx = use_context::<PopoverContext>().expect("PopoverContent must be used inside Popover");

    if !ctx.is_open {
        return html! {};
    }

    let onclick = Callback::from(|e: web_sys::MouseEvent| e.stop_propagation());

    let mut classes_vec = vec![
        "absolute",
        "z-10",
        "bg-background",
        "border",
        "rounded",
        "shadow-lg",
        "w-fit",
    ];

    // Position
    match props.position {
        PopoverPosition::Top => {
            classes_vec.push("bottom-full");
            classes_vec.push("mb-2");
        }
        PopoverPosition::Bottom => {
            classes_vec.push("top-full");
            classes_vec.push("mt-2");
        }
        PopoverPosition::Left => {
            classes_vec.push("right-full");
            classes_vec.push("mr-2");
        }
        PopoverPosition::Right => {
            classes_vec.push("left-full");
            classes_vec.push("ml-2");
        }
    }

    // Alignment
    match (props.position, props.align) {
        (PopoverPosition::Top | PopoverPosition::Bottom, PopoverAlign::Start) => {
            classes_vec.push("left-0");
        }
        (PopoverPosition::Top | PopoverPosition::Bottom, PopoverAlign::Center) => {
            classes_vec.push("left-1/2");
            classes_vec.push("transform");
            classes_vec.push("-translate-x-1/2");
        }
        (PopoverPosition::Top | PopoverPosition::Bottom, PopoverAlign::End) => {
            classes_vec.push("right-0");
        }
        (PopoverPosition::Left | PopoverPosition::Right, PopoverAlign::Start) => {
            classes_vec.push("top-0");
        }
        (PopoverPosition::Left | PopoverPosition::Right, PopoverAlign::Center) => {
            classes_vec.push("top-1/2");
            classes_vec.push("transform");
            classes_vec.push("-translate-y-1/2");
        }
        (PopoverPosition::Left | PopoverPosition::Right, PopoverAlign::End) => {
            classes_vec.push("bottom-0");
        }
    }

    html! {
        <div
            class={classes!(classes_vec, props.class.clone())}
            onclick={onclick}
        >
            <div class="p-4">
                { props.children.clone() }
            </div>
        </div>
    }
}
