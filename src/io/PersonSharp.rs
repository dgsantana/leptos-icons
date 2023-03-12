#[cfg(feature = "IoPersonSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoPersonSharp")]
/// *This icon requires the feature* `IoPersonSharp` *to be enabled*.
#[component]
pub fn PersonSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M256,256A112,112,0,1,0,144,144,112,112,0,0,0,256,256Zm0,32c-69.42,0-208,42.88-208,128v64H464V416C464,330.88,325.42,288,256,288Z" /></svg>
   }
}