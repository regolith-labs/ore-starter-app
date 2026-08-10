use dioxus::prelude::*;

#[component]
pub fn PencilIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                d: "M21.731 2.269a2.625 2.625 0 0 0-3.712 0l-1.157 1.157 3.712 3.712 1.157-1.157a2.625 2.625 0 0 0 0-3.712ZM19.513 8.199l-3.712-3.712-8.4 8.4a5.25 5.25 0 0 0-1.32 2.214l-.8 2.685a.75.75 0 0 0 .933.933l2.685-.8a5.25 5.25 0 0 0 2.214-1.32l8.4-8.4Z"
            }
            path {
                d: "M5.25 5.25a3 3 0 0 0-3 3v10.5a3 3 0 0 0 3 3h10.5a3 3 0 0 0 3-3V13.5a.75.75 0 0 0-1.5 0v5.25a1.5 1.5 0 0 1-1.5 1.5H5.25a1.5 1.5 0 0 1-1.5-1.5V8.25a1.5 1.5 0 0 1 1.5-1.5h5.25a.75.75 0 0 0 0-1.5H5.25Z"
            }
        }
    }
}

#[component]
pub fn ReplyIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            stroke_width: "3",
            stroke: "currentColor",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M9 15L3 9m0 0l6-6M3 9h12a6 6 0 010 12h-3"
            }
        }
    }
}

#[component]
pub fn DropIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 37.607 37.607",
            fill: "currentColor",
            class: "{class}",
            path {
                d: "M31.833,24.579c0,7.187-5.846,13.028-13.029,13.028S5.775,31.764,5.775,24.579c0-6.237,9.105-20.617,10.932-23.437
                    C17.167,0.43,17.957,0,18.806,0s1.638,0.43,2.099,1.142C22.728,3.962,31.833,18.342,31.833,24.579z"
            }
        }
    }
}

#[component]
pub fn OreIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 216 216",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M0.279729 192.083C-0.0932429 191.71 -0.0932429 191.105 0.279729 190.732L28.4516 162.56C28.7938 162.218 28.8414 161.68 28.5687 161.28C18.1262 145.969 12.0208 127.463 12.0208 107.532C12.0208 54.7824 54.7823 12.0209 107.531 12.0209C127.463 12.0209 145.969 18.1262 161.28 28.569C161.68 28.8417 162.218 28.7941 162.56 28.4519L190.732 0.279816C191.105 -0.0932721 191.71 -0.0932721 192.083 0.279816L215.72 23.9178C216.093 24.2908 216.093 24.8953 215.72 25.2683L187.365 53.6242C187.026 53.9626 186.975 54.493 187.239 54.8921C197.227 69.9845 203.042 88.0792 203.042 107.532C203.042 160.281 160.28 203.042 107.531 203.042C88.0788 203.042 69.9844 197.226 54.8921 187.24C54.4929 186.976 53.9625 187.026 53.6241 187.365L25.2681 215.721C24.8952 216.094 24.2904 216.094 23.9174 215.721L0.279729 192.083ZM107.531 167.703C97.5942 167.703 88.2198 165.294 79.96 161.029C69.2678 155.507 60.4434 146.875 54.6844 136.327C50.0141 127.774 47.3597 117.963 47.3597 107.532C47.3597 74.2996 74.2995 47.3598 107.531 47.3598C117.963 47.3598 127.774 50.0144 136.327 54.6845C146.874 60.4431 155.507 69.2685 161.029 79.9603C165.294 88.2205 167.703 97.5943 167.703 107.532C167.703 140.763 140.763 167.703 107.531 167.703Z"
            }
        }
    }
}

#[component]
pub fn SmileIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                d: "M12 2.25c-5.385 0-9.75 4.365-9.75 9.75s4.365 9.75 9.75 9.75 9.75-4.365 9.75-9.75S17.385 2.25 12 2.25Zm-2.625 6c-.54 0-.828.419-.936.634a1.96 1.96 0 0 0-.189.866c0 .298.059.605.189.866.108.215.395.634.936.634.54 0 .828-.419.936-.634.13-.26.189-.568.189-.866 0-.298-.059-.605-.189-.866-.108-.215-.395-.634-.936-.634Zm4.314.634c.108-.215.395-.634.936-.634.54 0 .828.419.936.634.13.26.189.568.189.866 0 .298-.059.605-.189.866-.108.215-.395.634-.936.634-.54 0-.828-.419-.936-.634a1.96 1.96 0 0 1-.189-.866c0-.298.059-.605.189-.866Zm2.023 6.828a.75.75 0 1 0-1.06-1.06 3.75 3.75 0 0 1-5.304 0 .75.75 0 0 0-1.06 1.06 5.25 5.25 0 0 0 7.424 0Z",
                clip_rule: "evenodd",
            }
        }
    }
}

#[component]
pub fn QrCodeIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            fill: "none",
            view_box: "0 0 24 24",
            stroke_width: "1.5",
            stroke: "currentColor",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M3.75 4.875c0-.621.504-1.125 1.125-1.125h4.5c.621 0 1.125.504 1.125 1.125v4.5c0 .621-.504 1.125-1.125 1.125h-4.5A1.125 1.125 0 0 1 3.75 9.375v-4.5ZM3.75 14.625c0-.621.504-1.125 1.125-1.125h4.5c.621 0 1.125.504 1.125 1.125v4.5c0 .621-.504 1.125-1.125 1.125h-4.5a1.125 1.125 0 0 1-1.125-1.125v-4.5ZM13.5 4.875c0-.621.504-1.125 1.125-1.125h4.5c.621 0 1.125.504 1.125 1.125v4.5c0 .621-.504 1.125-1.125 1.125h-4.5A1.125 1.125 0 0 1 13.5 9.375v-4.5Z"
            }
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M6.75 6.75h.75v.75h-.75v-.75ZM6.75 16.5h.75v.75h-.75v-.75ZM16.5 6.75h.75v.75h-.75v-.75ZM13.5 13.5h.75v.75h-.75v-.75ZM13.5 19.5h.75v.75h-.75v-.75ZM19.5 13.5h.75v.75h-.75v-.75ZM19.5 19.5h.75v.75h-.75v-.75ZM16.5 16.5h.75v.75h-.75v-.75Z"
            }
        }
    }
}

#[component]
pub fn SquaresSolidIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M3 6a3 3 0 0 1 3-3h2.25a3 3 0 0 1 3 3v2.25a3 3 0 0 1-3 3H6a3 3 0 0 1-3-3V6Zm9.75 0a3 3 0 0 1 3-3H18a3 3 0 0 1 3 3v2.25a3 3 0 0 1-3 3h-2.25a3 3 0 0 1-3-3V6ZM3 15.75a3 3 0 0 1 3-3h2.25a3 3 0 0 1 3 3V18a3 3 0 0 1-3 3H6a3 3 0 0 1-3-3v-2.25Zm9.75 0a3 3 0 0 1 3-3H18a3 3 0 0 1 3 3V18a3 3 0 0 1-3 3h-2.25a3 3 0 0 1-3-3v-2.25Z"
            }
        }
    }
}

#[component]
pub fn SquaresOutlineIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            class: "{class}",
            fill: "none",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            stroke_width: "1.5",
            stroke: "currentColor",
            path {
                d: "M3.75 6A2.25 2.25 0 0 1 6 3.75h2.25A2.25 2.25 0 0 1 10.5 6v2.25a2.25 2.25 0 0 1-2.25 2.25H6a2.25 2.25 0 0 1-2.25-2.25V6ZM3.75 15.75A2.25 2.25 0 0 1 6 13.5h2.25a2.25 2.25 0 0 1 2.25 2.25V18a2.25 2.25 0 0 1-2.25 2.25H6A2.25 2.25 0 0 1 3.75 18v-2.25ZM13.5 6a2.25 2.25 0 0 1 2.25-2.25H18A2.25 2.25 0 0 1 20.25 6v2.25A2.25 2.25 0 0 1 18 10.5h-2.25a2.25 2.25 0 0 1-2.25-2.25V6ZM13.5 15.75a2.25 2.25 0 0 1 2.25-2.25H18a2.25 2.25 0 0 1 2.25 2.25V18A2.25 2.25 0 0 1 18 20.25h-2.25A2.25 2.25 0 0 1 13.5 18v-2.25Z"
            }
        }
    }
}

#[component]
pub fn SquaresPlusIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                d: "M6 3a3 3 0 0 0-3 3v2.25a3 3 0 0 0 3 3h2.25a3 3 0 0 0 3-3V6a3 3 0 0 0-3-3H6ZM15.75 3a3 3 0 0 0-3 3v2.25a3 3 0 0 0 3 3H18a3 3 0 0 0 3-3V6a3 3 0 0 0-3-3h-2.25ZM6 12.75a3 3 0 0 0-3 3V18a3 3 0 0 0 3 3h2.25a3 3 0 0 0 3-3v-2.25a3 3 0 0 0-3-3H6ZM17.625 13.5a.75.75 0 0 0-1.5 0v2.625H13.5a.75.75 0 0 0 0 1.5h2.625v2.625a.75.75 0 0 0 1.5 0v-2.625h2.625a.75.75 0 0 0 0-1.5h-2.625V13.5Z"
            }
        }
    }
}

#[component]
pub fn SquaresPlusOutlineIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.5",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M13.5 16.875h3.375m0 0h3.375m-3.375 0V13.5m0 3.375v3.375M6 10.5h2.25a2.25 2.25 0 0 0 2.25-2.25V6a2.25 2.25 0 0 0-2.25-2.25H6A2.25 2.25 0 0 0 3.75 6v2.25A2.25 2.25 0 0 0 6 10.5Zm0 9.75h2.25A2.25 2.25 0 0 0 10.5 18v-2.25a2.25 2.25 0 0 0-2.25-2.25H6a2.25 2.25 0 0 0-2.25 2.25V18A2.25 2.25 0 0 0 6 20.25Zm9.75-9.75H18a2.25 2.25 0 0 0 2.25-2.25V6A2.25 2.25 0 0 0 18 3.75h-2.25A2.25 2.25 0 0 0 13.5 6v2.25a2.25 2.25 0 0 0 2.25 2.25Z"
            }
        }
    }
}

#[component]
pub fn SearchIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            fill: "currentColor",
            view_box: "0 0 24 24",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                d: "M10.5 3.75a6.75 6.75 0 1 0 0 13.5 6.75 6.75 0 0 0 0-13.5ZM2.25 10.5a8.25 8.25 0 1 1 14.59 5.28l4.69 4.69a.75.75 0 1 1-1.06 1.06l-4.69-4.69A8.25 8.25 0 0 1 2.25 10.5Z",
                clip_rule: "evenodd"
            }
        }
    }
}

#[component]
pub fn DownloadIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            fill: "none",
            stroke: "currentColor",
            view_box: "0 0 24 24",
            class: "{class}",
            path {
                fill: "none",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: 1.5,
                d: "M3 16.5v2.25A2.25 2.25 0 0 0 5.25 21h13.5A2.25 2.25 0 0 0 21 18.75V16.5M16.5 12 12 16.5m0 0L7.5 12m4.5 4.5V3"
            }
        }
    }
}

#[component]
pub fn CubeIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                d: "M12.378 1.602a.75.75 0 0 0-.756 0L3 6.632l9 5.25 9-5.25-8.622-5.03ZM21.75 7.93l-9 5.25v9l8.628-5.032a.75.75 0 0 0 .372-.648V7.93ZM11.25 22.18v-9l-9-5.25v8.57a.75.75 0 0 0 .372.648l8.628 5.033Z"
            }
        }
    }
}

#[component]
pub fn CubeOutlineIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            fill: "none",
            stroke: "currentColor",
            view_box: "0 0 24 24",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: 1.5,
                d: "m21 7.5-9-5.25L3 7.5m18 0-9 5.25m9-5.25v9l-9 5.25M3 7.5l9 5.25M3 7.5v9l9 5.25m0-9v9"
            }
        }
    }
}

#[component]
pub fn CubeTransparentIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                d: "M11.622 1.602a.75.75 0 0 1 .756 0l2.25 1.313a.75.75 0 0 1-.756 1.295L12 3.118 10.128 4.21a.75.75 0 1 1-.756-1.295l2.25-1.313ZM5.898 5.81a.75.75 0 0 1-.27 1.025l-1.14.665 1.14.665a.75.75 0 1 1-.756 1.295L3.75 8.806v.944a.75.75 0 0 1-1.5 0V7.5a.75.75 0 0 1 .372-.648l2.25-1.312a.75.75 0 0 1 1.026.27Zm12.204 0a.75.75 0 0 1 1.026-.27l2.25 1.312a.75.75 0 0 1 .372.648v2.25a.75.75 0 0 1-1.5 0v-.944l-1.122.654a.75.75 0 1 1-.756-1.295l1.14-.665-1.14-.665a.75.75 0 0 1-.27-1.025Zm-9 5.25a.75.75 0 0 1 1.026-.27L12 11.882l1.872-1.092a.75.75 0 1 1 .756 1.295l-1.878 1.096V15a.75.75 0 0 1-1.5 0v-1.82l-1.878-1.095a.75.75 0 0 1-.27-1.025ZM3 13.5a.75.75 0 0 1 .75.75v1.82l1.878 1.095a.75.75 0 1 1-.756 1.295l-2.25-1.312a.75.75 0 0 1-.372-.648v-2.25A.75.75 0 0 1 3 13.5Zm18 0a.75.75 0 0 1 .75.75v2.25a.75.75 0 0 1-.372.648l-2.25 1.312a.75.75 0 1 1-.756-1.295l1.878-1.096V14.25a.75.75 0 0 1 .75-.75Zm-9 5.25a.75.75 0 0 1 .75.75v.944l1.122-.654a.75.75 0 1 1 .756 1.295l-2.25 1.313a.75.75 0 0 1-.756 0l-2.25-1.313a.75.75 0 1 1 .756-1.295l1.122.654V19.5a.75.75 0 0 1 .75-.75Z",
                clip_rule: "evenodd"
            }
        }
    }
}

