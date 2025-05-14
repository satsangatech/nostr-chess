use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SelectContext<T: 'static + Clone + PartialEq + std::fmt::Display> {
    pub selected: Option<T>,
    pub is_open: bool,
    pub toggle: Callback<()>,
    pub select_value: Callback<T>,
    pub close: Callback<()>,
}

#[derive(Properties, PartialEq)]
pub struct SelectProviderProps<T: 'static + Clone + PartialEq + std::fmt::Display> {
    pub children: Children,
    pub context: SelectContext<T>,
}

#[function_component(SelectProvider)]
pub fn select_provider<T: 'static + Clone + PartialEq + std::fmt::Display>(
    props: &SelectProviderProps<T>,
) -> Html {
    html! {
        <ContextProvider<SelectContext<T>> context={props.context.clone()}>
            { for props.children.iter() }
        </ContextProvider<SelectContext<T>>>
    }
}

#[derive(Properties, PartialEq)]
pub struct SelectProps<T: 'static + Clone + PartialEq + std::fmt::Display> {
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub required: bool,
    #[prop_or(Callback::noop())]
    pub onchange: Callback<Option<T>>,
}

#[function_component(Select)]
pub fn select<T: 'static + Clone + PartialEq + std::fmt::Display>(props: &SelectProps<T>) -> Html {
    let selected = use_state(|| None);
    let is_open = use_state(|| false);
    let listener_ref = use_mut_ref(|| None); // 👈 Add this line

    let toggle = {
        let is_open = is_open.clone();
        Callback::from(move |()| is_open.set(!*is_open))
    };

    let close = {
        let is_open = is_open.clone();
        Callback::from(move |()| is_open.set(false))
    };

    let select_value = {
        let selected = selected.clone();
        let is_open = is_open.clone();
        let onchange = props.onchange.clone();
        Callback::from(move |value: T| {
            onchange.emit(Some(value.clone()));
            selected.set(Some(value));
            is_open.set(false);
        })
    };

    {
        use_effect_with(is_open.clone(), move |is_open| {
            if **is_open {
                let document = web_sys::window().unwrap().document().unwrap();
                let is_open = is_open.clone();
                let listener = gloo::events::EventListener::new(&document, "click", move |_| {
                    is_open.set(false);
                });

                *listener_ref.borrow_mut() = Some(listener); // 👈 Keep listener alive
            } else {
                *listener_ref.borrow_mut() = None; // 👈 Drop listener when closed
            }

            || {}
        });
    }

    let context = SelectContext {
        selected: (*selected).clone(),
        is_open: *is_open,
        toggle,
        select_value,
        close,
    };

    let value = selected.as_ref().map_or_else(
        || "Select an option".to_string(),
        std::string::ToString::to_string,
    );

    html! {
        <div class="relative inline-block min-w-fit w-full">
            <input
                type="hidden"
                name={props.name.clone()}
                id={props.id.clone().unwrap_or_else(|| props.name.clone())}
                disabled={props.disabled}
                required={props.required}
                {value}
            />

            <SelectProvider<T> context={context}>
                { props.children.clone() }
            </SelectProvider<T>>
        </div>
    }
}

#[derive(Properties, PartialEq, Eq, Clone)]
pub struct SelectTriggerProps {
    #[prop_or("Select an option".to_string())]
    pub label: String,
    #[prop_or(classes!("w-full", "px-3", "py-2", "border", "rounded-md", "text-sm", "bg-background", "text-foreground", "border-input", "shadow-sm", "focus:outline-none", "focus:ring-2", "focus:ring-ring", "focus:ring-offset-2", "focus:ring-offset-background", "disabled:cursor-not-allowed", "disabled:opacity-50"))]
    pub class: Classes,
    #[prop_or_default]
    pub value: Option<String>,
}

#[function_component(SelectTrigger)]
pub fn select_trigger<T>(props: &SelectTriggerProps) -> Html
where
    T: 'static + Clone + PartialEq + std::fmt::Display,
{
    let ctx = use_context::<SelectContext<T>>().expect("SelectTrigger must be used inside Select");
    let SelectTriggerProps {
        label,
        class,
        value,
    } = (*props).clone();
    let onclick = {
        let toggle = ctx.toggle.clone();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            toggle.emit(());
        })
    };

    let inner_label = value.unwrap_or_else(|| ctx.selected.map_or(label, |v| v.to_string()));

    html! {
        <button type="button" onclick={onclick} {class}>
            { inner_label }
        </button>
    }
}

#[function_component(SelectContent)]
pub fn select_content<T: 'static + Clone + PartialEq + std::fmt::Display>(
    props: &yew::html::ChildrenProps,
) -> Html {
    let ctx = use_context::<SelectContext<T>>().expect("SelectContent must be used inside Select");

    if !ctx.is_open {
        return html! {};
    }

    let onclick = Callback::from(|e: MouseEvent| e.stop_propagation());

    html! {
        <div
            onclick={onclick}
            class="absolute z-20 mt-1 w-full rounded-md border bg-background text-foreground shadow-md animate-in fade-in-0 slide-in-from-top-1 max-h-60 overflow-y-auto"
        >
            { props.children.clone() }
        </div>
    }
}

#[derive(Properties, PartialEq, Eq, Clone)]
pub struct SelectItemProps<T>
where
    T: 'static + Clone + PartialEq,
{
    pub value: T,
    #[prop_or_default]
    pub label: Option<String>,
}

#[function_component(SelectItem)]
pub fn select_item<T>(props: &SelectItemProps<T>) -> Html
where
    T: 'static + Clone + PartialEq + std::fmt::Display,
{
    let ctx = use_context::<SelectContext<T>>().expect("SelectItem must be used inside Select");

    let onclick = {
        let value = props.value.clone();
        let select_value = ctx.select_value.clone();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            select_value.emit(value.clone());
        })
    };

    let is_selected = ctx.selected.as_ref() == Some(&props.value);
    let item_classes = if is_selected {
        classes!("bg-accent", "text-accent-foreground")
    } else {
        classes!("hover:bg-muted", "hover:text-muted-foreground")
    };

    let content = props
        .label
        .clone()
        .unwrap_or_else(|| props.value.to_string());

    html! {
        <div
            onclick={onclick}
            class={classes!("px-3", "py-2", "text-sm", "cursor-pointer", "select-none", "rounded-sm", item_classes)}
        >
            {
                if is_selected {
                    html! { <strong>{ content }</strong> }
                } else {
                    html! { <span>{ content }</span> }
                }
            }
        </div>
    }
}
