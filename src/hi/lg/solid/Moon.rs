#[cfg(feature = "HiLgSolidMoon")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "HiLgSolidMoon")]
/// *This icon requires the feature* `HiLgSolidMoon` *to be enabled*.
#[component]
pub fn Moon(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M9.52839 1.71774C9.74339 1.93274 9.80731 2.25628 9.69021 2.53689C9.2458 3.60192 9 4.77131 9 6.00001C9 10.9706 13.0294 15 18 15C19.2287 15 20.3981 14.7542 21.4631 14.3098C21.7437 14.1927 22.0673 14.2566 22.2823 14.4716C22.4973 14.6866 22.5612 15.0102 22.4441 15.2908C20.8618 19.0827 17.1183 21.75 12.75 21.75C6.95101 21.75 2.25 17.049 2.25 11.25C2.25 6.88172 4.91735 3.13817 8.70924 1.55591C8.98985 1.43882 9.31338 1.50274 9.52839 1.71774Z" fill="#0F172A" /></svg>
   }
}