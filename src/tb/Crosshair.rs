#[cfg(feature = "TbCrosshair")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbCrosshair")]
/// *This icon requires the feature* `TbCrosshair` *to be enabled*.
#[component]
pub fn Crosshair(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-crosshair" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M4 8v-2a2 2 0 0 1 2 -2h2" /><path d="M4 16v2a2 2 0 0 0 2 2h2" /><path d="M16 4h2a2 2 0 0 1 2 2v2" /><path d="M16 20h2a2 2 0 0 0 2 -2v-2" /><path d="M9 12l6 0" /><path d="M12 9l0 6" /></svg>
   }
}