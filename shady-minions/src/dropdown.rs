use gloo::events::EventListener;
use web_sys::{wasm_bindgen::JsCast, MouseEvent};
use yew::prelude::*;

// Shared context for the dropdown state
#[derive(Clone, PartialEq)]
pub struct DropdownState {
    is_open: bool,
    set_is_open: Callback<bool>,
}

#[derive(Properties, PartialEq)]
pub struct DropdownMenuProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(DropdownMenu)]
pub fn dropdown_menu(props: &DropdownMenuProps) -> Html {
    let is_open = use_state(|| false);

    let set_is_open = {
        let is_open = is_open.clone();
        Callback::from(move |value: bool| {
            is_open.set(value);
        })
    };

    let state = DropdownState {
        is_open: *is_open,
        set_is_open,
    };

    let context = use_memo(state, std::clone::Clone::clone);

    html! {
        <ContextProvider<DropdownState> context={(*context).clone()}>
            <div class="relative inline-block text-left">
                {props.children.clone()}
            </div>
        </ContextProvider<DropdownState>>
    }
}

#[derive(Properties, PartialEq)]
pub struct DropdownMenuTriggerProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub aria_label: Option<String>,
}

#[function_component(DropdownMenuTrigger)]
pub fn dropdown_menu_trigger(props: &DropdownMenuTriggerProps) -> Html {
    let dropdown_context = use_context::<DropdownState>()
        .expect("DropdownMenuTrigger must be used within a DropdownMenu");

    let onclick = {
        let set_is_open = dropdown_context.set_is_open;
        let is_open = dropdown_context.is_open;
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            set_is_open.emit(!is_open);
        })
    };

    let classes = classes!(
        "inline-flex",
        "items-center",
        "justify-center",
        "rounded-md",
        "text-sm",
        "font-medium",
        "transition-colors",
        "focus-visible:outline-none",
        "focus-visible:ring-2",
        "focus-visible:ring-ring",
        "focus-visible:ring-offset-2",
        "disabled:opacity-50",
        "disabled:pointer-events-none",
        "ring-offset-background",
        "hover:bg-accent",
        "hover:text-accent-foreground",
        "h-10",
        "py-2",
        "px-4",
        props.class.clone()
    );

    html! {
        <button
            type="button"
            class={classes}
            onclick={onclick}
            aria-expanded={dropdown_context.is_open.to_string()}
            aria-haspopup="true"
            disabled={props.disabled}
            aria-label={props.aria_label.clone()}
            data-state={if dropdown_context.is_open { "open" } else { "closed" }}
        >
            {props.children.clone()}
        </button>
    }
}

#[derive(Properties, PartialEq)]
pub struct DropdownMenuContentProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or(String::from("bottom"))]
    pub side: String,
    #[prop_or(String::from("center"))]
    pub align: String,
}

