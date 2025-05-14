use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TabsProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub default_value: String,
    #[prop_or(Callback::noop())]
    pub on_value_change: Callback<String>,
    #[prop_or_default]
    pub active_tab: Option<UseStateHandle<String>>, // changed to optional
}

#[function_component(Tabs)]
pub fn tabs(props: &TabsProps) -> Html {
    let internal_state = use_state(|| props.default_value.clone());
    let active_tab = props
        .active_tab
        .clone()
        .unwrap_or_else(|| internal_state.clone());

    let set_active_tab = {
        let active_tab = active_tab.clone();
        Callback::from(move |value: String| {
            active_tab.set(value.clone());
        })
    };
    let context = TabsContext {
        active_tab: (*active_tab).clone(),
        set_active_tab,
    };

    html! {
        <div class={classes!("w-full", props.class.clone())}>
            <ContextProvider<TabsContext> context={context}>
                {props.children.clone()}
            </ContextProvider<TabsContext>>
        </div>
    }
}

#[derive(Clone, PartialEq)]
pub struct TabsContext {
    pub active_tab: String,
    pub set_active_tab: Callback<String>,
}

#[derive(Properties, PartialEq)]
pub struct TabsListProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(TabsList)]
pub fn tabs_list(props: &TabsListProps) -> Html {
    let classes = classes!(
        "inline-flex",
        "h-10 ",
        "items-center",
        "justify-center",
        "rounded-md",
        "bg-muted",
        "p-1",
        "text-muted-foreground",
        props.class.clone()
    );

    html! {
        <div class={classes} role="tablist">
            {props.children.clone()}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct TabsTriggerProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    pub value: String,
}

#[function_component(TabsTrigger)]
pub fn tabs_trigger(props: &TabsTriggerProps) -> Html {
    let tabs_ctx = use_context::<TabsContext>().expect("TabsContext not found");
    let is_active = tabs_ctx.active_tab == props.value;

    let onclick = {
        let value = props.value.clone();
        let set_active_tab = tabs_ctx.set_active_tab;
        Callback::from(move |_| set_active_tab.emit(value.clone()))
    };

    let classes = classes!(
        "transition-all",
        "duration-300",
        "ease-in-out",
        "flex-1",
        "inline-flex",
        "items-center",
        "justify-center",
        "whitespace-nowrap",
        "rounded-md",
        "px-3",
        "py-1.5",
        "text-sm",
        "font-medium",
        "ring-offset-background",
        "focus-visible:outline-none",
        "focus-visible:ring-2",
        "focus-visible:ring-ring",
        "focus-visible:ring-offset-2",
        "disabled:pointer-events-none",
        "disabled:opacity-50",
        if is_active {
            "bg-background text-foreground shadow-sm"
        } else {
            "text-muted-foreground hover:text-foreground hover:bg-muted"
        },
        props.class.clone()
    );

    html! {
        <button
            class={classes}
            role="tab"
            aria-selected={is_active.to_string()}
            tabindex={if is_active { "0" } else { "-1" }}
            onclick={onclick}
        >
            {props.children.clone()}
        </button>
    }
}

#[derive(Properties, PartialEq)]
pub struct TabsContentProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
    pub value: String,
}

#[function_component(TabsContent)]
pub fn tabs_content(props: &TabsContentProps) -> Html {
    let tabs_ctx = use_context::<TabsContext>().expect("TabsContext not found");
    let is_active = tabs_ctx.active_tab == props.value;

    let base_classes = classes!(
        "transition-opacity",
        "duration-300",
        "ease-in-out",
        "transform",
        "mt-2",
        "ring-offset-background",
        "focus-visible:outline-none",
        "focus-visible:ring-2",
        "focus-visible:ring-ring",
        "focus-visible:ring-offset-2",
        props.class.clone()
    );

    let dynamic_style = if is_active {
        classes!("opacity-100", "translate-y-0")
    } else {
        classes!("opacity-0", "-translate-y-2", "hidden")
    };

    html! {
        <div
            class={classes!(base_classes, dynamic_style)}
            role="tabpanel"
            tabindex="0"
        >
            {props.children.clone()}
        </div>
    }
}
