use dioxus::prelude::*;

/// A small inline SVG sparkline chart with hover crosshair and tooltip.
#[component]
pub fn Sparkline(
    data: Vec<f64>,
    labels: Vec<String>,
    id: String,
    #[props(default = 120.0)] width: f64,
    #[props(default = 32.0)] height: f64,
) -> Element {
    let mut hovered_index: Signal<Option<usize>> = use_signal(|| None);

    if data.len() < 2 {
        return rsx! {};
    }

    let points = normalize_points(&data, width, height);
    let line_points = points_to_polyline(&points);
    let area = area_path(&points, width, height);
    let gradient_id = format!("sf-{}", id);
    let n = data.len();
    let segment_width = width / n as f64;

    let mut container_ref: Signal<Option<std::rc::Rc<MountedData>>> = use_signal(|| None);

    let resolve_index = move |client_x: f64| {
        spawn(async move {
            let Some(el) = container_ref() else { return };
            let Ok(rect) = el.get_client_rect().await else { return };
            let relative_x = client_x - rect.origin.x;
            let idx = ((relative_x / width) * n as f64).floor().max(0.0) as usize;
            let idx = idx.min(n - 1);
            hovered_index.set(Some(idx));
        });
    };

    rsx! {
        div {
            class: "relative flex-shrink-0",
            style: "width: {width}px; height: {height}px; overflow: visible; touch-action: none;",
            onmounted: move |cx| container_ref.set(Some(cx.data())),
            onmouseleave: move |_| hovered_index.set(None),
            ontouchstart: move |e: Event<TouchData>| {
                if let Some(touch) = e.touches().first() {
                    resolve_index(touch.client_coordinates().x);
                }
            },
            ontouchmove: move |e: Event<TouchData>| {
                if let Some(touch) = e.touches().first() {
                    resolve_index(touch.client_coordinates().x);
                }
            },
            ontouchend: move |_| hovered_index.set(None),

            svg {
                width: "{width}",
                height: "{height}",
                view_box: "0 0 {width} {height}",
                style: "overflow: visible; animation: sparkline-fade 0.3s ease-out both;",

                // Gradient fill
                defs {
                    linearGradient {
                        id: "{gradient_id}",
                        x1: "0",
                        y1: "0",
                        x2: "0",
                        y2: "1",
                        stop { offset: "0%", stop_color: "#3b82f6", stop_opacity: "0.3" }
                        stop { offset: "100%", stop_color: "#3b82f6", stop_opacity: "0.0" }
                    }
                }

                // Area fill
                path {
                    d: "{area}",
                    fill: "url(#{gradient_id})",
                    pointer_events: "none",
                }

                // Line
                polyline {
                    points: "{line_points}",
                    fill: "none",
                    stroke: "#3b82f6",
                    stroke_width: "1.5",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    pointer_events: "none",
                }

                // Hit-target rects
                for i in 0..n {
                    rect {
                        key: "{i}",
                        x: "{i as f64 * segment_width}",
                        y: "0",
                        width: "{segment_width}",
                        height: "{height}",
                        fill: "transparent",
                        onmouseenter: move |_| hovered_index.set(Some(i)),
                    }
                }

                // Crosshair + dot on hover
                if let Some(idx) = hovered_index() {
                    if idx < points.len() {
                        {
                            let (cx, cy) = points[idx];
                            rsx! {
                                line {
                                    x1: "{cx}",
                                    y1: "0",
                                    x2: "{cx}",
                                    y2: "{height}",
                                    stroke: "#3b82f6",
                                    stroke_width: "1",
                                    opacity: "0.5",
                                    stroke_dasharray: "2 2",
                                    pointer_events: "none",
                                }
                                circle {
                                    cx: "{cx}",
                                    cy: "{cy}",
                                    r: "3",
                                    fill: "#3b82f6",
                                    stroke: "white",
                                    stroke_width: "1",
                                    pointer_events: "none",
                                }
                            }
                        }
                    }
                }
            }

            // Tooltip (rendered as HTML overlay, positioned above chart)
            if let Some(idx) = hovered_index() {
                if idx < labels.len() {
                    {
                        let (cx, _cy) = points[idx];
                        // Position tooltip centered on crosshair, above the chart
                        let tooltip_style = format!(
                            "left: {:.1}px; bottom: {:.1}px; transform: translateX(-50%);",
                            cx,
                            height + 8.0
                        );
                        rsx! {
                            div {
                                class: "absolute whitespace-nowrap z-[200] px-3 py-2 rounded-lg bg-surface-elevated border-2 border-elements-lowEmphasis text-sm text-elements-highEmphasis shadow-xl pointer-events-none",
                                style: "{tooltip_style}",
                                "{labels[idx]}"
                            }
                        }
                    }
                }
            }
        }

        style {
            r#"
            @keyframes sparkline-fade {{
                from {{ opacity: 0; }}
                to {{ opacity: 1; }}
            }}
            "#
        }
    }
}

fn normalize_points(data: &[f64], w: f64, h: f64) -> Vec<(f64, f64)> {
    let n = data.len();
    if n < 2 {
        return vec![];
    }
    let min = data.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = if (max - min).abs() < f64::EPSILON {
        1.0
    } else {
        max - min
    };
    let pad = h * 0.05;
    let usable_h = h - pad * 2.0;

    data.iter()
        .enumerate()
        .map(|(i, &v)| {
            let x = i as f64 / (n - 1) as f64 * w;
            let y = pad + usable_h - (v - min) / range * usable_h;
            (x, y)
        })
        .collect()
}

fn points_to_polyline(pts: &[(f64, f64)]) -> String {
    pts.iter()
        .map(|(x, y)| format!("{:.2},{:.2}", x, y))
        .collect::<Vec<_>>()
        .join(" ")
}

fn area_path(pts: &[(f64, f64)], w: f64, h: f64) -> String {
    if pts.is_empty() {
        return String::new();
    }
    let mut d = format!("M {:.2},{:.2}", pts[0].0, pts[0].1);
    for &(x, y) in &pts[1..] {
        d.push_str(&format!(" L {:.2},{:.2}", x, y));
    }
    let last_x = pts.last().unwrap().0;
    d.push_str(&format!(" L {:.2},{:.2} L {:.2},{:.2} Z", last_x, h, pts[0].0, h));
    d
}