#[function_component(DropdownMenuContent)]
pub fn dropdown_menu_content(props: &DropdownMenuContentProps) -> Html {
    let dropdown_context = use_context::<DropdownState>()
        .expect("DropdownMenuContent must be used within a DropdownMenu");

    let node_ref = use_node_ref();

    // Handle click outside to close dropdown
    {
        let node_ref = node_ref.clone();
        let set_is_open = dropdown_context.set_is_open.clone();

        use_effect_with((), move |()| {
            let document = web_sys::window().unwrap().document().unwrap();

            let listener = EventListener::new(&document, "mousedown", move |event| {
                let event = event.unchecked_ref::<MouseEvent>();
                if let Some(element) = node_ref.cast::<web_sys::Element>() {
                    if !element.contains(Some(&event.target_dyn_into::<web_sys::Node>().unwrap())) {
                        set_is_open.emit(false);
                    }
                }
            });

            move || {
                drop(listener);
            }
        });
    }

    // Calculate position classes based on side and align props
    let position_classes = match (props.side.as_str(), props.align.as_str()) {
        ("bottom", "center") => "top-full left-1/2 -translate-x-1/2 mt-2",
        ("bottom", "end") => "top-full right-0 mt-2",
        ("top", "start") => "bottom-full left-0 mb-2",
        ("top", "center") => "bottom-full left-1/2 -translate-x-1/2 mb-2",
        ("top", "end") => "bottom-full right-0 mb-2",
        ("left", "start") => "right-full top-0 mr-2",
        ("left", "center") => "right-full top-1/2 -translate-y-1/2 mr-2",
        ("left", "end") => "right-full bottom-0 mr-2",
        ("right", "start") => "left-full top-0 ml-2",
        ("right", "center") => "left-full top-1/2 -translate-y-1/2 ml-2",
        ("right", "end") => "left-full bottom-0 ml-2",
        _ => "top-full left-0 mt-2", // Default
    };

    let classes = classes!(
        "z-50",
        "min-w-[8rem]",
        "overflow-hidden",
        "rounded-md",
        "border",
        "border-border",
        "bg-popover",
        "p-1",
        "text-popover-foreground",
        "shadow-md",
        "data-[state=open]:animate-in",
        "data-[state=closed]:animate-out",
        "data-[state=closed]:fade-out-0",
        "data-[state=open]:fade-in-0",
        "data-[state=closed]:zoom-out-95",
        "data-[state=open]:zoom-in-95",
        position_classes,
        props.class.clone()
    );

    if !dropdown_context.is_open {
        return html! {};
    }

    html! {
        <div
            ref={node_ref}
            class={classes}
            data-state={if dropdown_context.is_open { "open" } else { "closed" }}
            role="menu"
            aria-orientation="vertical"
        >
            {props.children.clone()}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct DropdownMenuItemProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub inset: bool,
}

#[function_component(DropdownMenuItem)]
pub fn dropdown_menu_item(props: &DropdownMenuItemProps) -> Html {
    let dropdown_context = use_context::<DropdownState>()
        .expect("DropdownMenuItem must be used within a DropdownMenu");

    let onclick = {
        let user_onclick = props.onclick.clone();
        let set_is_open = dropdown_context.set_is_open;
        let disabled = props.disabled;

        Callback::from(move |e: MouseEvent| {
            if !disabled {
                user_onclick.emit(e);
                set_is_open.emit(false);
            }
        })
    };

    let inset_class = if props.inset { "pl-8" } else { "" };

    let classes = classes!(
        "relative",
        "flex",
        "cursor-default",
        "select-none",
        "items-center",
        "rounded-sm",
        "px-2",
        "py-1.5",
        "text-sm",
        "outline-none",
        "transition-colors",
        "focus:bg-accent",
        "focus:text-accent-foreground",
        "hover:bg-accent",
        "hover:text-accent-foreground",
        "data-[disabled]:pointer-events-none",
        "data-[disabled]:opacity-50",
        inset_class,
        props.class.clone()
    );

    html! {
        <div
            class={classes}
            onclick={onclick}
            role="menuitem"
            tabindex="0"
            data-disabled={props.disabled.to_string()}
        >
            {props.children.clone()}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct DropdownMenuLabelProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub inset: bool,
}

#[function_component(DropdownMenuLabel)]
pub fn dropdown_menu_label(props: &DropdownMenuLabelProps) -> Html {
    let inset_class = if props.inset { "pl-8" } else { "" };

    let classes = classes!(
        "px-2",
        "py-1.5",
        "text-sm",
        "font-semibold",
        inset_class,
        props.class.clone()
    );

    html! {
        <div class={classes}>
            {props.children.clone()}
        </div>
    }
}

#[derive(Properties, PartialEq, Eq)]
pub struct DropdownMenuSeparatorProps {
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(DropdownMenuSeparator)]
pub fn dropdown_menu_separator(props: &DropdownMenuSeparatorProps) -> Html {
    let classes = classes!("my-1", "h-px", "bg-border", props.class.clone());

    html! {
        <div class={classes} role="separator" />
    }
}