#[component]
pub fn BriefcaseIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                d: "M7.5 5.25a3 3 0 0 1 3-3h3a3 3 0 0 1 3 3v.205c.933.085 1.857.197 2.774.334 1.454.218 2.476 1.483 2.476 2.917v3.033c0 1.211-.734 2.352-1.936 2.752A24.726 24.726 0 0 1 12 15.75c-2.73 0-5.357-.442-7.814-1.259-1.202-.4-1.936-1.541-1.936-2.752V8.706c0-1.434 1.022-2.7 2.476-2.917A48.814 48.814 0 0 1 7.5 5.455V5.25Zm7.5 0v.09a49.488 49.488 0 0 0-6 0v-.09a1.5 1.5 0 0 1 1.5-1.5h3a1.5 1.5 0 0 1 1.5 1.5Zm-3 8.25a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5Z",
                clip_rule: "evenodd"
            }
            path {
                d: "M3 18.4v-2.796a4.3 4.3 0 0 0 .713.31A26.226 26.226 0 0 0 12 17.25c2.892 0 5.68-.468 8.287-1.335.252-.084.49-.189.713-.311V18.4c0 1.452-1.047 2.728-2.523 2.923-2.12.282-4.282.427-6.477.427a49.19 49.19 0 0 1-6.477-.427C4.047 21.128 3 19.852 3 18.4Z"
            }
        }
    }
}

#[component]
pub fn BriefcaseOutlineIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            fill: "none",
            stroke: "currentColor",
            view_box: "0 0 24 24",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: 1.5,
                d: "M20.25 14.15v4.25c0 1.094-.787 2.036-1.872 2.18-2.087.277-4.216.42-6.378.42s-4.291-.143-6.378-.42c-1.085-.144-1.872-1.086-1.872-2.18v-4.25m16.5 0a2.18 2.18 0 0 0 .75-1.661V8.706c0-1.081-.768-2.015-1.837-2.175a48.114 48.114 0 0 0-3.413-.387m4.5 8.006c-.194.165-.42.295-.673.38A23.978 23.978 0 0 1 12 15.75c-2.648 0-5.195-.429-7.577-1.22a2.016 2.016 0 0 1-.673-.38m0 0A2.18 2.18 0 0 1 3 12.489V8.706c0-1.081.768-2.015 1.837-2.175a48.111 48.111 0 0 1 3.413-.387m7.5 0V5.25A2.25 2.25 0 0 0 13.5 3h-3a2.25 2.25 0 0 0-2.25 2.25v.894m7.5 0a48.667 48.667 0 0 0-7.5 0M12 12.75h.008v.008H12v-.008Z"
            }
        }
    }
}

#[component]
pub fn CalendarIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                d: "M12.75 12.75a.75.75 0 1 1-1.5 0 .75.75 0 0 1 1.5 0ZM7.5 15.75a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5ZM8.25 17.25a.75.75 0 1 1-1.5 0 .75.75 0 0 1 1.5 0ZM9.75 15.75a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5ZM10.5 17.25a.75.75 0 1 1-1.5 0 .75.75 0 0 1 1.5 0ZM12 15.75a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5ZM12.75 17.25a.75.75 0 1 1-1.5 0 .75.75 0 0 1 1.5 0ZM14.25 15.75a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5ZM15 17.25a.75.75 0 1 1-1.5 0 .75.75 0 0 1 1.5 0ZM16.5 15.75a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5ZM15 12.75a.75.75 0 1 1-1.5 0 .75.75 0 0 1 1.5 0ZM16.5 13.5a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5Z"
            }
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M6.75 2.25A.75.75 0 0 1 7.5 3v1.5h9V3A.75.75 0 0 1 18 3v1.5h.75a3 3 0 0 1 3 3v11.25a3 3 0 0 1-3 3H5.25a3 3 0 0 1-3-3V7.5a3 3 0 0 1 3-3H6V3a.75.75 0 0 1 .75-.75Zm13.5 9a1.5 1.5 0 0 0-1.5-1.5H5.25a1.5 1.5 0 0 0-1.5 1.5v7.5a1.5 1.5 0 0 0 1.5 1.5h13.5a1.5 1.5 0 0 0 1.5-1.5v-7.5Z"
            }
        }
    }
}

#[component]
pub fn CalendarOutlineIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            fill: "none",
            stroke: "currentColor",
            view_box: "0 0 24 24",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: 1.5,
                d: "M6.75 3v2.25M17.25 3v2.25M3 18.75V7.5a2.25 2.25 0 0 1 2.25-2.25h13.5A2.25 2.25 0 0 1 21 7.5v11.25m-18 0A2.25 2.25 0 0 0 5.25 21h13.5A2.25 2.25 0 0 0 21 18.75m-18 0v-7.5A2.25 2.25 0 0 1 5.25 9h13.5A2.25 2.25 0 0 1 21 11.25v7.5m-9-6h.008v.008H12v-.008ZM12 15h.008v.008H12V15Zm0 2.25h.008v.008H12v-.008ZM9.75 15h.008v.008H9.75V15Zm0 2.25h.008v.008H9.75v-.008ZM7.5 15h.008v.008H7.5V15Zm0 2.25h.008v.008H7.5v-.008Zm6.75-4.5h.008v.008h-.008v-.008Zm0 2.25h.008v.008h-.008V15Zm0 2.25h.008v.008h-.008v-.008Zm2.25-4.5h.008v.008H16.5v-.008Zm0 2.25h.008v.008H16.5V15Z"
            }
        }
    }
}

#[component]
pub fn AdjustmentsHorizontalIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            stroke_width: 0,
            path {
                d: "M18.75 12.75h1.5a.75.75 0 0 0 0-1.5h-1.5a.75.75 0 0 0 0 1.5ZM12 6a.75.75 0 0 1 .75-.75h7.5a.75.75 0 0 1 0 1.5h-7.5A.75.75 0 0 1 12 6ZM12 18a.75.75 0 0 1 .75-.75h7.5a.75.75 0 0 1 0 1.5h-7.5A.75.75 0 0 1 12 18ZM3.75 6.75h1.5a.75.75 0 1 0 0-1.5h-1.5a.75.75 0 0 0 0 1.5ZM5.25 18.75h-1.5a.75.75 0 0 1 0-1.5h1.5a.75.75 0 0 1 0 1.5ZM3 12a.75.75 0 0 1 .75-.75h7.5a.75.75 0 0 1 0 1.5h-7.5A.75.75 0 0 1 3 12ZM9 3.75a2.25 2.25 0 1 0 0 4.5 2.25 2.25 0 0 0 0-4.5ZM12.75 12a2.25 2.25 0 1 1 4.5 0 2.25 2.25 0 0 1-4.5 0ZM9 15.75a2.25 2.25 0 1 0 0 4.5 2.25 2.25 0 0 0 0-4.5Z"
            }
        }
    }
}

#[component]
pub fn PauseIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                d: "M6.75 5.25a.75.75 0 0 1 .75-.75H9a.75.75 0 0 1 .75.75v13.5a.75.75 0 0 1-.75.75H7.5a.75.75 0 0 1-.75-.75V5.25Zm7.5 0A.75.75 0 0 1 15 4.5h1.5a.75.75 0 0 1 .75.75v13.5a.75.75 0 0 1-.75.75H15a.75.75 0 0 1-.75-.75V5.25Z",
                clip_rule: "evenodd"
            }
        }
    }
}

#[component]
pub fn PauseCircleIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                d: "M2.25 12c0-5.385 4.365-9.75 9.75-9.75s9.75 4.365 9.75 9.75-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12ZM9 8.25a.75.75 0 0 0-.75.75v6c0 .414.336.75.75.75h.75a.75.75 0 0 0 .75-.75V9a.75.75 0 0 0-.75-.75H9Zm5.25 0a.75.75 0 0 0-.75.75v6c0 .414.336.75.75.75H15a.75.75 0 0 0 .75-.75V9a.75.75 0 0 0-.75-.75h-.75Z",
                clip_rule: "evenodd"
            }
        }
    }
}

#[component]
pub fn PlayIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                d: "M4.5 5.653c0-1.427 1.529-2.33 2.779-1.643l11.54 6.347c1.295.712 1.295 2.573 0 3.286L7.28 19.99c-1.25.687-2.779-.217-2.779-1.643V5.653Z",
                clip_rule: "evenodd"
            }
        }
    }
}

#[component]
pub fn StopIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                d: "M6 6h12v12H6z"
            }
        }
    }
}

#[component]
pub fn PlusIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 18 18",
            class: "{class}",
            path {
                d: "M9 1L9 17",
                stroke: "currentColor",
                stroke_width: 3,
                stroke_linecap: "round"
            }
            path {
                d: "M1 9L17 9",
                stroke: "currentColor",
                stroke_width: 3,
                stroke_linecap: "round"
            }
        }
    }
}

#[component]
pub fn SwitchIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 17 16",
            class: "{class}",
            path {
                d: "M5.16667 2.66675L5.16667 13.3334M5.16667 13.3334L2.5 10.6667M5.16667 13.3334L7.83333 10.6667M11.8333 13.3334L11.8333 2.66675M11.8333 2.66675L9.16667 5.33341M11.8333 2.66675L14.5 5.33341",
                stroke: "currentColor",
                stroke_width: 1.5,
                stroke_linecap: "round",
                stroke_linejoin: "round"
            }
        }
    }
}

#[component]
pub fn PlusCircleIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                d:"M12 2.25c-5.385 0-9.75 4.365-9.75 9.75s4.365 9.75 9.75 9.75 9.75-4.365 9.75-9.75S17.385 2.25 12 2.25ZM12.75 9a.75.75 0 0 0-1.5 0v2.25H9a.75.75 0 0 0 0 1.5h2.25V15a.75.75 0 0 0 1.5 0v-2.25H15a.75.75 0 0 0 0-1.5h-2.25V9Z"
            }
        }
    }
}

#[component]
pub fn EyeDropperIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                d: "M16.098 2.598a3.75 3.75 0 1 1 3.622 6.275l-1.72.46V12a.75.75 0 0 1-.22.53l-.75.75a.75.75 0 0 1-1.06 0l-.97-.97-7.94 7.94a2.56 2.56 0 0 1-1.81.75 1.06 1.06 0 0 0-.75.31l-.97.97a.75.75 0 0 1-1.06 0l-.75-.75a.75.75 0 0 1 0-1.06l.97-.97a1.06 1.06 0 0 0 .31-.75c0-.68.27-1.33.75-1.81L11.69 9l-.97-.97a.75.75 0 0 1 0-1.06l.75-.75A.75.75 0 0 1 12 6h2.666l.461-1.72c.165-.617.49-1.2.971-1.682Zm-3.348 7.463L4.81 18a1.06 1.06 0 0 0-.31.75c0 .318-.06.63-.172.922a2.56 2.56 0 0 1 .922-.172c.281 0 .551-.112.75-.31l7.94-7.94-1.19-1.19Z"
            }
        }
    }
}

#[component]
pub fn InfoIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 16 16",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M15 8A7 7 0 1 1 1 8a7 7 0 0 1 14 0ZM9 5a1 1 0 1 1-2 0 1 1 0 0 1 2 0ZM6.75 8a.75.75 0 0 0 0 1.5h.75v1.75a.75.75 0 0 0 1.5 0v-2.5A.75.75 0 0 0 8.25 8h-1.5Z",
            }
        }
    }
}

#[component]
pub fn AlertInfoIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 16 16",
            fill: "#F8BA77",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M15 8A7 7 0 1 1 1 8a7 7 0 0 1 14 0ZM9 5a1 1 0 1 1-2 0 1 1 0 0 1 2 0ZM6.75 8a.75.75 0 0 0 0 1.5h.75v1.75a.75.75 0 0 0 1.5 0v-2.5A.75.75 0 0 0 8.25 8h-1.5Z",
            }
        }
    }
}

#[component]
pub fn AlertWarningIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "#F8BA77",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M9.401 3.003c1.155-2 4.043-2 5.197 0l7.355 12.748c1.154 2-.29 4.5-2.599 4.5H4.645c-2.309 0-3.752-2.5-2.598-4.5L9.4 3.003ZM12 8.25a.75.75 0 0 1 .75.75v3.75a.75.75 0 0 1-1.5 0V9a.75.75 0 0 1 .75-.75Zm0 8.25a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5Z"
            }
        }
    }
}

#[component]
pub fn WarningIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M9.401 3.003c1.155-2 4.043-2 5.197 0l7.355 12.748c1.154 2-.29 4.5-2.599 4.5H4.645c-2.309 0-3.752-2.5-2.598-4.5L9.4 3.003ZM12 8.25a.75.75 0 0 1 .75.75v3.75a.75.75 0 0 1-1.5 0V9a.75.75 0 0 1 .75-.75Zm0 8.25a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5Z"
            }
        }
    }
}

// #[component]
// pub fn CheckIcon(class: Option<String>) -> Element {
//     let class = class.unwrap_or("".to_string());
//     rsx! {
//         svg {
//             view_box: "0 0 24 24",
//             fill: "currentColor",
//             class: "{class}",
//             path {
//                 fill_rule: "evenodd",
//                 clip_rule: "evenodd",
//                 d: "M2.25 12c0-5.385 4.365-9.75 9.75-9.75s9.75 4.365 9.75 9.75-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12Zm13.36-1.814a.75.75 0 1 0-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 0 0-1.06 1.06l2.25 2.25a.75.75 0 0 0 1.14-.094l3.75-5.25Z"
//             }
//         }
//     }
// }

