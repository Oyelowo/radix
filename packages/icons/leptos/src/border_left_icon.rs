use leptos::{svg::Svg, *};
#[component]
pub fn BorderLeftIcon(
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
            <path
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M1.75 1L1.75 14L0.249999 14L0.25 1L1.75 1Z"
                fill=color.clone()
            />
            <rect
                x="10"
                y="7"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 10 7)"
                fill=color.clone()
            />
            <rect
                x="10"
                y="13"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 10 13)"
                fill=color.clone()
            />
            <rect
                x="12"
                y="7"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 12 7)"
                fill=color.clone()
            />
            <rect
                x="12"
                y="13"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 12 13)"
                fill=color.clone()
            />
            <rect
                x="8"
                y="7"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 8 7)"
                fill=color.clone()
            />
            <rect
                x="14"
                y="7"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 14 7)"
                fill=color.clone()
            />
            <rect
                x="8"
                y="13"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 8 13)"
                fill=color.clone()
            />
            <rect
                x="14"
                y="13"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 14 13)"
                fill=color.clone()
            />
            <rect
                x="8"
                y="5"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 8 5)"
                fill=color.clone()
            />
            <rect
                x="14"
                y="5"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 14 5)"
                fill=color.clone()
            />
            <rect
                x="8"
                y="3"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 8 3)"
                fill=color.clone()
            />
            <rect
                x="14"
                y="3"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 14 3)"
                fill=color.clone()
            />
            <rect
                x="8"
                y="9"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 8 9)"
                fill=color.clone()
            />
            <rect
                x="14"
                y="9"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 14 9)"
                fill=color.clone()
            />
            <rect
                x="8"
                y="11"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 8 11)"
                fill=color.clone()
            />
            <rect
                x="14"
                y="11"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 14 11)"
                fill=color.clone()
            />
            <rect
                x="6"
                y="7"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 6 7)"
                fill=color.clone()
            />
            <rect
                x="6"
                y="13"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 6 13)"
                fill=color.clone()
            />
            <rect
                x="4"
                y="7"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 4 7)"
                fill=color.clone()
            />
            <rect
                x="4"
                y="13"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 4 13)"
                fill=color.clone()
            />
            <rect
                x="10"
                y="1"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 10 1)"
                fill=color.clone()
            />
            <rect
                x="12"
                y="1"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 12 1)"
                fill=color.clone()
            />
            <rect
                x="8"
                y="1"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 8 1)"
                fill=color.clone()
            />
            <rect
                x="14"
                y="1"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 14 1)"
                fill=color.clone()
            />
            <rect
                x="6"
                y="1"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 6 1)"
                fill=color.clone()
            />
            <rect
                x="4"
                y="1"
                width="1"
                height="1"
                rx=".5"
                transform="rotate(90 4 1)"
                fill=color.clone()
            />
        </svg>
    }
}
