use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct KeyboardIconProps {
    #[prop_or(AttrValue::from("currentColor"))]
    pub color: AttrValue,
}
#[function_component(UseNodeRef)]
pub fn KeyboardIcon(props: &KeyboardIconProps) -> Html {
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
                d="M13.5 4H1.5C1.22386 4 1 4.22386 1 4.5V10.5C1 10.7761 1.22386 11 1.5 11H13.5C13.7761 11 14 10.7761 14 10.5V4.5C14 4.22386 13.7761 4 13.5 4ZM1.5 3C0.671573 3 0 3.67157 0 4.5V10.5C0 11.3284 0.671573 12 1.5 12H13.5C14.3284 12 15 11.3284 15 10.5V4.5C15 3.67157 14.3284 3 13.5 3H1.5ZM2 5H3V6H2V5ZM5 5H4V6H5V5ZM6 5H7V6H6V5ZM9 5H8V6H9V5ZM10 5H11V6H10V5ZM13 5H12V6H13V5ZM11 7H12V8H11V7ZM13 9H12V10H13V9ZM9 7H10V8H9V7ZM8 7H7V8H8V7ZM5 7H6V8H5V7ZM4 7H3V8H4V7ZM2 9H3V10H2V9ZM11 9H4V10H11V9Z"
                fill={& props.color}
            />
        </svg>
    }
}
