#[cfg(feature = "ImGooglePlus3")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImGooglePlus3")]
/// *This icon requires the feature* `ImGooglePlus3` *to be enabled*.
#[component]
pub fn GooglePlus3(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M8 0c-4.419 0-8 3.581-8 8s3.581 8 8 8 8-3.581 8-8-3.581-8-8-8zM6 12c-2.212 0-4-1.787-4-4s1.788-4 4-4c1.081 0 1.984 0.394 2.681 1.047l-1.088 1.044c-0.297-0.284-0.816-0.616-1.594-0.616-1.366 0-2.481 1.131-2.481 2.525s1.116 2.525 2.481 2.525c1.584 0 2.178-1.137 2.269-1.725h-2.269v-1.372h3.778c0.034 0.2 0.063 0.4 0.063 0.663 0 2.287-1.531 3.909-3.841 3.909zM13 8v1h-1v-1h-1v-1h1v-1h1v1h1v1h-1z" /></svg>
   }
}