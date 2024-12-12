// Component
use leptos::*;

#[derive(Debug, Clone)]
struct BoxState {
    is_open: RwSignal<bool>,
}

#[component]
pub fn Grid() -> impl IntoView {
    view! {
        <div class="grid">
            {(1..=24).into_iter().map(|day| {


                let state = BoxState {
                    is_open: create_rw_signal(false)
                };
                

                view! {
                    <div class="relative box">

                        <canvas class="black-border">
                            "Your browser does not support canvas"
                        </canvas>
                        <Show 
                            when=move || !state.is_open.get()
                            fallback=|| view! { <span></span> }
                        >
                        <span class="absolute top-2 left-2 day">{day}</span>
                            <div 
                                class="absolute inset-0 bg-transitional cursor-pointer"
                                on:click=move |_| state.is_open.set(true)
                            >
                            </div>
                        </Show>
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
