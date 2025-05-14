use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SliderProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub default_value: Vec<u32>,
    #[prop_or(100)]
    pub max: u32,
    #[prop_or(0)]
    pub min: u32,
    #[prop_or(1)]
    pub step: u32,
    #[prop_or_default]
    pub value: Vec<u32>,
    #[prop_or(Callback::noop())]
    pub on_value_change: Callback<Vec<u32>>,
}

#[function_component(Slider)]
pub fn slider(props: &SliderProps) -> Html {
    let value = if props.value.is_empty() {
        props.default_value.first().copied().unwrap_or(0)
    } else {
        props.value.first().copied().unwrap_or(0)
    };

    let onchange = {
        let on_value_change = props.on_value_change.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Ok(value) = input.value().parse::<u32>() {
                on_value_change.emit(vec![value]);
            }
        })
    };

    let classes = classes!("w-full", props.class.clone());

    html! {
        <div class={classes}>
            <input
                type="range"
                min={props.min.to_string()}
                max={props.max.to_string()}
                step={props.step.to_string()}
                value={value.to_string()}
                onchange={onchange}
                class="w-full h-2 rounded-lg appearance-none cursor-pointer bg-input accent-accent focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 ring-offset-background"
            />
        </div>
    }
}
