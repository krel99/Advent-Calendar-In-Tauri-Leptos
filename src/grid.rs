use js_sys::Math;
use leptos::html::Canvas;
use leptos::wasm_bindgen::JsCast;
use leptos::*;
use web_sys;

#[derive(Debug, Clone)]
struct BoxState {
    is_open: RwSignal<bool>,
}

fn random_range(min: f64, max: f64) -> f64 {
    min + Math::random() * (max - min)
}

fn draw_snowflake(
    context: &web_sys::CanvasRenderingContext2d,
    x: f64,
    y: f64,
    base_size: f64,
    day: i32
) {
    context.save();
    context.translate(x, y).unwrap();

    // Randomize parameters based on day as seed
    let arms = if day == 1 { 8 } else { 6 + ((Math::random() * 4.0) as i32) };
    let size = base_size * (0.8 + Math::random() * 0.4); // Vary size by ±20%
    let branch_pos = 0.3 + Math::random() * 0.3; // Branch position varies between 30-60% of arm
    let branch_size = size * (0.15 + Math::random() * 0.15); // Branch size varies

    for i in 0..arms {
        context.begin_path();
        context.move_to(0.0, 0.0);
        context.line_to(size, 0.0);

        // Main branches
        context.move_to(size * branch_pos, 0.0);
        context.line_to(size * branch_pos + branch_size, branch_size);
        context.move_to(size * branch_pos, 0.0);
        context.line_to(size * branch_pos + branch_size, -branch_size);

        // Optional extra branches
        if Math::random() > 0.5 {
            let small_branch_pos = size * (branch_pos + 0.2);
            let small_branch_size = branch_size * 0.6;
            context.move_to(small_branch_pos, 0.0);
            context.line_to(small_branch_pos + small_branch_size, small_branch_size * 0.7);
            context.move_to(small_branch_pos, 0.0);
            context.line_to(small_branch_pos + small_branch_size, -small_branch_size * 0.7);
        }

        context.stroke();
        context.rotate((2.0 * std::f64::consts::PI) / (arms as f64)).unwrap();
    }

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
                                canvas_elem.set_width(125);
                                canvas_elem.set_height(125);

                                // Clear canvas
                                context.clear_rect(0.0, 0.0, 125.0, 125.0);

                                // Semi-transparent background
                                context.set_fill_style(&"rgba(135, 206, 235, 0.7)".into());
                                context.fill_rect(0.0, 0.0, 125.0, 125.0);

                                // Snowflake style
                                context.set_stroke_style(&"#FFFFFF".into());
                                context.set_line_width(2.0);

                                match day {
                                    1 => {
                                        // Special pattern for day 1
                                        draw_snowflake(&context, 62.5, 62.5, 30.0, 1);
                                        draw_snowflake(&context, 31.25, 31.25, 15.0, 1);
                                        draw_snowflake(&context, 93.75, 93.75, 15.0, 1);
                                        draw_snowflake(&context, 31.25, 93.75, 15.0, 1);
                                        draw_snowflake(&context, 93.75, 31.25, 15.0, 1);
                                    },
                                    _ => {
                                        // Random sizes and positions for other days
                                        let main_size = 25.0 + random_range(-5.0, 5.0);
                                        let main_x = 62.5 + random_range(-10.0, 10.0);
                                        let main_y = 62.5 + random_range(-10.0, 10.0);
                                        draw_snowflake(&context, main_x, main_y, main_size, day);

                                        // 50% chance for a second smaller snowflake
                                        if Math::random() > 0.5 {
                                            let small_x = random_range(30.0, 95.0);
                                            let small_y = random_range(30.0, 95.0);
                                            draw_snowflake(&context, small_x, small_y, main_size * 0.6, day);
                                        }

                                        // 25% chance for a second smaller snowflake
                                        if Math::random() > 0.75 {
                                            let small_x = random_range(20.0, 105.0);
                                            let small_y = random_range(20.0, 105.0);
                                            draw_snowflake(&context, small_x, small_y, main_size * 0.6, day);
                                        }
                                        // 10% chance for a second smaller snowflake
                                        if Math::random() > 0.9 {
                                            let small_x = random_range(10.0, 115.0);
                                            let small_y = random_range(10.0, 115.0);
                                            draw_snowflake(&context, small_x, small_y, main_size * 0.6, day);
                                        }
                                        // 3% chance for a second smaller snowflake
                                        if Math::random() > 0.97 {
                                            let small_x = random_range(3.0, 122.0);
                                            let small_y = random_range(3.0, 122.0);
                                            draw_snowflake(&context, small_x, small_y, main_size * 0.6, day);
                                        }
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
