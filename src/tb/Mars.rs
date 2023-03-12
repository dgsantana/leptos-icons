#[cfg(feature = "TbMars")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbMars")]
/// *This icon requires the feature* `TbMars` *to be enabled*.
#[component]
pub fn Mars(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-mars" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M10 14m-5 0a5 5 0 1 0 10 0a5 5 0 1 0 -10 0" /><path d="M19 5l-5.4 5.4" /><path d="M19 5l-5 0" /><path d="M19 5l0 5" /></svg>
   }
}