#[component]
pub fn CancelIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            stroke_width: "3",
            stroke: "currentColor",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M6 18 18 6M6 6l12 12"
            }
        }
    }
}

#[component]
pub fn CheckIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            fill: "none",
            view_box: "0 0 24 24",
            stroke_width: "3",
            stroke: "currentColor",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "m4.5 12.75 6 6 9-13.5"
            }
        }
    }
}

#[component]
pub fn CheckCircleIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M2.25 12c0-5.385 4.365-9.75 9.75-9.75s9.75 4.365 9.75 9.75-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12Zm13.36-1.814a.75.75 0 1 0-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 0 0-1.06 1.06l2.25 2.25a.75.75 0 0 0 1.14-.094l3.75-5.25Z"
            }
        }
    }
}

#[component]
pub fn SignalIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                d: "M5.636 4.575a.75.75 0 0 1 0 1.061 9 9 0 0 0 0 12.728.75.75 0 1 1-1.06 1.06c-4.101-4.1-4.101-10.748 0-14.849a.75.75 0 0 1 1.06 0Zm12.728 0a.75.75 0 0 1 1.06 0c4.101 4.1 4.101 10.75 0 14.85a.75.75 0 1 1-1.06-1.061 9 9 0 0 0 0-12.728.75.75 0 0 1 0-1.06ZM7.757 6.697a.75.75 0 0 1 0 1.06 6 6 0 0 0 0 8.486.75.75 0 0 1-1.06 1.06 7.5 7.5 0 0 1 0-10.606.75.75 0 0 1 1.06 0Zm8.486 0a.75.75 0 0 1 1.06 0 7.5 7.5 0 0 1 0 10.606.75.75 0 0 1-1.06-1.06 6 6 0 0 0 0-8.486.75.75 0 0 1 0-1.06ZM9.879 8.818a.75.75 0 0 1 0 1.06 3 3 0 0 0 0 4.243.75.75 0 1 1-1.061 1.061 4.5 4.5 0 0 1 0-6.364.75.75 0 0 1 1.06 0Zm4.242 0a.75.75 0 0 1 1.061 0 4.5 4.5 0 0 1 0 6.364.75.75 0 0 1-1.06-1.06 3 3 0 0 0 0-4.243.75.75 0 0 1 0-1.061ZM10.875 12a1.125 1.125 0 1 1 2.25 0 1.125 1.125 0 0 1-2.25 0Z",
                clip_rule: "evenodd",
            }
        }
    }
}

#[component]
pub fn StarFourIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            width: "24",
            height: "24",
            view_box: "0 0 24 24",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            class: "{class}",
            path {
                d: "M11.375 24C11.1458 22.6458 10.5417 21.1875 9.5625 19.625C8.58333 18.0417 7.1875 16.5729 5.375 15.2187C3.58333 13.8646 1.79167 13 0 12.625V11.3125C1.77083 10.8958 3.46875 10.125 5.09375 9C6.73958 7.85417 8.11458 6.47917 9.21875 4.875C10.3437 3.22917 11.0625 1.60417 11.375 0H12.6875C12.875 1.04167 13.25 2.11458 13.8125 3.21875C14.375 4.30208 15.0938 5.34375 15.9688 6.34375C16.8646 7.32292 17.8646 8.20833 18.9687 9C20.6146 10.1667 22.2917 10.9375 24 11.3125V12.625C22.8542 12.8542 21.6667 13.3229 20.4375 14.0312C19.2292 14.7396 18.1042 15.5833 17.0625 16.5625C16.0208 17.5208 15.1667 18.5312 14.5 19.5937C13.5208 21.1562 12.9167 22.625 12.6875 24H11.375Z",
                fill: "currentColor",
            }
        }
    }
}

#[component]
pub fn ViewfinderIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            class: "{class}",
            // Top-left corner
            path { d: "M3 7.5V6a3 3 0 0 1 3-3h1.5" }
            // Top-right corner
            path { d: "M16.5 3H18a3 3 0 0 1 3 3v1.5" }
            // Bottom-left corner
            path { d: "M3 16.5V18a3 3 0 0 0 3 3h1.5" }
            // Bottom-right corner
            path { d: "M16.5 21H18a3 3 0 0 0 3-3v-1.5" }
            // Center circle
            circle { cx: "12", cy: "12", r: "3.75" }
        }
    }
}

#[component]
pub fn UserIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M7.5 6a4.5 4.5 0 1 1 9 0 4.5 4.5 0 0 1-9 0ZM3.751 20.105a8.25 8.25 0 0 1 16.498 0 .75.75 0 0 1-.437.695A18.683 18.683 0 0 1 12 22.5c-2.786 0-5.433-.608-7.812-1.7a.75.75 0 0 1-.437-.695Z"
            }
        }
    }
}

#[component]
pub fn ShieldIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                d: "M12.516 2.17a.75.75 0 0 0-1.032 0 11.209 11.209 0 0 1-7.877 3.08.75.75 0 0 0-.722.515A12.74 12.74 0 0 0 2.25 9.75c0 5.942 4.064 10.933 9.563 12.348a.749.749 0 0 0 .374 0c5.499-1.415 9.563-6.406 9.563-12.348 0-1.39-.223-2.73-.635-3.985a.75.75 0 0 0-.722-.516l-.143.001c-2.996 0-5.717-1.17-7.734-3.08Zm3.094 8.016a.75.75 0 1 0-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 0 0-1.06 1.06l2.25 2.25a.75.75 0 0 0 1.14-.094l3.75-5.25Z",
                clip_rule: "evenodd",
            }
        }
    }
}

#[component]
pub fn BookmarkIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("size-6".to_string());
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                d: "M6.32 2.577a49.255 49.255 0 0 1 11.36 0c1.497.174 2.57 1.46 2.57 2.93V21a.75.75 0 0 1-1.085.67L12 18.089l-7.165 3.583A.75.75 0 0 1 3.75 21V5.507c0-1.47 1.073-2.756 2.57-2.93Z",
                clip_rule: "evenodd"
            }
        }
    }
}

#[component]
pub fn UserCircleIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M18.685 19.097A9.723 9.723 0 0 0 21.75 12c0-5.385-4.365-9.75-9.75-9.75S2.25 6.615 2.25 12a9.723 9.723 0 0 0 3.065 7.097A9.716 9.716 0 0 0 12 21.75a9.716 9.716 0 0 0 6.685-2.653Zm-12.54-1.285A7.486 7.486 0 0 1 12 15a7.486 7.486 0 0 1 5.855 2.812A8.224 8.224 0 0 1 12 20.25a8.224 8.224 0 0 1-5.855-2.438ZM15.75 9a3.75 3.75 0 1 1-7.5 0 3.75 3.75 0 0 1 7.5 0Z"
            }
        }
    }
}

#[component]
pub fn DrawerIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 29 38",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M5 33V5H16V33H5ZM0 3C0 1.34315 1.34315 0 3 0H26C27.6569 0 29 1.34315 29 3V35C29 36.6569 27.6569 38 26 38H3C1.34315 38 0 36.6569 0 35V3Z"
            }
        }
    }
}

#[component]
pub fn HamburgerMenuIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("h-6 w-6".to_string());
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            fill: "none",
            view_box: "0 0 24 24",
            stroke_width: "2.5",
            stroke: "currentColor",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5",
            }
        }
    }
}

#[component]
pub fn UserGroupIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M8.25 6.75a3.75 3.75 0 1 1 7.5 0 3.75 3.75 0 0 1-7.5 0ZM15.75 9.75a3 3 0 1 1 6 0 3 3 0 0 1-6 0ZM2.25 9.75a3 3 0 1 1 6 0 3 3 0 0 1-6 0ZM6.31 15.117A6.745 6.745 0 0 1 12 12a6.745 6.745 0 0 1 6.709 7.498.75.75 0 0 1-.372.568A12.696 12.696 0 0 1 12 21.75c-2.305 0-4.47-.612-6.337-1.684a.75.75 0 0 1-.372-.568 6.787 6.787 0 0 1 1.019-4.38Z",
            }
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M5.082 14.254a8.287 8.287 0 0 0-1.308 5.135 9.687 9.687 0 0 1-1.764-.44l-.115-.04a.563.563 0 0 1-.373-.487l-.01-.121a3.75 3.75 0 0 1 3.57-4.047ZM20.226 19.389a8.287 8.287 0 0 0-1.308-5.135 3.75 3.75 0 0 1 3.57 4.047l-.01.121a.563.563 0 0 1-.373.486l-.115.04c-.567.2-1.156.349-1.764.441Z"
            }
        }
    }
}

#[component]
pub fn TagIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M5.25 2.25a3 3 0 0 0-3 3v4.318a3 3 0 0 0 .879 2.121l9.58 9.581c.92.92 2.39 1.186 3.548.428a18.849 18.849 0 0 0 5.441-5.44c.758-1.16.492-2.629-.428-3.548l-9.58-9.581a3 3 0 0 0-2.122-.879H5.25ZM6.375 7.5a1.125 1.125 0 1 0 0-2.25 1.125 1.125 0 0 0 0 2.25Z"
            }
        }
    }
}

#[component]
pub fn GlobeIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                d: "M21.721 12.752a9.711 9.711 0 0 0-.945-5.003 12.754 12.754 0 0 1-4.339 2.708 18.991 18.991 0 0 1-.214 4.772 17.165 17.165 0 0 0 5.498-2.477ZM14.634 15.55a17.324 17.324 0 0 0 .332-4.647c-.952.227-1.945.347-2.966.347-1.021 0-2.014-.12-2.966-.347a17.515 17.515 0 0 0 .332 4.647 17.385 17.385 0 0 0 5.268 0ZM9.772 17.119a18.963 18.963 0 0 0 4.456 0A17.182 17.182 0 0 1 12 21.724a17.18 17.18 0 0 1-2.228-4.605ZM7.777 15.23a18.87 18.87 0 0 1-.214-4.774 12.753 12.753 0 0 1-4.34-2.708 9.711 9.711 0 0 0-.944 5.004 17.165 17.165 0 0 0 5.498 2.477ZM21.356 14.752a9.765 9.765 0 0 1-7.478 6.817 18.64 18.64 0 0 0 1.988-4.718 18.627 18.627 0 0 0 5.49-2.098ZM2.644 14.752c1.682.971 3.53 1.688 5.49 2.099a18.64 18.64 0 0 0 1.988 4.718 9.765 9.765 0 0 1-7.478-6.816ZM13.878 2.43a9.755 9.755 0 0 1 6.116 3.986 11.267 11.267 0 0 1-3.746 2.504 18.63 18.63 0 0 0-2.37-6.49ZM12 2.276a17.152 17.152 0 0 1 2.805 7.121c-.897.23-1.837.353-2.805.353-.968 0-1.908-.122-2.805-.353A17.151 17.151 0 0 1 12 2.276ZM10.122 2.43a18.629 18.629 0 0 0-2.37 6.49 11.266 11.266 0 0 1-3.746-2.504 9.754 9.754 0 0 1 6.116-3.985Z"
            }
        }
    }
}

#[component]
pub fn PriceChartOutlineIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            stroke: "currentColor",
            stroke_width: "1.5",
            fill: "none",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M3.75 3v11.25A2.25 2.25 0 0 0 6 16.5h2.25M3.75 3h-1.5m1.5 0h16.5m0 0h1.5m-1.5 0v11.25A2.25 2.25 0 0 1 18 16.5h-2.25m-7.5 0h7.5m-7.5 0-1 3m8.5-3 1 3m0 0 .5 1.5m-.5-1.5h-9.5m0 0-.5 1.5m.75-9 3-3 2.148 2.148A12.061 12.061 0 0 1 16.5 7.605"
            }
        }
    }
}

#[component]
pub fn PriceChartIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M2.25 2.25a.75.75 0 0 0 0 1.5H3v10.5a3 3 0 0 0 3 3h1.21l-1.172 3.513a.75.75 0 0 0 1.424.474l.329-.987h8.418l.33.987a.75.75 0 0 0 1.422-.474l-1.17-3.513H18a3 3 0 0 0 3-3V3.75h.75a.75.75 0 0 0 0-1.5H2.25Zm6.54 15h6.42l.5 1.5H8.29l.5-1.5Zm8.085-8.995a.75.75 0 1 0-.75-1.299 12.81 12.81 0 0 0-3.558 3.05L11.03 8.47a.75.75 0 0 0-1.06 0l-3 3a.75.75 0 1 0 1.06 1.06l2.47-2.47 1.617 1.618a.75.75 0 0 0 1.146-.102 11.312 11.312 0 0 1 3.612-3.321Z"
            }
        }
    }
}

#[component]
pub fn GlobeOutlineIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            stroke: "currentColor",
            stroke_width: "1.5",
            fill: "none",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M12 21a9.004 9.004 0 0 0 8.716-6.747M12 21a9.004 9.004 0 0 1-8.716-6.747M12 21c2.485 0 4.5-4.03 4.5-9S14.485 3 12 3m0 18c-2.485 0-4.5-4.03-4.5-9S9.515 3 12 3m0 0a8.997 8.997 0 0 1 7.843 4.582M12 3a8.997 8.997 0 0 0-7.843 4.582m15.686 0A11.953 11.953 0 0 1 12 10.5c-2.998 0-5.74-1.1-7.843-2.918m15.686 0A8.959 8.959 0 0 1 21 12c0 .778-.099 1.533-.284 2.253m0 0A17.919 17.919 0 0 1 12 16.5c-3.162 0-6.133-.815-8.716-2.247m0 0A9.015 9.015 0 0 1 3 12c0-1.605.42-3.113 1.157-4.418"
            }
        }
    }
}

