use leptos::*;
use crate::grid::Grid;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="main-container">
            <h1>"Advent Calendar"</h1>
            <Grid />
        </div>
    }
}
