use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CardProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    let classes = classes!(
        "rounded-lg",
        "border",
        "bg-background",
        "text-foreground",
        "shadow-sm",
        props.class.clone()
    );

    html! {
        <div class={classes}>
            {props.children.clone()}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CardHeaderProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(CardHeader)]
pub fn card_header(props: &CardHeaderProps) -> Html {
    let classes = classes!(
        "flex",
        "flex-col",
        "space-y-1.5",
        "p-6",
        props.class.clone()
    );

    html! {
        <div class={classes}>
            {props.children.clone()}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CardTitleProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(CardTitle)]
pub fn card_title(props: &CardTitleProps) -> Html {
    let classes = classes!(
        "text-2xl",
        "font-semibold",
        "leading-none",
        "tracking-tight",
        props.class.clone()
    );

    html! {
        <h3 class={classes}>
            {props.children.clone()}
        </h3>
    }
}

#[derive(Properties, PartialEq)]
pub struct CardDescriptionProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(CardDescription)]
pub fn card_description(props: &CardDescriptionProps) -> Html {
    let classes = classes!("text-sm", "text-muted-foreground", props.class.clone());

    html! {
        <p class={classes}>
            {props.children.clone()}
        </p>
    }
}

#[derive(Properties, PartialEq)]
pub struct CardContentProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(CardContent)]
pub fn card_content(props: &CardContentProps) -> Html {
    let classes = classes!("p-6", "pt-0", props.class.clone());

    html! {
        <div class={classes}>
            {props.children.clone()}
        </div>
    }
}