#[component]
pub fn SquareStackIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                d: "M11.644 1.59a.75.75 0 0 1 .712 0l9.75 5.25a.75.75 0 0 1 0 1.32l-9.75 5.25a.75.75 0 0 1-.712 0l-9.75-5.25a.75.75 0 0 1 0-1.32l9.75-5.25Z"
            }
            path {
                d: "m3.265 10.602 7.668 4.129a2.25 2.25 0 0 0 2.134 0l7.668-4.13 1.37.739a.75.75 0 0 1 0 1.32l-9.75 5.25a.75.75 0 0 1-.71 0l-9.75-5.25a.75.75 0 0 1 0-1.32l1.37-.738Z"
            }
            path {
                d: "m10.933 19.231-7.668-4.13-1.37.739a.75.75 0 0 0 0 1.32l9.75 5.25c.221.12.489.12.71 0l9.75-5.25a.75.75 0 0 0 0-1.32l-1.37-.738-7.668 4.13a2.25 2.25 0 0 1-2.134-.001Z"
            }
        }
    }
}

#[component]
pub fn CircleStackIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                d:"M21 6.375c0 2.692-4.03 4.875-9 4.875S3 9.067 3 6.375 7.03 1.5 12 1.5s9 2.183 9 4.875Z"
            }
            path {
                d:"M12 12.75c2.685 0 5.19-.586 7.078-1.609a8.283 8.283 0 0 0 1.897-1.384c.016.121.025.244.025.368C21 12.817 16.97 15 12 15s-9-2.183-9-4.875c0-.124.009-.247.025-.368a8.285 8.285 0 0 0 1.897 1.384C6.809 12.164 9.315 12.75 12 12.75Z"
            }
            path {
                d:"M12 16.5c2.685 0 5.19-.586 7.078-1.609a8.282 8.282 0 0 0 1.897-1.384c.016.121.025.244.025.368 0 2.692-4.03 4.875-9 4.875s-9-2.183-9-4.875c0-.124.009-.247.025-.368a8.284 8.284 0 0 0 1.897 1.384C6.809 15.914 9.315 16.5 12 16.5Z"
            }
            path {
                d:"M12 20.25c2.685 0 5.19-.586 7.078-1.609a8.282 8.282 0 0 0 1.897-1.384c.016.121.025.244.025.368 0 2.692-4.03 4.875-9 4.875s-9-2.183-9-4.875c0-.124.009-.247.025-.368a8.284 8.284 0 0 0 1.897 1.384C6.809 19.664 9.315 20.25 12 20.25Z"
            }
        }
    }
}
#[component]
pub fn CircleStackOutlineIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.5",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M20.25 6.375c0 2.278-3.694 4.125-8.25 4.125S3.75 8.653 3.75 6.375m16.5 0c0-2.278-3.694-4.125-8.25-4.125S3.75 4.097 3.75 6.375m16.5 0v11.25c0 2.278-3.694 4.125-8.25 4.125s-8.25-1.847-8.25-4.125V6.375m16.5 0v3.75m-16.5-3.75v3.75m16.5 0v3.75C20.25 16.153 16.556 18 12 18s-8.25-1.847-8.25-4.125v-3.75m16.5 0c0 2.278-3.694 4.125-8.25 4.125s-8.25-1.847-8.25-4.125"
            }
        }
    }
}

#[component]
pub fn ScaleIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                d:"M12 2.25a.75.75 0 0 1 .75.75v.756a49.106 49.106 0 0 1 9.152 1 .75.75 0 0 1-.152 1.485h-1.918l2.474 10.124a.75.75 0 0 1-.375.84A6.723 6.723 0 0 1 18.75 18a6.723 6.723 0 0 1-3.181-.795.75.75 0 0 1-.375-.84l2.474-10.124H12.75v13.28c1.293.076 2.534.343 3.697.776a.75.75 0 0 1-.262 1.453h-8.37a.75.75 0 0 1-.262-1.453c1.162-.433 2.404-.7 3.697-.775V6.24H6.332l2.474 10.124a.75.75 0 0 1-.375.84A6.723 6.723 0 0 1 5.25 18a6.723 6.723 0 0 1-3.181-.795.75.75 0 0 1-.375-.84L4.168 6.241H2.25a.75.75 0 0 1-.152-1.485 49.105 49.105 0 0 1 9.152-1V3a.75.75 0 0 1 .75-.75Zm4.878 13.543 1.872-7.662 1.872 7.662h-3.744Zm-9.756 0L5.25 8.131l-1.872 7.662h3.744Z"
            }
        }
    }
}

#[component]
pub fn BeakerIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                d:"M10.5 3.798v5.02a3 3 0 0 1-.879 2.121l-2.377 2.377a9.845 9.845 0 0 1 5.091 1.013 8.315 8.315 0 0 0 5.713.636l.285-.071-3.954-3.955a3 3 0 0 1-.879-2.121v-5.02a23.614 23.614 0 0 0-3 0Zm4.5.138a.75.75 0 0 0 .093-1.495A24.837 24.837 0 0 0 12 2.25a25.048 25.048 0 0 0-3.093.191A.75.75 0 0 0 9 3.936v4.882a1.5 1.5 0 0 1-.44 1.06l-6.293 6.294c-1.62 1.621-.903 4.475 1.471 4.88 2.686.46 5.447.698 8.262.698 2.816 0 5.576-.239 8.262-.697 2.373-.406 3.092-3.26 1.47-4.881L15.44 9.879A1.5 1.5 0 0 1 15 8.818V3.936Z"
            }
        }
    }
}

#[component]
pub fn CarrotDownIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 12 7",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                d: "M11.3535 1.35378L6.35354 6.35378C6.3071 6.40027 6.25196 6.43715 6.19126 6.46231C6.13056 6.48748 6.0655 6.50043 5.99979 6.50043C5.93408 6.50043 5.86902 6.48748 5.80832 6.46231C5.74762 6.43715 5.69248 6.40027 5.64604 6.35378L0.64604 1.35378C0.552219 1.25996 0.499512 1.13272 0.499512 1.00003C0.499512 0.867352 0.552219 0.740104 0.64604 0.646284C0.73986 0.552463 0.867108 0.499756 0.99979 0.499756C1.13247 0.499756 1.25972 0.552463 1.35354 0.646284L5.99979 5.29316L10.646 0.646284C10.6925 0.599829 10.7476 0.562978 10.8083 0.537837C10.869 0.512696 10.9341 0.499756 10.9998 0.499756C11.0655 0.499756 11.1305 0.512696 11.1912 0.537837C11.2519 0.562978 11.3071 0.599829 11.3535 0.646284C11.4 0.692739 11.4368 0.747889 11.462 0.808586C11.4871 0.869282 11.5001 0.934336 11.5001 1.00003C11.5001 1.06573 11.4871 1.13079 11.462 1.19148C11.4368 1.25218 11.4 1.30733 11.3535 1.35378Z"
            }
        }
    }
}

#[component]
pub fn CarrotRightIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 12 12",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                d: "M8.64798 6.39795L4.89798 10.1479C4.79231 10.2536 4.64898 10.313 4.49954 10.313C4.3501 10.313 4.20678 10.2536 4.1011 10.1479C3.99543 10.0423 3.93607 9.89896 3.93607 9.74951C3.93607 9.60007 3.99543 9.45675 4.1011 9.35107L7.45314 5.99998L4.10204 2.64795C4.04972 2.59563 4.00821 2.53351 3.9799 2.46514C3.95158 2.39678 3.937 2.32351 3.937 2.24951C3.937 2.17551 3.95158 2.10224 3.9799 2.03388C4.00821 1.96551 4.04972 1.9034 4.10204 1.85107C4.15437 1.79875 4.21648 1.75724 4.28485 1.72893C4.35321 1.70061 4.42648 1.68604 4.50048 1.68604C4.57448 1.68604 4.64775 1.70061 4.71611 1.72893C4.78448 1.75724 4.84659 1.79875 4.89892 1.85107L8.64892 5.60107C8.70129 5.65339 8.74283 5.71554 8.77114 5.78395C8.79945 5.85235 8.81398 5.92567 8.81389 5.99971C8.8138 6.07374 8.7991 6.14703 8.77063 6.21537C8.74216 6.28371 8.70048 6.34575 8.64798 6.39795Z" ,
            }
        }
    }
}

#[component]
pub fn BoltIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                d: "M14.615 1.595a.75.75 0 0 1 .359.852L12.982 9.75h7.268a.75.75 0 0 1 .548 1.262l-10.5 11.25a.75.75 0 0 1-1.272-.71l1.992-7.302H3.75a.75.75 0 0 1-.548-1.262l10.5-11.25a.75.75 0 0 1 .913-.143Z"
            }
        }
    }
}

#[component]
pub fn BoltOutlineIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            fill: "none",
            stroke: "currentColor",
            view_box: "0 0 24 24",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: 1.5,
                d: "m3.75 13.5 10.5-11.25L12 10.5h8.25L9.75 21.75 12 13.5H3.75Z"
            }
        }
    }
}

#[component]
pub fn PieChartIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                d: "M2.25 13.5a8.25 8.25 0 0 1 8.25-8.25.75.75 0 0 1 .75.75v6.75H18a.75.75 0 0 1 .75.75 8.25 8.25 0 0 1-16.5 0Z"
            }
            path {
                fill_rule: "evenodd",
                d: "M12.75 3a.75.75 0 0 1 .75-.75 8.25 8.25 0 0 1 8.25 8.25.75.75 0 0 1-.75.75h-7.5a.75.75 0 0 1-.75-.75V3Z"
            }
        }
    }
}

#[component]
pub fn WalletIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            stroke: "currentColor",
            fill: "none",
            stroke_width: "1.5",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M21 12a2.25 2.25 0 0 0-2.25-2.25H15a3 3 0 1 1-6 0H5.25A2.25 2.25 0 0 0 3 12m18 0v6a2.25 2.25 0 0 1-2.25 2.25H5.25A2.25 2.25 0 0 1 3 18v-6m18 0V9M3 12V9m18 0a2.25 2.25 0 0 0-2.25-2.25H5.25A2.25 2.25 0 0 0 3 9m18 0V6a2.25 2.25 0 0 0-2.25-2.25H5.25A2.25 2.25 0 0 0 3 6v3"
            }
        }
    }
}

#[component]
pub fn WalletSolidIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                d: "M2.273 5.625A4.483 4.483 0 0 1 5.25 4.5h13.5c1.141 0 2.183.425 2.977 1.125A3 3 0 0 0 18.75 3H5.25a3 3 0 0 0-2.977 2.625ZM2.273 8.625A4.483 4.483 0 0 1 5.25 7.5h13.5c1.141 0 2.183.425 2.977 1.125A3 3 0 0 0 18.75 6H5.25a3 3 0 0 0-2.977 2.625ZM5.25 9a3 3 0 0 0-3 3v6a3 3 0 0 0 3 3h13.5a3 3 0 0 0 3-3v-6a3 3 0 0 0-3-3H15a.75.75 0 0 0-.75.75 2.25 2.25 0 0 1-4.5 0A.75.75 0 0 0 9 9H5.25Z"
            }
        }
    }
}

#[component]
pub fn EyeSlashSolidIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                d: "M3.53 2.47a.75.75 0 0 0-1.06 1.06l18 18a.75.75 0 1 0 1.06-1.06l-18-18ZM22.676 12.553a11.249 11.249 0 0 1-2.631 4.31l-3.099-3.099a5.25 5.25 0 0 0-6.71-6.71L7.759 4.577a11.217 11.217 0 0 1 4.242-.827c4.97 0 9.185 3.223 10.675 7.69.12.362.12.752 0 1.113Z"
            }
            path {
                d: "M15.75 12c0 .18-.013.357-.037.53l-4.244-4.243A3.75 3.75 0 0 1 15.75 12ZM12.53 15.713l-4.243-4.244a3.75 3.75 0 0 0 4.244 4.243Z"
            }
            path {
                d: "M6.75 12c0-.619.107-1.213.304-1.764l-3.1-3.1a11.25 11.25 0 0 0-2.63 4.31c-.12.362-.12.752 0 1.114 1.489 4.467 5.704 7.69 10.675 7.69 1.5 0 2.933-.294 4.242-.827l-2.477-2.477A5.25 5.25 0 0 1 6.75 12Z"
            }
        }
    }
}

#[component]
pub fn SwapIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 17 16",
            class: "{class} rotate-45",
            path {
                stroke: "currentColor",
                d: "M5.16667 2.6665L5.16667 13.3332M5.16667 13.3332L2.5 10.6665M5.16667 13.3332L7.83333 10.6665M11.8333 13.3332L11.8333 2.6665M11.8333 2.6665L9.16667 5.33317M11.8333 2.6665L14.5 5.33317",
                stroke_width: 2,
                stroke_linecap:"round",
                stroke_linejoin:"round"
            }
        }
    }
}

#[component]
pub fn PaperAirplaneIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                d: "M3.478 2.404a.75.75 0 0 0-.926.941l2.432 7.905H13.5a.75.75 0 0 1 0 1.5H4.984l-2.432 7.905a.75.75 0 0 0 .926.94 60.519 60.519 0 0 0 18.445-8.986.75.75 0 0 0 0-1.218A60.517 60.517 0 0 0 3.478 2.404Z"
            }
        }
    }
}

