#[cfg(feature = "IoBatteryFull")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoBatteryFull")]
/// *This icon requires the feature* `IoBatteryFull` *to be enabled*.
#[component]
pub fn BatteryFull(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><rect x="32" y="144" width="400" height="224" rx="45.7" ry="45.7" style="fill:none;stroke:#000;stroke-linecap:square;stroke-miterlimit:10;stroke-width:32px" /><rect x="85.69" y="198.93" width="292.63" height="114.14" rx="4" ry="4" style="stroke:#000;stroke-linecap:square;stroke-miterlimit:10;stroke-width:32px" /><line x1="480" y1="218.67" x2="480" y2="293.33" style="fill:none;stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px" /></svg>
   }
}