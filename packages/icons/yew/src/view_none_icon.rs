use yew::prelude::*;
#[derive(PartialEq, Properties)]
pub struct ViewNoneIconProps {
    #[prop_or(AttrValue::from("currentColor"))]
    pub color: AttrValue,
}
#[function_component(UseNodeRef)]
pub fn ViewNoneIcon(props: &ViewNoneIconProps) -> Html {
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
                d="M14 2.58711L1.85163 13H13.5C13.7761 13 14 12.7761 14 12.5V2.58711ZM0.762879 13.8067L0.825396 13.8796L0.854717 13.8545C1.05017 13.9478 1.26899 14 1.5 14H13.5C14.3284 14 15 13.3284 15 12.5V2.5C15 1.93949 14.6926 1.45078 14.2371 1.19331L14.1746 1.12037L14.1453 1.1455C13.9498 1.05222 13.731 1 13.5 1H1.5C0.671573 1 0 1.67157 0 2.5V12.5C0 13.0605 0.307435 13.5492 0.762879 13.8067ZM1 12.4129L13.1484 2H1.5C1.22386 2 1 2.22386 1 2.5V12.4129Z"
                fill={& props.color}
            />
        </svg>
    }
}