#[component]
pub fn CopyIcon(class: Option<String>, solid: bool) -> Element {
    let class = class.unwrap_or("".to_string());
    if solid {
        rsx! {
            svg {
                view_box: "0 0 24 24",
                fill: "currentColor",
                class: "{class}",
                path {
                    fill_rule: "evenodd",
                    d: "M7.5 3.375c0-1.036.84-1.875 1.875-1.875h.375a3.75 3.75 0 0 1 3.75 3.75v1.875C13.5 8.161 14.34 9 15.375 9h1.875A3.75 3.75 0 0 1 21 12.75v3.375C21 17.16 20.16 18 19.125 18h-9.75A1.875 1.875 0 0 1 7.5 16.125V3.375Z",
                }
                path {
                    fill_rule: "evenodd",
                    d: "M15 5.25a5.23 5.23 0 0 0-1.279-3.434 9.768 9.768 0 0 1 6.963 6.963A5.23 5.23 0 0 0 17.25 7.5h-1.875A.375.375 0 0 1 15 7.125V5.25ZM4.875 6H6v10.125A3.375 3.375 0 0 0 9.375 19.5H16.5v1.125c0 1.035-.84 1.875-1.875 1.875h-9.75A1.875 1.875 0 0 1 3 20.625V7.875C3 6.839 3.84 6 4.875 6Z",
                }
            }
        }
    } else {
        rsx! {
            svg {
                fill: "none",
                view_box: "0 0 24 24",
                stroke_width: "2",
                stroke: "currentColor",
                class: "{class}",
                path {
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    d: "M15.75 17.25v3.375c0 .621-.504 1.125-1.125 1.125h-9.75a1.125 1.125 0 0 1-1.125-1.125V7.875c0-.621.504-1.125 1.125-1.125H6.75a9.06 9.06 0 0 1 1.5.124m7.5 10.376h3.375c.621 0 1.125-.504 1.125-1.125V11.25c0-4.46-3.243-8.161-7.5-8.876a9.06 9.06 0 0 0-1.5-.124H9.375c-.621 0-1.125.504-1.125 1.125v3.5m7.5 10.375H9.375a1.125 1.125 0 0 1-1.125-1.125v-9.25m12 6.625v-1.875a3.375 3.375 0 0 0-3.375-3.375h-1.5a1.125 1.125 0 0 1-1.125-1.125v-1.5a3.375 3.375 0 0 0-3.375-3.375H9.75",
                }
            }
        }
    }
}

#[component]
pub fn ChartIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                d: "M18.375 2.25c-1.035 0-1.875.84-1.875 1.875v15.75c0 1.035.84 1.875 1.875 1.875h.75c1.035 0 1.875-.84 1.875-1.875V4.125c0-1.036-.84-1.875-1.875-1.875h-.75ZM9.75 8.625c0-1.036.84-1.875 1.875-1.875h.75c1.036 0 1.875.84 1.875 1.875v11.25c0 1.035-.84 1.875-1.875 1.875h-.75a1.875 1.875 0 0 1-1.875-1.875V8.625ZM3 13.125c0-1.036.84-1.875 1.875-1.875h.75c1.036 0 1.875.84 1.875 1.875v6.75c0 1.035-.84 1.875-1.875 1.875h-.75A1.875 1.875 0 0 1 3 19.875v-6.75Z"
            }
        }
    }
}

#[component]
pub fn GithubIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 1024 1024",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                transform: "scale(64)",
                d: "M8 0C3.58 0 0 3.58 0 8C0 11.54 2.29 14.53 5.47 15.59C5.87 15.66 6.02 15.42 6.02 15.21C6.02 15.02 6.01 14.39 6.01 13.72C4 14.09 3.48 13.23 3.32 12.78C3.23 12.55 2.84 11.84 2.5 11.65C2.22 11.5 1.82 11.13 2.49 11.12C3.12 11.11 3.57 11.7 3.72 11.94C4.44 13.15 5.59 12.81 6.05 12.6C6.12 12.08 6.33 11.73 6.56 11.53C4.78 11.33 2.92 10.64 2.92 7.58C2.92 6.71 3.23 5.99 3.74 5.43C3.66 5.23 3.38 4.41 3.82 3.31C3.82 3.31 4.49 3.1 6.02 4.13C6.66 3.95 7.34 3.86 8.02 3.86C8.7 3.86 9.38 3.95 10.02 4.13C11.55 3.09 12.22 3.31 12.22 3.31C12.66 4.41 12.38 5.23 12.3 5.43C12.81 5.99 13.12 6.7 13.12 7.58C13.12 10.65 11.25 11.33 9.47 11.53C9.76 11.78 10.01 12.26 10.01 13.01C10.01 14.08 10 14.94 10 15.21C10 15.42 10.15 15.67 10.55 15.59C13.71 14.53 16 11.53 16 8C16 3.58 12.42 0 8 0Z",
            }
        }
    }
}

#[component(no_case_check)]
pub fn XIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 300 271",
            fill: "currentColor",
            class: "{class}",
            path {
                d: "m236 0h46l-101 115 118 156h-92.6l-72.5-94.8-83 94.8h-46l107-123-113-148h94.9l65.5 86.6zm-16.1 244h25.5l-165-218h-27.4z"
                // d: "M178.57 127.15 290.27 0h-26.46l-97.03 110.38L89.34 0H0l117.13 166.93L0 300.25h26.46l102.4-116.59 81.8 116.59h89.34M36.01 19.54H76.66l187.13 262.13h-40.66"
            }
        }
    }
}

#[component]
pub fn MegaphoneIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                d: "M16.881 4.345A23.112 23.112 0 0 1 8.25 6H7.5a5.25 5.25 0 0 0-.88 10.427 21.593 21.593 0 0 0 1.378 3.94c.464 1.004 1.674 1.32 2.582.796l.657-.379c.88-.508 1.165-1.593.772-2.468a17.116 17.116 0 0 1-.628-1.607c1.918.258 3.76.75 5.5 1.446A21.727 21.727 0 0 0 18 11.25c0-2.414-.393-4.735-1.119-6.905ZM18.26 3.74a23.22 23.22 0 0 1 1.24 7.51 23.22 23.22 0 0 1-1.41 7.992.75.75 0 1 0 1.409.516 24.555 24.555 0 0 0 1.415-6.43 2.992 2.992 0 0 0 .836-2.078c0-.807-.319-1.54-.836-2.078a24.65 24.65 0 0 0-1.415-6.43.75.75 0 1 0-1.409.516c.059.16.116.321.17.483Z"
            }
        }
    }
}

#[component]
pub fn OreWordmarkIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 120 24",
            fill: "currentColor",
            class: "{class}",
            path {
                d: "M32.3192 22H8.73312C7.79376 22 6.90545 21.8426 6.0682 21.5278C5.25136 21.1944 4.52642 20.75 3.89338 20.1944C3.28075 19.6204 2.79065 18.963 2.42308 18.2222C2.07592 17.463 1.90234 16.6574 1.90234 15.8056V8.19444C1.90234 7.34259 2.07592 6.5463 2.42308 5.80556C2.79065 5.0463 3.28075 4.38889 3.89338 3.83333C4.52642 3.25926 5.25136 2.81481 6.0682 2.5C6.90545 2.16667 7.79376 2 8.73312 2H32.3192C33.2586 2 34.1367 2.16667 34.9535 2.5C35.7907 2.81481 36.5157 3.25926 37.1283 3.83333C37.7613 4.38889 38.2514 5.0463 38.5986 5.80556C38.9662 6.5463 39.15 7.34259 39.15 8.19444V15.8056C39.15 16.6574 38.9662 17.463 38.5986 18.2222C38.2514 18.963 37.7613 19.6204 37.1283 20.1944C36.5157 20.75 35.7907 21.1944 34.9535 21.5278C34.1367 21.8426 33.2586 22 32.3192 22ZM33.3607 13.9167V10.0833C33.3607 9.69444 33.279 9.33333 33.1156 9C32.9727 8.64815 32.7582 8.34259 32.4724 8.08333C32.2069 7.82407 31.8801 7.62037 31.4921 7.47222C31.1246 7.32407 30.7162 7.25 30.2669 7.25H10.8467C9.94816 7.25 9.19259 7.51852 8.57996 8.05556C7.98776 8.59259 7.69165 9.26852 7.69165 10.0833V13.9167C7.69165 14.3056 7.76313 14.6759 7.90607 15.0278C8.06944 15.3611 8.28386 15.6574 8.54933 15.9167C8.83522 16.1759 9.17217 16.3796 9.56016 16.5278C9.94816 16.6759 10.3668 16.75 10.816 16.75H30.2056C31.1246 16.75 31.8801 16.4722 32.4724 15.9167C33.0646 15.3611 33.3607 14.6944 33.3607 13.9167Z"
            }
            path {
                d: "M78.7146 22H71.6387L67.2585 16.2222H47.7157V22H41.9264V2H72.2207C73.1805 2 74.0688 2.15741 74.8857 2.47222C75.7229 2.78704 76.4478 3.2037 77.0605 3.72222C77.6935 4.24074 78.1836 4.84259 78.5308 5.52778C78.8983 6.19444 79.0821 6.87963 79.0821 7.58333V10.5C79.0821 11.0926 78.9392 11.6944 78.6533 12.3056C78.3878 12.9167 78.0202 13.4815 77.5506 14C77.1013 14.5185 76.5704 14.963 75.9577 15.3333C75.3655 15.6852 74.7529 15.9167 74.1199 16.0278L78.7146 22ZM73.2928 9.25V8.86111C73.2928 8.43519 73.0886 8.06482 72.6802 7.75C72.2718 7.41667 71.7102 7.25 70.9955 7.25H47.7157V10.9722H70.9955C71.7715 10.9722 72.3433 10.7963 72.7108 10.4444C73.0988 10.0926 73.2928 9.69444 73.2928 9.25Z"
            }
            path {
                d: "M81.8607 2H118.098V7.25H87.65V9.38889H117.362V14.6111H87.65V16.75H118.098V22H81.8607V2Z"
            }
        }
    }
}

#[component]
pub fn ReceiptRefundIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.5",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m0 12.75h7.5m-7.5 3H12M10.5 21h2.25m-8.625-6.472A11.956 11.956 0 0112 2.25c1.982 0 3.91.402 5.715 1.172M5.625 17.25a11.956 11.956 0 01-1.39-2.938m13.422-5.016a11.956 11.956 0 013.422 5.016M12 21.75c-1.982 0-3.91-.402-5.715-1.172"
            }
        }
    }
}

#[component]
pub fn ReceiptRefundSolidIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                d: "M12 1.5c-1.921 0-3.816.111-5.68.327-1.497.174-2.57 1.46-2.57 2.93V21.75a.75.75 0 0 0 1.029.696l3.471-1.388 3.472 1.388a.75.75 0 0 0 .556 0l3.472-1.388 3.471 1.388a.75.75 0 0 0 1.029-.696V4.757c0-1.47-1.073-2.756-2.57-2.93A49.255 49.255 0 0 0 12 1.5Zm-.97 6.53a.75.75 0 1 0-1.06-1.06L7.72 9.22a.75.75 0 0 0 0 1.06l2.25 2.25a.75.75 0 1 0 1.06-1.06l-.97-.97h3.065a1.875 1.875 0 0 1 0 3.75H12a.75.75 0 0 0 0 1.5h1.125a3.375 3.375 0 1 0 0-6.75h-3.064l.97-.97Z",
                clip_rule: "evenodd",
            }
        }
    }
}

#[component]
pub fn TreasuryIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                d: "M11.584 2.376a.75.75 0 0 1 .832 0l9 6a.75.75 0 1 1-.832 1.248L12 3.901 3.416 9.624a.75.75 0 0 1-.832-1.248l9-6Z"
            }
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M20.25 10.332v9.918H21a.75.75 0 0 1 0 1.5H3a.75.75 0 0 1 0-1.5h.75v-9.918a.75.75 0 0 1 .634-.74A49.109 49.109 0 0 1 12 9c2.59 0 5.134.202 7.616.592a.75.75 0 0 1 .634.74Zm-7.5 2.418a.75.75 0 0 0-1.5 0v6.75a.75.75 0 0 0 1.5 0v-6.75Zm3-.75a.75.75 0 0 1 .75.75v6.75a.75.75 0 0 1-1.5 0v-6.75a.75.75 0 0 1 .75-.75ZM9 12.75a.75.75 0 0 0-1.5 0v6.75a.75.75 0 0 0 1.5 0v-6.75Z"
            }
            path {
                d: "M12 7.875a1.125 1.125 0 1 0 0-2.25 1.125 1.125 0 0 0 0 2.25Z"
            }
        }
    }
}

#[component]
pub fn BusIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                d: "M3.375 4.5C2.339 4.5 1.5 5.34 1.5 6.375V13.5h12V6.375c0-1.036-.84-1.875-1.875-1.875h-8.25ZM13.5 15h-12v2.625c0 1.035.84 1.875 1.875 1.875h.375a3 3 0 1 1 6 0h3a.75.75 0 0 0 .75-.75V15Z"
            }
            path {
                d: "M8.25 19.5a1.5 1.5 0 1 0-3 0 1.5 1.5 0 0 0 3 0ZM15.75 6.75a.75.75 0 0 0-.75.75v11.25c0 .087.015.17.042.248a3 3 0 0 1 5.958.464c.853-.175 1.522-.935 1.464-1.883a18.659 18.659 0 0 0-3.732-10.104 1.837 1.837 0 0 0-1.47-.725H15.75Z"
            }
            path {
                d: "M19.5 19.5a1.5 1.5 0 1 0-3 0 1.5 1.5 0 0 0 3 0Z"
            }
        }
    }
}

