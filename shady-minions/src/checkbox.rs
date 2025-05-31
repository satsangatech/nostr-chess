use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct CheckboxProps {
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub checked: bool,
    #[prop_or(false)]
    pub disabled: bool,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub name: Option<String>,
    #[prop_or_default]
    pub value: Option<String>,
    #[prop_or_default]
    pub onchange: Callback<bool>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}
#[function_component(Checkbox)]
pub fn checkbox(props: &CheckboxProps) -> Html {
    let CheckboxProps {
        id,
        checked,
        disabled,
        required,
        name,
        value,
        onchange,
        children,
        class,
    } = props.clone();

    let checkbox_ref = use_node_ref();

    let onchange_handler = {
        let checkbox_ref = checkbox_ref.clone();
        Callback::from(move |_e: Event| {
            if let Some(input) = checkbox_ref.cast::<web_sys::HtmlInputElement>() {
                onchange.emit(input.checked());
            }
        })
    };

    html! {
        <div class={classes!("flex", "items-center", "space-x-2", class)}>
            <input
                ref={checkbox_ref.clone()}
                type="checkbox"
                id={id.clone()}
                checked={checked}
                disabled={disabled}
                required={required}
                name={name}
                value={value.unwrap_or_default()}
                class="peer hidden"
                onchange={onchange_handler}
            />
            <label
                for={id.clone()}
                class={classes!(
                    "h-4",
                    "w-4",
                    "rounded-sm",
                    "border",
                    "border-input",
                    "bg-background",
                    "flex",
                    "items-center",
                    "justify-center",
                    "cursor-pointer",
                    "disabled:cursor-not-allowed",
                    "disabled:opacity-50",
                    "peer-checked:bg-primary",
                    "peer-checked:text-primary-foreground"
                )}
            >
                {
                    if checked {
                        html! {
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                class="h-3 w-3"
                            >
                                <polyline points="20 6 9 17 4 12" />
                            </svg>
                        }
                    } else {
                        html! {}
                    }
                }
            </label>
            {
                if children.is_empty() {
                    html! {}
                } else {
                    html! {
                        <label
                            for={id.clone()}
                            class={classes!(
                                "text-sm",
                                "font-medium",
                                "leading-none",
                                "cursor-pointer",
                                "text-foreground",
                                "peer-disabled:cursor-not-allowed",
                                "peer-disabled:opacity-70"
                            )}
                        >
                            { children }
                        </label>
                    }
                }
            }
        </div>
    }
}
