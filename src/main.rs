use std::rc::Rc;

use web_sys::SvgsvgElement;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct ViewBoxCenter((f32, f32));

impl ViewBoxCenter {
    fn min_x(&self) -> f32 {
        self.0 .0 - 50.0
    }

    fn min_y(&self) -> f32 {
        self.0 .1 - 50.0
    }

    fn to_svg_prop(&self) -> String {
        format!("{} {} 100 100", self.min_x(), self.min_y(),)
    }
}

type Pan = (f32, f32);

impl Reducible for ViewBoxCenter {
    type Action = Pan;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        Self((self.0 .0 + action.0, self.0 .1 + action.1)).into()
    }
}

fn calc_pointer_coords(s: &SvgsvgElement, e: &PointerEvent) -> (f32, f32) {
    let point = s.create_svg_point();
    point.set_x(e.client_x() as f32);
    point.set_y(e.client_y() as f32);
    let x = point.matrix_transform(&s.get_screen_ctm().unwrap().inverse().unwrap());
    (x.x(), x.y())
}

#[function_component]
pub fn App() -> Html {
    let panning_origin = use_state(|| None::<(f32, f32)>);

    let onpointerdown = use_callback((), {
        let panning_origin = panning_origin.clone();
        move |e: PointerEvent, _| {
            if let Some(s) = e.target_dyn_into::<SvgsvgElement>() {
                let c = calc_pointer_coords(&s, &e);
                panning_origin.set(Some(c));
            }
        }
    });

    let view_box = use_reducer(|| ViewBoxCenter((50.0, 50.0)));

    let onpointerup = use_callback((), {
        let panning_origin = panning_origin.clone();
        move |_, _| panning_origin.set(None)
    });

    let onpointermove = use_callback(panning_origin.clone(), {
        let view_box = view_box.dispatcher();
        move |e: PointerEvent, panning_origin| {
            if let Some(o) = **panning_origin {
                let c = calc_pointer_coords(&e.target_dyn_into::<SvgsvgElement>().unwrap(), &e);
                view_box.dispatch((o.0 - c.0, o.1 - c.1));
            }
        }
    });

    html! {

        <>
                <svg {onpointerdown} {onpointermove} {onpointerup} id="my-svg" xmlns="http://www.w3.org/2000/svg" width="300px" height="300px" viewBox={view_box.to_svg_prop()} style="background-color:white; border: 5px solid black;" >
        <circle cx="50" cy="50" r="20" fill="gray"/>
                </svg>

                    // this prevents the touch from panning the whole body
                <script>{"const svg = document.getElementById(\"my-svg\");svg.addEventListener(\"touchmove\", e => e.preventDefault());"}</script>
        </>

    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