#[component]
pub fn LifeBuoyIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M19.449 8.448 16.388 11a4.52 4.52 0 0 1 0 2.002l3.061 2.55a8.275 8.275 0 0 0 0-7.103ZM15.552 19.45 13 16.388a4.52 4.52 0 0 1-2.002 0l-2.55 3.061a8.275 8.275 0 0 0 7.103 0ZM4.55 15.552 7.612 13a4.52 4.52 0 0 1 0-2.002L4.551 8.45a8.275 8.275 0 0 0 0 7.103ZM8.448 4.55 11 7.612a4.52 4.52 0 0 1 2.002 0l2.55-3.061a8.275 8.275 0 0 0-7.103 0Zm8.657-.86a9.776 9.776 0 0 1 1.79 1.415 9.776 9.776 0 0 1 1.414 1.788 9.764 9.764 0 0 1 0 10.211 9.777 9.777 0 0 1-1.415 1.79 9.777 9.777 0 0 1-1.788 1.414 9.764 9.764 0 0 1-10.212 0 9.776 9.776 0 0 1-1.788-1.415 9.776 9.776 0 0 1-1.415-1.788 9.764 9.764 0 0 1 0-10.212 9.774 9.774 0 0 1 1.415-1.788A9.774 9.774 0 0 1 6.894 3.69a9.764 9.764 0 0 1 10.211 0ZM14.121 9.88a2.985 2.985 0 0 0-1.11-.704 3.015 3.015 0 0 0-2.022 0 2.985 2.985 0 0 0-1.11.704c-.326.325-.56.705-.704 1.11a3.015 3.015 0 0 0 0 2.022c.144.405.378.785.704 1.11.325.326.705.56 1.11.704.652.233 1.37.233 2.022 0a2.985 2.985 0 0 0 1.11-.704c.326-.325.56-.705.704-1.11a3.016 3.016 0 0 0 0-2.022 2.985 2.985 0 0 0-.704-1.11Z"
            }
        }
    }
}

#[component]
pub fn LockOpenIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M18 1.5c2.9 0 5.25 2.35 5.25 5.25v3.75a.75.75 0 0 1-1.5 0V6.75a3.75 3.75 0 1 0-7.5 0v3a3 3 0 0 1 3 3v6.75a3 3 0 0 1-3 3H3.75a3 3 0 0 1-3-3v-6.75a3 3 0 0 1 3-3h9v-3c0-2.9 2.35-5.25 5.25-5.25Z"
            }
        }
    }
}

#[component]
pub fn ChatIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M5.337 21.718a6.707 6.707 0 0 1-.533-.074.75.75 0 0 1-.44-1.223 3.73 3.73 0 0 0 .814-1.686c.023-.115-.022-.317-.254-.543C3.274 16.587 2.25 14.41 2.25 12c0-5.03 4.428-9 9.75-9s9.75 3.97 9.75 9c0 5.03-4.428 9-9.75 9-.833 0-1.643-.097-2.417-.279a6.721 6.721 0 0 1-4.246.997Z"
            }
        }
    }
}

#[component]
pub fn ChatButtonRightLeftIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M4.913 2.658c2.075-.27 4.19-.408 6.337-.408 2.147 0 4.262.139 6.337.408 1.922.25 3.291 1.861 3.405 3.727a4.403 4.403 0 0 0-1.032-.211 50.89 50.89 0 0 0-8.42 0c-2.358.196-4.04 2.19-4.04 4.434v4.286a4.47 4.47 0 0 0 2.433 3.984L7.28 21.53A.75.75 0 0 1 6 21v-4.03a48.527 48.527 0 0 1-1.087-.128C2.905 16.58 1.5 14.833 1.5 12.862V6.638c0-1.97 1.405-3.718 3.413-3.979Z"
            }
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M15.75 7.5c-1.376 0-2.739.057-4.086.169C10.124 7.797 9 9.103 9 10.609v4.285c0 1.507 1.128 2.814 2.67 2.94 1.243.102 2.5.157 3.768.165l2.782 2.781a.75.75 0 0 0 1.28-.53v-2.39l.33-.026c1.542-.125 2.67-1.433 2.67-2.94v-4.286c0-1.505-1.125-2.811-2.664-2.94A49.392 49.392 0 0 0 15.75 7.5Z"
            }
        }
    }
}

#[component]
pub fn FlagIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M3 2.25a.75.75 0 0 1 .75.75v.54l1.838-.46a9.75 9.75 0 0 1 6.725.738l.108.054A8.25 8.25 0 0 0 18 4.524l3.11-.732a.75.75 0 0 1 .917.81 47.784 47.784 0 0 0 .005 10.337.75.75 0 0 1-.574.812l-3.114.733a9.75 9.75 0 0 1-6.594-.77l-.108-.054a8.25 8.25 0 0 0-5.69-.625l-2.202.55V21a.75.75 0 0 1-1.5 0V3A.75.75 0 0 1 3 2.25Z"
            }
        }
    }
}

#[component]
pub fn EyeSlashIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M3.53 2.47a.75.75 0 0 0-1.06 1.06l18 18a.75.75 0 1 0 1.06-1.06l-18-18ZM22.676 12.553a11.249 11.249 0 0 1-2.631 4.31l-3.099-3.099a5.25 5.25 0 0 0-6.71-6.71L7.759 4.577a11.217 11.217 0 0 1 4.242-.827c4.97 0 9.185 3.223 10.675 7.69.12.362.12.752 0 1.113Z"
            }
            path {
                d: "M15.75 12c0 .18-.013.357-.037.53l-4.244-4.243A3.75 3.75 0 0 1 15.75 12ZM12.53 15.713l-4.243-4.244a3.75 3.75 0 0 0 4.244 4.243Z"
            }
            path {
                d: "M6.75 12c0-.619.107-1.213.304-1.764l-3.1-3.1a11.25 11.25 0 0 0-2.63 4.31c-.12.362-.12.752 0 1.114 1.489 4.467 5.704 7.69 10.675 7.69 1.5 0 2.933-.294 4.242-.827l-2.477-2.477A5.25 5.25 0 0 1 6.75 12Z"
            }
        }
    }
}

#[component]
pub fn RefreshIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M4.755 10.059a7.5 7.5 0 0 1 12.548-3.364l1.903 1.903h-3.183a.75.75 0 1 0 0 1.5h4.992a.75.75 0 0 0 .75-.75V4.356a.75.75 0 0 0-1.5 0v3.18l-1.9-1.9A9 9 0 0 0 3.306 9.67a.75.75 0 1 0 1.45.388Zm15.408 3.352a.75.75 0 0 0-.919.53 7.5 7.5 0 0 1-12.548 3.364l-1.902-1.903h3.183a.75.75 0 0 0 0-1.5H2.984a.75.75 0 0 0-.75.75v4.992a.75.75 0 0 0 1.5 0v-3.18l1.9 1.9a9 9 0 0 0 15.059-4.035.75.75 0 0 0-.53-.918Z"
            }
        }
    }
}

#[component]
pub fn DiscordIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 -28.5 256 256",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "nonzero",
                d: "M216.856339,16.5966031 C200.285002,8.84328665 182.566144,3.2084988 164.041564,0 C161.766523,4.11318106 159.108624,9.64549908 157.276099,14.0464379 C137.583995,11.0849896 118.072967,11.0849896 98.7430163,14.0464379 C96.9108417,9.64549908 94.1925838,4.11318106 91.8971895,0 C73.3526068,3.2084988 55.6133949,8.86399117 39.0420583,16.6376612 C5.61752293,67.146514 -3.4433191,116.400813 1.08711069,164.955721 C23.2560196,181.510915 44.7403634,191.567697 65.8621325,198.148576 C71.0772151,190.971126 75.7283628,183.341335 79.7352139,175.300261 C72.104019,172.400575 64.7949724,168.822202 57.8887866,164.667963 C59.7209612,163.310589 61.5131304,161.891452 63.2445898,160.431257 C105.36741,180.133187 151.134928,180.133187 192.754523,160.431257 C194.506336,161.891452 196.298154,163.310589 198.110326,164.667963 C191.183787,168.842556 183.854737,172.420929 176.223542,175.320965 C180.230393,183.341335 184.861538,190.991831 190.096624,198.16893 C211.238746,191.588051 232.743023,181.531619 254.911949,164.955721 C260.227747,108.668201 245.831087,59.8662432 216.856339,16.5966031 Z M85.4738752,135.09489 C72.8290281,135.09489 62.4592217,123.290155 62.4592217,108.914901 C62.4592217,94.5396472 72.607595,82.7145587 85.4738752,82.7145587 C98.3405064,82.7145587 108.709962,94.5189427 108.488529,108.914901 C108.508531,123.290155 98.3405064,135.09489 85.4738752,135.09489 Z M170.525237,135.09489 C157.88039,135.09489 147.510584,123.290155 147.510584,108.914901 C147.510584,94.5396472 157.658606,82.7145587 170.525237,82.7145587 C183.391518,82.7145587 193.761324,94.5189427 193.539891,108.914901 C193.539891,123.290155 183.391518,135.09489 170.525237,135.09489 Z"
            }
        }
    }
}

#[component]
pub fn OttersecIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 23 23",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                d: "M16.509 20.7386C17.9619 20.1581 19.2679 19.2817 20.3377 18.1696C22.0596 16.3866 22.968 13.5267 22.968 13.5267C22.9383 13.8305 22.8908 14.1325 22.8256 14.4311C22.7319 14.8798 22.608 15.3221 22.4548 15.7552C22.2907 16.2401 22.0979 16.7155 21.8774 17.179C21.6203 17.7287 21.3163 18.2567 20.9683 18.7576C20.1403 19.8811 19.0887 20.8342 17.8748 21.5612C17.0299 22.031 16.1275 22.3972 15.1887 22.6514C12.3197 23.3924 6.14484 23.2148 2.46433 18.7297C-0.214746 15.4637 -0.0267768 11.7976 0.0226226 11.0969C0.05919 10.5751 0.406265 6.57916 3.84816 3.54337C4.85459 2.65826 6.01093 1.94634 7.2669 1.43861C7.26447 1.1033 7.38569 0.778075 7.60895 0.521103C7.8322 0.264137 8.14293 0.0921773 8.48549 0.0359769C8.82813 -0.0202235 9.18022 0.0429988 9.47898 0.214341C9.77765 0.385687 10.0035 0.653972 10.116 0.97123C10.9653 0.909327 11.6256 0.90561 11.9553 0.934711C12.1503 0.951418 12.3762 0.978656 12.3762 0.978656C12.8064 1.02327 13.231 1.10894 13.6438 1.23432C14.1611 1.37223 14.6547 1.58226 15.1091 1.8577C15.7904 2.28238 15.8725 2.59621 16.8137 3.16326C16.9151 3.22454 16.9991 3.27221 17.053 3.30195L17.4212 3.40716C17.5996 3.45794 17.6945 3.48395 17.776 3.50683C17.9293 3.55018 18.1474 3.61331 18.4214 3.69937C18.3553 3.82687 18.2995 3.93335 18.2585 4.00887C18.1751 4.16486 18.1109 4.28066 18.1109 4.28498C17.8107 4.84641 17.366 5.3239 16.8195 5.67166C16.2729 6.01941 14.424 5.79604 13.6438 5.67166C12.8637 5.54727 10.413 4.89967 8.07655 6.42502C5.26528 8.26046 5.31725 11.5692 5.33523 12.0583C5.35064 12.4904 5.49498 15.1752 7.8321 17.0162C9.55205 18.3707 11.4472 18.4573 12.1215 18.479C12.6309 18.4951 14.6908 18.5112 16.6764 17.179C17.5722 16.583 18.3177 15.8002 18.8577 14.8885C19.3615 14.0149 19.647 13.0399 19.6917 12.041C19.7133 11.3267 19.6047 10.6143 19.3709 9.93621C19.1399 9.34406 18.8323 8.78253 18.4554 8.26479C19.2509 10.6791 18.5196 12.66 18.5196 12.66C18.313 13.219 18.0429 13.9191 17.3847 14.6174C16.5308 15.5224 15.3529 15.9558 14.9122 16.1137C14.5749 16.2317 14.2299 16.3279 13.8793 16.4015C13.6095 16.4546 13.4043 16.477 13.1222 16.5078L13.1095 16.5092C12.8327 16.5417 12.5541 16.5563 12.2755 16.5532C12.2755 16.5532 12.8022 16.4541 13.3071 16.3273C14.3199 16.0644 15.2542 15.5749 16.0342 14.8985C16.4487 14.5339 17.1095 13.9383 17.4899 12.9479C17.7358 12.3077 17.7612 11.7724 17.7843 11.2847L17.7844 11.282C17.8172 10.5077 17.7263 9.73307 17.5149 8.98533C17.3236 8.30469 17.0361 7.65269 16.6604 7.04777L17.0658 7.08864C17.694 7.17174 18.3155 7.29585 18.9263 7.46005C19.5929 7.63956 20.1144 7.78443 20.7226 8.14102C20.9991 8.30318 21.2743 8.46476 21.5567 8.75999C22.0757 9.30416 22.2386 9.90465 22.3264 10.2458C22.6049 11.3248 22.3765 12.2329 22.1981 12.9077C21.9108 13.9941 21.4399 14.7412 21.1076 15.26C20.6268 16.0122 20.0443 16.6992 19.3754 17.3028C17.7337 18.7737 15.9047 19.3704 14.8526 19.6243C13.4283 19.9991 11.9332 20.0488 10.4856 19.7694C9.03799 19.49 7.67774 18.8893 6.51248 18.0147C4.92141 16.9004 4.15028 15.6512 3.80001 14.9722C2.7389 12.9157 2.81333 10.8041 2.85886 9.80563C2.87236 9.51649 2.88778 9.34015 2.88778 9.31718C2.5785 10.3429 2.08713 12.5369 2.88067 15.0607C3.09239 15.7348 3.56135 17.171 4.87654 18.5421C6.37968 20.109 8.10092 20.6636 9.04659 20.9564C10.1327 21.2926 13.164 22.0491 16.509 20.7386Z"
            }
        }
    }
}
#[component]
pub fn AppleIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 50 50",
            fill: "currentColor",
            class: "{class}",
            path {
                d: "M 44.527344 34.75 C 43.449219 37.144531 42.929688 38.214844 41.542969 40.328125 C 39.601563 43.28125 36.863281 46.96875 33.480469 46.992188 C 30.46875 47.019531 29.691406 45.027344 25.601563 45.0625 C 21.515625 45.082031 20.664063 47.03125 17.648438 47 C 14.261719 46.96875 11.671875 43.648438 9.730469 40.699219 C 4.300781 32.429688 3.726563 22.734375 7.082031 17.578125 C 9.457031 13.921875 13.210938 11.773438 16.738281 11.773438 C 20.332031 11.773438 22.589844 13.746094 25.558594 13.746094 C 28.441406 13.746094 30.195313 11.769531 34.351563 11.769531 C 37.492188 11.769531 40.8125 13.480469 43.1875 16.433594 C 35.421875 20.691406 36.683594 31.78125 44.527344 34.75 Z M 31.195313 8.46875 C 32.707031 6.527344 33.855469 3.789063 33.4375 1 C 30.972656 1.167969 28.089844 2.742188 26.40625 4.78125 C 24.878906 6.640625 23.613281 9.398438 24.105469 12.066406 C 26.796875 12ospatial 29.582031 10.546875 31.195313 8.46875 Z"
            }
        }
    }
}

