#[cfg(feature = "BiRegularDownArrowCircle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularDownArrowCircle")]
/// *This icon requires the feature* `BiRegularDownArrowCircle` *to be enabled*.
#[component]
pub fn DownArrowCircle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 1.993C6.486 1.994 2 6.48 2 11.994c0 5.513 4.486 9.999 10 10 5.514 0 10-4.486 10-10s-4.485-10-10-10.001zm0 18.001c-4.411-.001-8-3.59-8-8 0-4.411 3.589-8 8-8.001 4.411.001 8 3.59 8 8.001s-3.589 8-8 8z" /><path d="M13 8h-2v4H7.991l4.005 4.005L16 12h-3z" /></svg>
   }
}