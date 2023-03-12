#[cfg(feature = "BsRecordCircleFill")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BsRecordCircleFill")]
/// *This icon requires the feature* `BsRecordCircleFill` *to be enabled*.
#[component]
pub fn RecordCircleFill(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-record-circle-fill" viewBox="0 0 16 16"><path d="M16 8A8 8 0 1 1 0 8a8 8 0 0 1 16 0zm-8 3a3 3 0 1 0 0-6 3 3 0 0 0 0 6z" /></svg>
   }
}