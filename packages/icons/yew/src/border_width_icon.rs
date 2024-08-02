use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct BorderWidthIconProps {
    #[prop_or(AttrValue::from("currentColor"))]
    pub color: AttrValue,
}
#[function_component(UseNodeRef)]
pub fn BorderWidthIcon(props: &BorderWidthIconProps) -> Html {
    let node_ref = use_node_ref();
    html! {
        <svg
            ref={node_ref}
            width="15"
            height="15"
            viewBox="0 0 15 15"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path
                fill-rule="evenodd"
                clip-rule="evenodd"
                d="M1 3H14V4H1V3ZM1 6H14V8H1V6ZM14 10.25H1V12.75H14V10.25Z"
                fill={& props.color}
            />
        </svg>
    }
}
