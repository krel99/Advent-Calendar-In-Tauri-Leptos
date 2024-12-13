use leptos::*;
use leptos::html::Canvas;
use leptos::wasm_bindgen::JsCast;
use web_sys;

#[derive(Debug, Clone)]
struct BoxState {
    is_open: RwSignal<bool>,
}

fn draw_snowflake(context: &web_sys::CanvasRenderingContext2d, x: f64, y: f64, size: f64) {
    // Save the current context state
    context.save();

    // Move to the center point
    context.translate(x, y).unwrap();

    // Draw 8 lines from center
    for i in 0..8 {
        context.begin_path();
        context.move_to(0.0, 0.0);
        context.line_to(size, 0.0);

        // Add little branches
        context.move_to(size * 0.5, 0.0);
        context.line_to(size * 0.7, size * 0.2);
        context.move_to(size * 0.5, 0.0);
        context.line_to(size * 0.7, -size * 0.2);

        context.stroke();

        // Rotate for next line
        context.rotate(std::f64::consts::PI / 4.0).unwrap();
    }

    // Restore the context state
    context.restore();
}

#[component]
pub fn Grid() -> impl IntoView {
    view! {
        <div class="grid">
            {(1..=24).into_iter().map(|day| {
                let state = BoxState {
                    is_open: create_rw_signal(false)
                };
                
                let node_ref = create_node_ref::<Canvas>();
                
                let on_click = move |_| {
                    if let Some(canvas) = node_ref.get() {
                        let canvas_elem: &web_sys::HtmlCanvasElement = canvas.as_ref();
                        
                        if let Ok(Some(context)) = canvas_elem.get_context("2d") {
                            if let Ok(context) = context.dyn_into::<web_sys::CanvasRenderingContext2d>() {
                                // Set canvas size
                                canvas_elem.set_width(125);
                                canvas_elem.set_height(125);
                                
                                // Fill background
                                context.set_fill_style(&"#87CEEB".into()); // Sky blue
                                context.fill_rect(0.0, 0.0, 125.0, 125.0);
                                
                                // Set up snowflake style
                                context.set_stroke_style(&"#FFFFFF".into());
                                context.set_line_width(2.0);
                                
                                match day {
                                    1 => {
                                        // Draw center snowflake
                                        draw_snowflake(&context, 62.5, 62.5, 30.0);
                                        // Draw smaller corner snowflakes
                                        draw_snowflake(&context, 31.25, 31.25, 15.0);
                                        draw_snowflake(&context, 93.75, 93.75, 15.0);
                                        draw_snowflake(&context, 31.25, 93.75, 15.0);
                                        draw_snowflake(&context, 93.75, 31.25, 15.0);
                                    },
                                    _ => {
                                        // For now, just draw a single snowflake
                                        draw_snowflake(&context, 62.5, 62.5, 30.0);
                                    }
                                }
                            }
                        }
                    }
                    state.is_open.set(true);
                };

                view! {
                    <div class="relative box">
                        <canvas _ref=node_ref class="black-border">
                            "Your browser does not support canvas"
                        </canvas>
                        <Show 
                            when=move || !state.is_open.get()
                            fallback=|| view! { <span></span> }
                        >
                            <span class="absolute top-2 left-2 day">{day}</span>
                            <div 
                                class="absolute inset-0 bg-transitional cursor-pointer"
                                on:click=on_click
                            >
                            </div>
                        </Show>
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}
