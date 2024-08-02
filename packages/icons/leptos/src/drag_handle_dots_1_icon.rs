use leptos::{svg::Svg, *};
#[component]
pub fn DragHandleDots1Icon(
    #[prop(default = "currentColor".into(), into)] color: MaybeSignal<String>,
    #[prop(optional)] node_ref: NodeRef<Svg>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    view! {
        <svg
            {..attrs}
            node_ref=node_ref
            width="15"
            height="15"
            viewBox="0 0 15 15"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        >
            <circle cx="4.5" cy="2.5" r=".6" fill=color.clone() />
            <circle cx="4.5" cy="4.5" r=".6" fill=color.clone() />
            <circle cx="4.5" cy="6.499" r=".6" fill=color.clone() />
            <circle cx="4.5" cy="8.499" r=".6" fill=color.clone() />
            <circle cx="4.5" cy="10.498" r=".6" fill=color.clone() />
            <circle cx="4.5" cy="12.498" r=".6" fill=color.clone() />
            <circle cx="6.5" cy="2.5" r=".6" fill=color.clone() />
            <circle cx="6.5" cy="4.5" r=".6" fill=color.clone() />
            <circle cx="6.5" cy="6.499" r=".6" fill=color.clone() />
            <circle cx="6.5" cy="8.499" r=".6" fill=color.clone() />
            <circle cx="6.5" cy="10.498" r=".6" fill=color.clone() />
            <circle cx="6.5" cy="12.498" r=".6" fill=color.clone() />
            <circle cx="8.499" cy="2.5" r=".6" fill=color.clone() />
            <circle cx="8.499" cy="4.5" r=".6" fill=color.clone() />
            <circle cx="8.499" cy="6.499" r=".6" fill=color.clone() />
            <circle cx="8.499" cy="8.499" r=".6" fill=color.clone() />
            <circle cx="8.499" cy="10.498" r=".6" fill=color.clone() />
            <circle cx="8.499" cy="12.498" r=".6" fill=color.clone() />
            <circle cx="10.499" cy="2.5" r=".6" fill=color.clone() />
            <circle cx="10.499" cy="4.5" r=".6" fill=color.clone() />
            <circle cx="10.499" cy="6.499" r=".6" fill=color.clone() />
            <circle cx="10.499" cy="8.499" r=".6" fill=color.clone() />
            <circle cx="10.499" cy="10.498" r=".6" fill=color.clone() />
            <circle cx="10.499" cy="12.498" r=".6" fill=color.clone() />
        </svg>
    }
}
