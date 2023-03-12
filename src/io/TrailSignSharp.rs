#[cfg(feature = "IoTrailSignSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoTrailSignSharp")]
/// *This icon requires the feature* `IoTrailSignSharp` *to be enabled*.
#[component]
pub fn TrailSignSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M502.63,336l-80-80H278V224H448V64H278V32H234V64H89.37l-80,80,80,80H234v32H64V416H234v64h44V416H422.63Z" /></svg>
   }
}