use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main class="min-h-screen bg-background p-4 md:p-8">
            <div class="max-w-6xl mx-auto space-y-8">
                <header class="space-y-2">
                    <h1 class="text-3xl font-bold tracking-tight">{"Crypto Orderbook"}</h1>
                    <p class="text-muted-foreground">{"A minimalist interface for crypto asset swaps"}</p>
                </header>

                <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
                    <div class="lg:col-span-2">
                        <OrderBook />
                    </div>
                    <div>
                        <PlaceOrderForm />
                    </div>
                </div>
            </div>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

use shady_minions::ui::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use shady_minions::ui::{Tabs, TabsContent, TabsList, TabsTrigger};

// Types for our orderbook
#[derive(Clone, PartialEq, Debug)]
pub enum OrderType {
    Buy,
    Sell,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Order {
    id: String,
    price: f64,
    amount: f64,
    total: f64,
    order_type: OrderType,
}

#[function_component(OrderBook)]
pub fn order_book() -> Html {
    let buy_orders = use_state(Vec::new);
    let sell_orders = use_state(Vec::new);
    let spread = use_state(|| 0.0);
    let last_price = use_state(|| None::<f64>);
    let price_change = use_state(|| None::<String>);

    // Generate mock data on component mount
    {
        let buy_orders = buy_orders.clone();
        let sell_orders = sell_orders.clone();
        let spread = spread.clone();
        let last_price = last_price.clone();
        let price_change = price_change.clone();

        use_effect_with(
            (), // Empty dependencies array means this effect runs once on mount
            move |_| {
                // Generate random buy orders (higher prices are better for buyers)
                let mut mock_buy_orders = (0..10)
                    .map(|i| {
                        let price = 29000.0
                            - (i as f64) * 15.0
                            - (web_sys::js_sys::Math::random() * 10.0).floor();
                        let amount = 0.1 + web_sys::js_sys::Math::random() * 2.0;
                        let amount = (amount * 10000.0).round() / 10000.0; // Round to 4 decimal places

                        Order {
                            id: format!("buy-{}", i),
                            price,
                            amount,
                            total: (price * amount * 100.0).round() / 100.0, // Round to 2 decimal places
                            order_type: OrderType::Buy,
                        }
                    })
                    .collect::<Vec<_>>();

                // Sort by price descending
                mock_buy_orders.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
                buy_orders.set(mock_buy_orders);

                // Generate random sell orders (lower prices are better for sellers)
                let mut mock_sell_orders = (0..10)
                    .map(|i| {
                        let price = 29100.0
                            + (i as f64) * 15.0
                            + (web_sys::js_sys::Math::random() * 10.0).floor();
                        let amount = 0.1 + web_sys::js_sys::Math::random() * 2.0;
                        let amount = (amount * 10000.0).round() / 10000.0; // Round to 4 decimal places

                        Order {
                            id: format!("sell-{}", i),
                            price,
                            amount,
                            total: (price * amount * 100.0).round() / 100.0, // Round to 2 decimal places
                            order_type: OrderType::Sell,
                        }
                    })
                    .collect::<Vec<_>>();

                // Sort by price ascending
                mock_sell_orders.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
                sell_orders.set(mock_sell_orders);

                // Calculate spread and last price
                let buy_orders_val = buy_orders.clone();
                let sell_orders_val = sell_orders.clone();

                if !(*buy_orders_val).is_empty() && !(*sell_orders_val).is_empty() {
                    let highest_bid = (*buy_orders_val)[0].price;
                    let lowest_ask = (*sell_orders_val)[0].price;
                    spread.set(lowest_ask - highest_bid);

                    let new_price = ((highest_bid + lowest_ask) / 2.0).floor();

                    if let Some(prev) = *last_price {
                        if new_price > prev {
                            price_change.set(Some("up".to_string()));
                        } else {
                            price_change.set(Some("down".to_string()));
                        }
                    }

                    last_price.set(Some(new_price));
                }

                // Cleanup function to clear interval
                || {}
            },
        );
    }

    // Recalculate spread and last price when orders change
    {
        let buy_orders = buy_orders.clone();
        let sell_orders = sell_orders.clone();
        let spread = spread.clone();
        let last_price = last_price.clone();
        let price_change = price_change.clone();

        use_effect_with(((*buy_orders).clone(), (*sell_orders).clone()), move |_| {
            if !(*buy_orders).is_empty() && !(*sell_orders).is_empty() {
                let highest_bid = (*buy_orders)[0].price;
                let lowest_ask = (*sell_orders)[0].price;
                spread.set(lowest_ask - highest_bid);

                let new_price = ((highest_bid + lowest_ask) / 2.0).floor();

                if let Some(prev) = *last_price {
                    if new_price > prev {
                        price_change.set(Some("up".to_string()));
                    } else {
                        price_change.set(Some("down".to_string()));
                    }
                }

                last_price.set(Some(new_price));
            }

            || {}
        });
    }

    html! {
        <Card class="h-full">
            <CardHeader class="pb-3">
                <div class="flex justify-between items-center">
                    <div>
                        <CardTitle>{"BTC/USDT"}</CardTitle>
                        <CardDescription>{"Order Book"}</CardDescription>
                    </div>
                    {
                        if let Some(price) = *last_price {
                            html! {
                                <div class="text-right">
                                    <div class="flex items-center">
                                        <span class="text-2xl font-bold">{format!("${}", price.round() as i32)}</span>
                                        {
                                            if let Some(change) = (*price_change).clone() {
                                                if change == "up" {
                                                    html! {
                                                        <span class="ml-2">
                                                            <svg class="h-4 w-4 text-green-500" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                                                <path d="m18 15-6-6-6 6"/>
                                                            </svg>
                                                        </span>
                                                    }
                                                } else {
                                                    html! {
                                                        <span class="ml-2">
                                                            <svg class="h-4 w-4 text-red-500" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                                                <path d="m6 9 6 6 6-6"/>
                                                            </svg>
                                                        </span>
                                                    }
                                                }
                                            } else {
                                                html! {}
                                            }
                                        }
                                    </div>
                                    <div class="text-sm text-muted-foreground">
                                        {format!("Spread: ${:.2} ({:.3}%)", *spread, (*spread / price) * 100.0)}
                                    </div>
                                </div>
                            }
                        } else {
                            html! {}
                        }
                    }
                </div>
            </CardHeader>
            <CardContent>
                <Tabs default_value="combined" class="w-full">
                    <TabsList class="grid w-full grid-cols-3">
                        <TabsTrigger value="combined">{"Combined"}</TabsTrigger>
                        <TabsTrigger value="buys">{"Buys"}</TabsTrigger>
                        <TabsTrigger value="sells">{"Sells"}</TabsTrigger>
                    </TabsList>

                    <TabsContent value="combined" class="space-y-4">
                        <div class="mt-4">
                            // Sell orders (reversed to show highest at bottom)
                            <div class="mb-4">
                                <div class="grid grid-cols-3 text-sm font-medium text-muted-foreground mb-2">
                                    <div>{"Price (USDT)"}</div>
                                    <div class="text-right">{"Amount (BTC)"}</div>
                                    <div class="text-right">{"Total (USDT)"}</div>
                                </div>
                                <div class="space-y-1">
                                    {
                                        (*sell_orders).clone().into_iter().rev().map(|order| {
                                            let id = order.id.clone();
                                            html! {
                                                <div key={id} class="grid grid-cols-3 text-sm py-1 border-b border-border/40">
                                                    <div class="text-red-500">{format!("${}", order.price.round() as i32)}</div>
                                                    <div class="text-right">{format!("{:.4}", order.amount)}</div>
                                                    <div class="text-right">{format!("${:.2}", order.total)}</div>
                                                </div>
                                            }
                                        }).collect::<Html>()
                                    }
                                </div>
                            </div>

                            // Spread indicator
                            <div class="py-2 px-3 bg-muted/50 rounded-md text-center text-sm my-2">
                                {format!("Spread: ${:.2}", *spread)}
                            </div>

                            // Buy orders
                            <div class="mt-4">
                                <div class="space-y-1">
                                    {
                                        (*buy_orders).clone().into_iter().map(|order| {
                                            let id = order.id.clone();
                                            html! {
                                                <div key={id} class="grid grid-cols-3 text-sm py-1 border-b border-border/40">
                                                    <div class="text-green-500">{format!("${}", order.price.round() as i32)}</div>
                                                    <div class="text-right">{format!("{:.4}", order.amount)}</div>
                                                    <div class="text-right">{format!("${:.2}", order.total)}</div>
                                                </div>
                                            }
                                        }).collect::<Html>()
                                    }
                                </div>
                            </div>
                        </div>
                    </TabsContent>

                    <TabsContent value="buys">
                        <div class="mt-4">
                            <div class="grid grid-cols-3 text-sm font-medium text-muted-foreground mb-2">
                                <div>{"Price (USDT)"}</div>
                                <div class="text-right">{"Amount (BTC)"}</div>
                                <div class="text-right">{"Total (USDT)"}</div>
                            </div>
                            <div class="space-y-1">
                                {
                                    (*buy_orders).clone().into_iter().map(|order| {
                                        let id = order.id.clone();
                                        html! {
                                            <div key={id} class="grid grid-cols-3 text-sm py-1 border-b border-border/40">
                                                <div class="text-green-500">{format!("${}", order.price.round() as i32)}</div>
                                                <div class="text-right">{format!("{:.4}", order.amount)}</div>
                                                <div class="text-right">{format!("${:.2}", order.total)}</div>
                                            </div>
                                        }
                                    }).collect::<Html>()
                                }
                            </div>
                        </div>
                    </TabsContent>

                    <TabsContent value="sells">
                        <div class="mt-4">
                            <div class="grid grid-cols-3 text-sm font-medium text-muted-foreground mb-2">
                                <div>{"Price (USDT)"}</div>
                                <div class="text-right">{"Amount (BTC)"}</div>
                                <div class="text-right">{"Total (USDT)"}</div>
                            </div>
                            <div class="space-y-1">
                                {
                                    (*sell_orders).clone().into_iter().map(|order| {
                                        let id = order.id.clone();
                                        html! {
                                            <div key={id} class="grid grid-cols-3 text-sm py-1 border-b border-border/40">
                                                <div class="text-red-500">{format!("${}", order.price.round() as i32)}</div>
                                                <div class="text-right">{format!("{:.4}", order.amount)}</div>
                                                <div class="text-right">{format!("${:.2}", order.total)}</div>
                                            </div>
                                        }
                                    }).collect::<Html>()
                                }
                            </div>
                        </div>
                    </TabsContent>
                </Tabs>
            </CardContent>
        </Card>
    }
}

use shady_minions::ui::Button;
use shady_minions::ui::Input;
use shady_minions::ui::Label;
use shady_minions::ui::Slider;
use web_sys::SubmitEvent;

#[function_component(PlaceOrderForm)]
pub fn place_order_form() -> Html {
    let order_type = use_state(|| "buy".to_string());
    let price = use_state(|| 29000.0);
    let amount = use_state(|| 0.1);
    let percentage = use_state(|| 0);
    let switch_clicked = use_state(|| false);

    let total = *price * *amount;

    let handle_submit = {
        let order_type = order_type.clone();
        let price = price.clone();
        let amount = amount.clone();
        let total = total;

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let message = format!(
                "Order placed: {} {} BTC at {} for a total of {:.2}",
                (*order_type).to_uppercase(),
                *amount,
                *price,
                total
            );
            web_sys::window()
                .and_then(|win| win.alert_with_message(&message).ok())
                .unwrap_or(());
        })
    };

    let handle_percentage_change = {
        let percentage = percentage.clone();
        let amount = amount.clone();

        Callback::from(move |value: Vec<u32>| {
            if let Some(value) = value.first() {
                percentage.set(*value);
                // In a real app, this would calculate the amount based on available balance
                amount.set(0.1 + (*value as f64 / 100.0) * 2.0);
            }
        })
    };

    let handle_order_type_change = {
        let order_type = order_type.clone();

        Callback::from(move |value: String| {
            order_type.set(value);
        })
    };

    let handle_price_change = {
        let price = price.clone();

        Callback::from(move |value: String| {
            if let Ok(value) = value.parse::<f64>() {
                price.set(value);
            }
        })
    };

    let handle_amount_change = {
        let amount = amount.clone();

        Callback::from(move |value: String| {
            if let Ok(value) = value.parse::<f64>() {
                amount.set(value);
            }
        })
    };

    let handle_percentage_button = {
        let handle_percentage_change = handle_percentage_change.clone();

        Callback::from(move |value: u32| {
            handle_percentage_change.emit(vec![value]);
        })
    };

    let switch_clicked_onchange = {
        let switch_clicked = switch_clicked.clone();

        Callback::from(move |_| {
            switch_clicked.set(!*switch_clicked);
        })
    };

    html! {
        <Card>
            <CardHeader>
                <CardTitle>{"Place Order"}</CardTitle>
                <CardDescription>{"Swap BTC for USDT or vice versa"}</CardDescription>
            </CardHeader>
            <CardContent>
                <Tabs default_value="buy" on_value_change={handle_order_type_change} class="w-full">
                    <TabsList class="grid w-full grid-cols-2">
                        <TabsTrigger value="buy">{"Buy BTC"}</TabsTrigger>
                        <TabsTrigger value="sell">{"Sell BTC"}</TabsTrigger>
                    </TabsList>

                    <form onsubmit={handle_submit}>
                        <div class="space-y-4 mt-4">
                            <div class="space-y-2">
                                <Label r#for="price">{"Price (USDT)"}</Label>
                                <Input
                                    id="price"
                                    r#type={shady_minions::ui::InputType::Number}
                                    min="1"
                                    step="1"
                                    value={(*price).to_string()}
                                    onchange={handle_price_change}
                                    class="w-full"
                                />
                            </div>

                            <div class="space-y-2">
                                <Label r#for="amount">{"Amount (BTC)"}</Label>
                                <Input
                                    id="amount"
                                    r#type={shady_minions::ui::InputType::Number}
                                    min="0.0001"
                                    step="0.0001"
                                    value={(*amount).to_string()}
                                    onchange={&handle_amount_change}
                                    class="w-full"
                                />
                            </div>

                            <div class="space-y-2">
                                <Label r#for="amount">{"Notes"}</Label>
                                <shady_minions::ui::TextArea
                                    id="amount"
                                    value={(*amount).to_string()}
                                    onchange={&handle_amount_change}
                                    class="w-full"
                                />
                            </div>

                            <div class="space-y-2">
                                <div class="flex justify-between">
                                    <Label>{"Percentage"}</Label>
                                    <span class="text-sm text-muted-foreground">{format!("{}%", *percentage)}</span>
                                </div>
                                <Slider
                                    default_value={vec![0]}
                                    max={100}
                                    step={1}
                                    value={vec![*percentage]}
                                    on_value_change={handle_percentage_change}
                                />
                                    <shady_minions::ui::Switch
                                        class="text-xs h-7"
                                        checked={*switch_clicked}
                                        onchange={switch_clicked_onchange}
                                        />
                                <div class="flex justify-between mt-1">
                                    <Button
                                        r#type={shady_minions::ui::ButtonType::Button}
                                        variant={shady_minions::ui::ButtonVariant::Outline}
                                        size={shady_minions::ui::ButtonSize::Small}
                                        onclick={
                                            let handle_percentage_button = handle_percentage_button.clone();
                                            Callback::from(move |_| handle_percentage_button.emit(25))
                                        }
                                        class="text-xs h-7"
                                    >
                                        {"25%"}
                                    </Button>
                                    <Button
                                        r#type={shady_minions::ui::ButtonType::Button}
                                        variant={shady_minions::ui::ButtonVariant::Outline}
                                        size={shady_minions::ui::ButtonSize::Small}
                                        onclick={
                                            let handle_percentage_button = handle_percentage_button.clone();
                                            Callback::from(move |_| handle_percentage_button.emit(50))
                                        }
                                        class="text-xs h-7"
                                    >
                                        {"50%"}
                                    </Button>
                                    <Button
                                        r#type={shady_minions::ui::ButtonType::Button}
                                        variant={shady_minions::ui::ButtonVariant::Outline}
                                        size={shady_minions::ui::ButtonSize::Small}
                                        onclick={
                                            let handle_percentage_button = handle_percentage_button.clone();
                                            Callback::from(move |_| handle_percentage_button.emit(75))
                                        }
                                        class="text-xs h-7"
                                    >
                                        {"75%"}
                                    </Button>
                                    <Button
                                        r#type={shady_minions::ui::ButtonType::Button}
                                        variant={shady_minions::ui::ButtonVariant::Outline}
                                        size={shady_minions::ui::ButtonSize::Small}
                                        onclick={
                                            let handle_percentage_button = handle_percentage_button.clone();
                                            Callback::from(move |_| handle_percentage_button.emit(100))
                                        }
                                        class="text-xs h-7"
                                    >
                                        {"100%"}
                                    </Button>
                                </div>
                            </div>

                            <div class="pt-2 border-t">
                                <div class="flex justify-between text-sm">
                                    <span>{"Total:"}</span>
                                    <span class="font-medium">{format!("${:.2}", total)}</span>
                                </div>
                            </div>
                        </div>

                        <Button
                            r#type={shady_minions::ui::ButtonType::Submit}
                            class="w-full mt-6"
                            variant={if *order_type == "buy" { shady_minions::ui::ButtonVariant::Normal } else { shady_minions::ui::ButtonVariant::Destructive }}
                        >
                            {if *order_type == "buy" { "Buy BTC" } else { "Sell BTC" }}
                        </Button>
                    </form>
                </Tabs>
            </CardContent>
        </Card>
    }
}
