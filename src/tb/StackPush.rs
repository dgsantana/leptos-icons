#[cfg(feature = "TbStackPush")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbStackPush")]
/// *This icon requires the feature* `TbStackPush` *to be enabled*.
#[component]
pub fn StackPush(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-stack-push" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M6 10l-2 1l8 4l8 -4l-2 -1" /><path d="M4 15l8 4l8 -4" /><path d="M12 4v7" /><path d="M15 8l-3 3l-3 -3" /></svg>
   }
}