#[component]
pub fn WindowsIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 50 50",
            fill: "currentColor",
            class: "{class}",
            path {
                d: "M19.852 7.761l-15 2.25C4.362 10.085 4 10.505 4 11v12c0 .553.448 1 1 1h15c.552 0 1-.447 1-1V8.75c0-.291-.127-.567-.348-.758C20.432 7.803 20.139 7.721 19.852 7.761zM45.652 4.242c-.22-.189-.512-.271-.801-.231l-21 3.15C23.362 7.235 23 7.655 23 8.15V23c0 .553.448 1 1 1h21c.552 0 1-.447 1-1V5C46 4.709 45.873 4.433 45.652 4.242zM20 26H5c-.552 0-1 .447-1 1v12c0 .495.362.915.852.989l15 2.25c.05.007.099.011.148.011.238 0 .47-.085.652-.242C20.873 41.817 21 41.541 21 41.25V27C21 26.447 20.552 26 20 26zM45 26H24c-.552 0-1 .447-1 1v14.85c0 .495.362.915.852.989l21 3.15C44.901 45.996 44.951 46 45 46c.238 0 .47-.085.652-.242C45.873 45.567 46 45.291 46 45V27C46 26.447 45.552 26 45 26z"
            }
        }
    }
}

#[component]
pub fn FuzzlandIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 1581.98 1665.76",
            fill: "currentColor",
            class: "{class}",
            polygon {
                points: "741.82 0 54.22 299.32 396.84 505.25 741.82 355.08 741.82 0"
            }
            polygon {
                points: "840.15 0 840.15 355.08 1185.13 505.25 1527.76 299.32 840.15 0"
            }
            path {
                d: "M1240.07,586.96c-8.44,283.2-158.75,543.34-399.92,692.08v386.72c444.73-278.66,722.66-759.86,741.83-1284.3l-341.91,205.5Z",
            }
            path {
                d: "M741.82,1279.04c-241.17-148.75-391.48-408.88-399.92-692.08L0,381.46c19.17,524.44,297.1,1005.64,741.82,1284.3v-386.72Z"
            }
        }
    }
}

#[component]
pub fn CogIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M11.078 2.25c-.917 0-1.699.663-1.85 1.567L9.05 4.889c-.02.12-.115.26-.297.348a7.493 7.493 0 0 0-.986.57c-.166.115-.334.126-.45.083L6.3 5.508a1.875 1.875 0 0 0-2.282.819l-.922 1.597a1.875 1.875 0 0 0 .432 2.385l.84.692c.095.078.17.229.154.43a7.598 7.598 0 0 0 0 1.139c.015.2-.059.352-.153.43l-.841.692a1.875 1.875 0 0 0-.432 2.385l.922 1.597a1.875 1.875 0 0 0 2.282.818l1.019-.382c.115-.043.283-.031.45.082.312.214.641.405.985.57.182.088.277.228.297.35l.178 1.071c.151.904.933 1.567 1.85 1.567h1.844c.916 0 1.699-.663 1.85-1.567l.178-1.072c.02-.12.114-.26.297-.349.344-.165.673-.356.985-.57.167-.114.335-.125.45-.082l1.02.382a1.875 1.875 0 0 0 2.28-.819l.923-1.597a1.875 1.875 0 0 0-.432-2.385l-.84-.692c-.095-.078-.17-.229-.154-.43a7.614 7.614 0 0 0 0-1.139c-.016-.2.059-.352.153-.43l.84-.692c.708-.582.891-1.59.433-2.385l-.922-1.597a1.875 1.875 0 0 0-2.282-.818l-1.02.382c-.114.043-.282.031-.449-.083a7.49 7.49 0 0 0-.985-.57c-.183-.087-.277-.227-.297-.348l-.179-1.072a1.875 1.875 0 0 0-1.85-1.567h-1.843ZM12 15.75a3.75 3.75 0 1 0 0-7.5 3.75 3.75 0 0 0 0 7.5Z"
            }
        }
    }
}

#[component]
pub fn NumberedListIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M7.491 5.992a.75.75 0 0 1 .75-.75h12a.75.75 0 1 1 0 1.5h-12a.75.75 0 0 1-.75-.75ZM7.49 11.995a.75.75 0 0 1 .75-.75h12a.75.75 0 0 1 0 1.5h-12a.75.75 0 0 1-.75-.75ZM7.491 17.994a.75.75 0 0 1 .75-.75h12a.75.75 0 1 1 0 1.5h-12a.75.75 0 0 1-.75-.75ZM2.24 3.745a.75.75 0 0 1 .75-.75h1.125a.75.75 0 0 1 .75.75v3h.375a.75.75 0 0 1 0 1.5H2.99a.75.75 0 0 1 0-1.5h.375v-2.25H2.99a.75.75 0 0 1-.75-.75ZM2.79 10.602a.75.75 0 0 1 0-1.06 1.875 1.875 0 1 1 2.652 2.651l-.55.55h.35a.75.75 0 0 1 0 1.5h-2.16a.75.75 0 0 1-.53-1.281l1.83-1.83a.375.375 0 0 0-.53-.53.75.75 0 0 1-1.062 0ZM2.24 15.745a.75.75 0 0 1 .75-.75h1.125a1.875 1.875 0 0 1 1.501 2.999 1.875 1.875 0 0 1-1.501 3H2.99a.75.75 0 0 1 0-1.501h1.125a.375.375 0 0 0 .036-.748H3.74a.75.75 0 0 1-.75-.75v-.002a.75.75 0 0 1 .75-.75h.411a.375.375 0 0 0-.036-.748H2.99a.75.75 0 0 1-.75-.75Z"
            }
        }
    }
}

#[component]
pub fn RepeatIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M12 5.25c1.213 0 2.415.046 3.605.135a3.256 3.256 0 0 1 3.01 3.01c.044.583.077 1.17.1 1.759L17.03 8.47a.75.75 0 1 0-1.06 1.06l3 3a.75.75 0 0 0 1.06 0l3-3a.75.75 0 0 0-1.06-1.06l-1.752 1.751c-.023-.65-.06-1.296-.108-1.939a4.756 4.756 0 0 0-4.392-4.392 49.422 49.422 0 0 0-7.436 0A4.756 4.756 0 0 0 3.89 8.282c-.017.224-.033.447-.046.672a.75.75 0 1 0 1.497.092c.013-.217.028-.434.044-.651a3.256 3.256 0 0 1 3.01-3.01c1.19-.09 2.392-.135 3.605-.135Zm-6.97 6.22a.75.75 0 0 0-1.06 0l-3 3a.75.75 0 1 0 1.06 1.06l1.752-1.751c.023.65.06 1.296.108 1.939a4.756 4.756 0 0 0 4.392 4.392 49.413 49.413 0 0 0 7.436 0 4.756 4.756 0 0 0 4.392-4.392c.017-.223.032-.447.046-.672a.75.75 0 0 0-1.497-.092c-.013.217-.028.434-.044.651a3.256 3.256 0 0 1-3.01 3.01 47.953 47.953 0 0 1-7.21 0 3.256 3.256 0 0 1-3.01-3.01 47.759 47.759 0 0 1-.1-1.759L6.97 15.53a.75.75 0 0 0 1.06-1.06l-3-3Z"
            }
        }
    }
}

#[component]
pub fn ReloadIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
        }
    }
}

#[component]
pub fn DexscreenIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 252 300",
            fill: "currentColor",
            fill_rule: "evenodd",
            class: "{class}",
            path {
                d: "M151.818 106.866c9.177-4.576 20.854-11.312 32.545-20.541 2.465 5.119 2.735 9.586 1.465 13.193-.9 2.542-2.596 4.753-4.826 6.512-2.415 1.901-5.431 3.285-8.765 4.033-6.326 1.425-13.712.593-20.419-3.197m1.591 46.886l12.148 7.017c-24.804 13.902-31.547 39.716-39.557 64.859-8.009-25.143-14.753-50.957-39.556-64.859l12.148-7.017a5.95 5.95 0 003.84-5.845c-1.113-23.547 5.245-33.96 13.821-40.498 3.076-2.342 6.434-3.518 9.747-3.518s6.671 1.176 9.748 3.518c8.576 6.538 14.934 16.951 13.821 40.498a5.95 5.95 0 003.84 5.845zM126 0c14.042.377 28.119 3.103 40.336 8.406 8.46 3.677 16.354 8.534 23.502 14.342 3.228 2.622 5.886 5.155 8.814 8.071 7.897.273 19.438-8.5 24.796-16.709-9.221 30.23-51.299 65.929-80.43 79.589-.012-.005-.02-.012-.029-.018-5.228-3.992-11.108-5.988-16.989-5.988s-11.76 1.996-16.988 5.988c-.009.005-.017.014-.029.018-29.132-13.66-71.209-49.359-80.43-79.589 5.357 8.209 16.898 16.982 24.795 16.709 2.929-2.915 5.587-5.449 8.814-8.071C69.31 16.94 77.204 12.083 85.664 8.406 97.882 3.103 111.959.377 126 0m-25.818 106.866c-9.176-4.576-20.854-11.312-32.544-20.541-2.465 5.119-2.735 9.586-1.466 13.193.901 2.542 2.597 4.753 4.826 6.512 2.416 1.901 5.432 3.285 8.766 4.033 6.326 1.425 13.711.593 20.418-3.197"
            }
            path {
                d: "M197.167 75.016c6.436-6.495 12.107-13.684 16.667-20.099l2.316 4.359c7.456 14.917 11.33 29.774 11.33 46.494l-.016 26.532.14 13.754c.54 33.766 7.846 67.929 24.396 99.193l-34.627-27.922-24.501 39.759-25.74-24.231L126 299.604l-41.132-66.748-25.739 24.231-24.501-39.759L0 245.25c16.55-31.264 23.856-65.427 24.397-99.193l.14-13.754-.016-26.532c0-16.721 3.873-31.578 11.331-46.494l2.315-4.359c4.56 6.415 10.23 13.603 16.667 20.099l-2.01 4.175c-3.905 8.109-5.198 17.176-2.156 25.799 1.961 5.554 5.54 10.317 10.154 13.953 4.48 3.531 9.782 5.911 15.333 7.161 3.616.814 7.3 1.149 10.96 1.035-.854 4.841-1.227 9.862-1.251 14.978L53.2 160.984l25.206 14.129a41.926 41.926 0 015.734 3.869c20.781 18.658 33.275 73.855 41.861 100.816 8.587-26.961 21.08-82.158 41.862-100.816a41.865 41.865 0 015.734-3.869l25.206-14.129-32.665-18.866c-.024-5.116-.397-10.137-1.251-14.978 3.66.114 7.344-.221 10.96-1.035 5.551-1.25 10.854-3.63 15.333-7.161 4.613-3.636 8.193-8.399 10.153-13.953 3.043-8.623 1.749-17.689-2.155-25.799l-2.01-4.175z"
            }
        }
    }
}

#[component]
pub fn BulbIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.5",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M12 18v-5.25m0 0a6.01 6.01 0 0 0 1.5-.189m-1.5.189a6.01 6.01 0 0 1-1.5-.189m3.75 7.478a12.06 12.06 0 0 1-4.5 0m3.75 2.383a14.406 14.406 0 0 1-3 0M14.25 18v-.192c0-.983.658-1.823 1.508-2.316a7.5 7.5 0 1 0-7.517 0c.85.493 1.509 1.333 1.509 2.316V18"
            }
        }
    }
}

#[component]
pub fn BookIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                d: "M11.25 4.533A9.707 9.707 0 0 0 6 3a9.735 9.735 0 0 0-3.25.555.75.75 0 0 0-.5.707v14.25a.75.75 0 0 0 1 .707A8.237 8.237 0 0 1 6 18.75c1.995 0 3.823.707 5.25 1.886V4.533ZM12.75 20.636A8.214 8.214 0 0 1 18 18.75c.966 0 1.89.166 2.75.47a.75.75 0 0 0 1-.708V4.262a.75.75 0 0 0-.5-.707A9.735 9.735 0 0 0 18 3a9.707 9.707 0 0 0-5.25 1.533v16.103Z"
            }
        }
    }
}

#[component]
pub fn QueueListIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                d: "M5.625 3.75a2.625 2.625 0 1 0 0 5.25h12.75a2.625 2.625 0 0 0 0-5.25H5.625ZM3.75 11.25a.75.75 0 0 0 0 1.5h16.5a.75.75 0 0 0 0-1.5H3.75ZM3 15.75a.75.75 0 0 1 .75-.75h16.5a.75.75 0 0 1 0 1.5H3.75a.75.75 0 0 1-.75-.75ZM3.75 18.75a.75.75 0 0 0 0 1.5h16.5a.75.75 0 0 0 0-1.5H3.75Z"
            }
        }
    }
}

#[component]
pub fn QuestionCircleIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.5",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M9.879 7.519c1.171-1.025 3.071-1.025 4.242 0 1.172 1.025 1.172 2.687 0 3.712-.203.179-.43.326-.67.442-.745.361-1.45.999-1.45 1.827v.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Zm-9 5.25h.008v.008H12v-.008Z"
            }
        }
    }
}

#[component]
pub fn QuestionCircleSolidIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M2.25 12c0-5.385 4.365-9.75 9.75-9.75s9.75 4.365 9.75 9.75-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12Zm11.378-3.917c-.89-.777-2.366-.777-3.255 0a.75.75 0 0 1-.988-1.129c1.454-1.272 3.776-1.272 5.23 0 1.513 1.324 1.513 3.518 0 4.842a3.75 3.75 0 0 1-.837.552c-.676.328-1.028.774-1.028 1.152v.75a.75.75 0 0 1-1.5 0v-.75c0-1.279 1.06-2.107 1.875-2.502.182-.088.351-.199.503-.331.83-.727.83-1.857 0-2.584ZM12 18a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5Z"
            }
        }
    }
}

#[component]
pub fn ChevronDownIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2.5",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "m19.5 8.25-7.5 7.5-7.5-7.5"
            }
        }
    }
}

#[component]
pub fn ChevronLeftIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 26 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "3",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M15.75 19.5 8.25 12l7.5-7.5"
            }
        }
    }
}

#[component]
pub fn ChevronDoubleLeftIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 26 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "3",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "m18.75 4.5-7.5 7.5 7.5 7.5m-6-15L5.25 12l7.5 7.5"
            }
        }
    }
}

#[component]
pub fn ChevronRightIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 26 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "3",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M8.25 4.5l7.5 7.5-7.5 7.5"
            }
        }
    }
}

#[component]
pub fn ArrowLeftIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2.5",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M10.5 19.5L3 12m0 0l7.5-7.5M3 12h18"
            }
        }
    }
}

#[component]
pub fn KeyIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.5",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M15.75 5.25a3 3 0 013 3m3 0a6 6 0 01-7.029 5.912c-.563-.097-1.159.026-1.563.43L10.5 17.25H8.25v2.25H6v2.25H2.25v-2.818c0-.597.237-1.17.659-1.591l6.499-6.499c.404-.404.527-1 .43-1.563A6 6 0 1121.75 8.25z"
            }
        }
    }
}

#[component]
pub fn ClockIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M12 6v6h4.5m4.5 0a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"
            }
        }
    }
}

#[component]
pub fn ClockSolidIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M12 2.25c-5.385 0-9.75 4.365-9.75 9.75s4.365 9.75 9.75 9.75 9.75-4.365 9.75-9.75S17.385 2.25 12 2.25ZM12.75 6a.75.75 0 0 0-1.5 0v6c0 .414.336.75.75.75h4.5a.75.75 0 0 0 0-1.5h-3.75V6Z"
            }
        }
    }
}

#[component]
pub fn StarIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.5",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M11.48 3.499a.562.562 0 0 1 1.04 0l2.125 5.111a.563.563 0 0 0 .475.345l5.518.442c.499.04.701.663.321.988l-4.204 3.602a.563.563 0 0 0-.182.557l1.285 5.385a.562.562 0 0 1-.84.61l-4.725-2.885a.562.562 0 0 0-.586 0L6.982 20.54a.562.562 0 0 1-.84-.61l1.285-5.386a.562.562 0 0 0-.182-.557l-4.204-3.602a.562.562 0 0 1 .321-.988l5.518-.442a.563.563 0 0 0 .475-.345L11.48 3.5Z"
            }
        }
    }
}

#[component]
pub fn TrophyIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.5",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M16.5 18.75h-9m9 0a3 3 0 0 1 3 3h-15a3 3 0 0 1 3-3m9 0v-3.375c0-.621-.503-1.125-1.125-1.125h-.871M7.5 18.75v-3.375c0-.621.504-1.125 1.125-1.125h.872m5.007 0H9.497m5.007 0a7.454 7.454 0 0 1-.982-3.172M9.497 14.25a7.454 7.454 0 0 0 .981-3.172M5.25 4.236c-.982.143-1.954.317-2.916.52A6.003 6.003 0 0 0 7.73 9.728M5.25 4.236V4.5c0 2.108.966 3.99 2.48 5.228M5.25 4.236V2.721C7.456 2.41 9.71 2.25 12 2.25c2.291 0 4.545.16 6.75.47v1.516M7.73 9.728a6.726 6.726 0 0 0 2.748 1.35m8.272-6.842V4.5c0 2.108-.966 3.99-2.48 5.228m2.48-5.492a46.32 46.32 0 0 1 2.916.52 6.003 6.003 0 0 1-5.395 4.972m0 0a6.726 6.726 0 0 1-2.749 1.35m0 0a6.772 6.772 0 0 1-3.044 0"
            }
        }
    }
}

#[component]
pub fn TrophySolidIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M5.166 2.621v.858c-1.035.148-2.059.33-3.071.543a.75.75 0 0 0-.584.859 6.753 6.753 0 0 0 6.138 5.6 6.73 6.73 0 0 0 2.743 1.346A6.707 6.707 0 0 1 9.279 15H8.54c-1.036 0-1.875.84-1.875 1.875V19.5h-.75a2.25 2.25 0 0 0-2.25 2.25c0 .414.336.75.75.75h15a.75.75 0 0 0 .75-.75 2.25 2.25 0 0 0-2.25-2.25h-.75v-2.625c0-1.036-.84-1.875-1.875-1.875h-.739a6.706 6.706 0 0 1-1.112-3.173 6.73 6.73 0 0 0 2.743-1.347 6.753 6.753 0 0 0 6.139-5.6.75.75 0 0 0-.585-.858 47.077 47.077 0 0 0-3.07-.543V2.62a.75.75 0 0 0-.658-.744 49.22 49.22 0 0 0-6.093-.377c-2.063 0-4.096.128-6.093.377a.75.75 0 0 0-.657.744Zm0 2.629c0 1.196.312 2.32.857 3.294A5.266 5.266 0 0 1 3.16 5.337a45.6 45.6 0 0 1 2.006-.343v.256Zm13.5 0v-.256c.674.1 1.343.214 2.006.343a5.265 5.265 0 0 1-2.863 3.207 6.72 6.72 0 0 0 .857-3.294Z"
            }
        }
    }
}

#[component]
pub fn SparklesIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.5",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M9.813 15.904 9 18.75l-.813-2.846a4.5 4.5 0 0 0-3.09-3.09L2.25 12l2.846-.813a4.5 4.5 0 0 0 3.09-3.09L9 5.25l.813 2.846a4.5 4.5 0 0 0 3.09 3.09L15.75 12l-2.846.813a4.5 4.5 0 0 0-3.09 3.09ZM18.259 8.715 18 9.75l-.259-1.035a3.375 3.375 0 0 0-2.455-2.456L14.25 6l1.036-.259a3.375 3.375 0 0 0 2.455-2.456L18 2.25l.259 1.035a3.375 3.375 0 0 0 2.456 2.456L21.75 6l-1.035.259a3.375 3.375 0 0 0-2.456 2.456ZM16.894 20.567 16.5 21.75l-.394-1.183a2.25 2.25 0 0 0-1.423-1.423L13.5 18.75l1.183-.394a2.25 2.25 0 0 0 1.423-1.423l.394-1.183.394 1.183a2.25 2.25 0 0 0 1.423 1.423l1.183.394-1.183.394a2.25 2.25 0 0 0-1.423 1.423Z"
            }
        }
    }
}

#[component]
pub fn SparklesSolidIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M9 4.5a.75.75 0 0 1 .721.544l.813 2.846a3.75 3.75 0 0 0 2.576 2.576l2.846.813a.75.75 0 0 1 0 1.442l-2.846.813a3.75 3.75 0 0 0-2.576 2.576l-.813 2.846a.75.75 0 0 1-1.442 0l-.813-2.846a3.75 3.75 0 0 0-2.576-2.576l-2.846-.813a.75.75 0 0 1 0-1.442l2.846-.813A3.75 3.75 0 0 0 7.466 7.89l.813-2.846A.75.75 0 0 1 9 4.5ZM18 1.5a.75.75 0 0 1 .728.568l.258 1.036c.236.94.97 1.674 1.91 1.91l1.036.258a.75.75 0 0 1 0 1.456l-1.036.258c-.94.236-1.674.97-1.91 1.91l-.258 1.036a.75.75 0 0 1-1.456 0l-.258-1.036a2.625 2.625 0 0 0-1.91-1.91l-1.036-.258a.75.75 0 0 1 0-1.456l1.036-.258a2.625 2.625 0 0 0 1.91-1.91l.258-1.036A.75.75 0 0 1 18 1.5ZM16.5 15a.75.75 0 0 1 .712.513l.394 1.183c.15.447.5.799.948.948l1.183.395a.75.75 0 0 1 0 1.422l-1.183.395c-.447.15-.799.5-.948.948l-.395 1.183a.75.75 0 0 1-1.422 0l-.395-1.183a1.5 1.5 0 0 0-.948-.948l-1.183-.395a.75.75 0 0 1 0-1.422l1.183-.395c.447-.15.799-.5.948-.948l.395-1.183A.75.75 0 0 1 16.5 15Z"
            }
        }
    }
}

#[component]
pub fn ArrowRightIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2.5",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M13.5 4.5L21 12m0 0l-7.5 7.5M21 12H3"
            }
        }
    }
}

#[component]
pub fn MarketArrowIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                d: "M21 1C22.1046 1 23 1.89543 23 3V21C23 22.1046 22.1046 23 21 23H3C1.89543 23 1 22.1046 1 21V3C1 1.89543 1.89543 1 3 1H21ZM16.1367 7.6709L17.1162 8.60547L13.0459 12.418L9.87988 9.4707L4 14.9238L5.16797 16L9.87988 11.623L13.0459 14.6055L18.2852 9.68262L19.2646 10.6357L20 7L16.1367 7.6709Z"
            }
        }
    }
}

#[component]
pub fn MarketArrowOutlineIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                d: "M21.2041 1.01074C22.2128 1.113 23 1.96435 23 3V21L22.9893 21.2041C22.8938 22.1457 22.1457 22.8938 21.2041 22.9893L21 23H3L2.7959 22.9893C1.85435 22.8938 1.1062 22.1457 1.01074 21.2041L1 21V3C1 1.96435 1.78722 1.113 2.7959 1.01074L3 1H21L21.2041 1.01074ZM3 2.5C2.72386 2.5 2.5 2.72386 2.5 3V21C2.5 21.2761 2.72386 21.5 3 21.5H21C21.2761 21.5 21.5 21.2761 21.5 21V3C21.5 2.72386 21.2761 2.5 21 2.5H3ZM19.2646 10.6357L18.2852 9.68262L13.0459 14.6055L9.87988 11.623L5.16797 16L4 14.9238L9.87988 9.4707L13.0459 12.418L17.1162 8.60547L16.1367 7.6709L20 7L19.2646 10.6357Z"
            }
        }
    }
}

#[component]
pub fn QuestionOutlineIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "1.5",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M9.879 7.519c1.171-1.025 3.071-1.025 4.242 0 1.172 1.025 1.172 2.687 0 3.712-.203.179-.43.326-.67.442-.745.361-1.45.999-1.45 1.827v.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Zm-9 5.25h.008v.008H12v-.008Z"
            }
        }
    }
}

#[component]
pub fn QuestionIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M2.25 12c0-5.385 4.365-9.75 9.75-9.75s9.75 4.365 9.75 9.75-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12Zm11.378-3.917c-.89-.777-2.366-.777-3.255 0a.75.75 0 0 1-.988-1.129c1.454-1.272 3.776-1.272 5.23 0 1.513 1.324 1.513 3.518 0 4.842a3.75 3.75 0 0 1-.837.552c-.676.328-1.028.774-1.028 1.152v.75a.75.75 0 0 1-1.5 0v-.75c0-1.279 1.06-2.107 1.875-2.502.182-.088.351-.199.503-.331.83-.727.83-1.857 0-2.584ZM12 18a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5Z"
            }
        }
    }
}

#[component]
pub fn SquaresWinnerIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            // Top-right: solid
            path {
                d: "M12.75 6a3 3 0 0 1 3-3H18a3 3 0 0 1 3 3v2.25a3 3 0 0 1-3 3h-2.25a3 3 0 0 1-3-3V6Z",
            }
            // Top-left, bottom-left, bottom-right: dimmed
            path {
                opacity: "0.25",
                d: "M3 6a3 3 0 0 1 3-3h2.25a3 3 0 0 1 3 3v2.25a3 3 0 0 1-3 3H6a3 3 0 0 1-3-3V6ZM3 15.75a3 3 0 0 1 3-3h2.25a3 3 0 0 1 3 3V18a3 3 0 0 1-3 3H6a3 3 0 0 1-3-3v-2.25Zm9.75 0a3 3 0 0 1 3-3H18a3 3 0 0 1 3 3V18a3 3 0 0 1-3 3h-2.25a3 3 0 0 1-3-3v-2.25Z",
            }
        }
    }
}

#[component]
pub fn CrownIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            // Crown body: left tip, left valley, center peak, right valley, right tip, bottom right, bottom left
            path {
                d: "M2 6.5 L6.5 11 L9 6.5 L12 11 L15 6.5 L17.5 11 L22 6.5 L19.5 17.5 L4.5 17.5 Z",
            }
            // Crown band
            rect {
                x: "4.5",
                y: "18.5",
                width: "15",
                height: "2.5",
                rx: "1",
            }
        }
    }
}

#[component]
pub fn AutoReloadIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or("".to_string());
    rsx! {
        svg {
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2.5",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0 3.181 3.183a8.25 8.25 0 0 0 13.803-3.7M4.031 9.865a8.25 8.25 0 0 1 13.803-3.7l3.181 3.182m0-4.991v4.99",
            }
        }
    }
}
