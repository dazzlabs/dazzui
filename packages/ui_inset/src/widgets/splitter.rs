use super::ValueChanged;
use crate::Theme;
use inset_foundation::{App, Handle};
use inset_painting::{Axis, EdgeInsetsGeometry};
use inset_rendering::HitTestBehavior;
use inset_services::{HardwareKeyboard, KeyEvent, LogicalKeyboardKey};
use inset_widgets::*;
use std::rc::Rc;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SplitterOrientation {
    #[default]
    Horizontal,
    Vertical,
}
#[derive(Debug, Clone)]
pub struct SplitterPanel {
    pub child: WidgetRef,
    pub default_size: f64,
    pub min_size: f64,
    pub max_size: f64,
}
impl SplitterPanel {
    pub fn new<K>(child: impl IntoWidget<K>) -> Self {
        Self {
            child: child.into_widget(),
            default_size: 1.0,
            min_size: 0.0,
            max_size: 100.0,
        }
    }
}
#[derive(Clone)]
pub struct Splitter {
    pub panels: Vec<SplitterPanel>,
    pub orientation: SplitterOrientation,
    pub on_sizes_changed: Option<ValueChanged<Vec<f64>>>,
}
impl std::fmt::Debug for Splitter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Splitter")
            .field("panels", &self.panels)
            .finish()
    }
}
impl Splitter {
    pub fn new(panels: Vec<SplitterPanel>) -> Self {
        assert!(panels.len() >= 2);
        Self {
            panels,
            orientation: SplitterOrientation::Horizontal,
            on_sizes_changed: None,
        }
    }
    pub fn initial_sizes(&self) -> Vec<f64> {
        assert!(self.panels.iter().all(|p| p.default_size.is_finite()
            && p.default_size >= 0.0
            && p.min_size >= 0.0
            && p.max_size <= 100.0
            && p.min_size <= p.max_size));
        let total: f64 = self.panels.iter().map(|p| p.default_size).sum();
        assert!(total > 0.0);
        assert!(
            self.panels.iter().map(|p| p.min_size).sum::<f64>() <= 100.0
                && self.panels.iter().map(|p| p.max_size).sum::<f64>() >= 100.0
        );
        let mut sizes: Vec<_> = self
            .panels
            .iter()
            .map(|p| (p.default_size / total * 100.0).clamp(p.min_size, p.max_size))
            .collect();
        for _ in 0..self.panels.len() + 1 {
            let delta = 100.0 - sizes.iter().sum::<f64>();
            if delta.abs() < 1e-9 {
                break;
            }
            let free: Vec<_> = self
                .panels
                .iter()
                .enumerate()
                .filter(|(i, p)| {
                    if delta > 0.0 {
                        sizes[*i] < p.max_size
                    } else {
                        sizes[*i] > p.min_size
                    }
                })
                .map(|(i, _)| i)
                .collect();
            if free.is_empty() {
                break;
            }
            let share = delta / free.len() as f64;
            for i in free {
                sizes[i] =
                    (sizes[i] + share).clamp(self.panels[i].min_size, self.panels[i].max_size);
            }
        }
        sizes
    }
    pub fn resize_pair(&self, sizes: &mut [f64], index: usize, leading: f64) {
        let pair = sizes[index] + sizes[index + 1];
        let lead = &self.panels[index];
        let trail = &self.panels[index + 1];
        let lo = lead.min_size.max(pair - trail.max_size);
        let hi = lead.max_size.min(pair - trail.min_size);
        sizes[index] = leading.clamp(lo, hi);
        sizes[index + 1] = pair - sizes[index];
    }
}
impl StatefulWidget for Splitter {
    type State = SplitterState;
    fn create_state(&self) -> Self::State {
        SplitterState {
            state: StateData::new(),
            sizes: self.initial_sizes(),
        }
    }
}
pub struct SplitterState {
    state: StateData<Splitter>,
    sizes: Vec<f64>,
}
impl SplitterState {
    fn resize(self: Handle<Self>, app: &mut App, index: usize, delta: f64) {
        let w = self.widget(app).clone();
        self.set_state(app, |s| {
            let next = s.sizes[index] + delta;
            w.resize_pair(&mut s.sizes, index, next);
        });
        if let Some(callback) = w.on_sizes_changed {
            let sizes = app.get(self).sizes.clone();
            callback(app, sizes);
        }
    }
}
impl State for SplitterState {
    type Widget = Splitter;
    inset_widgets::state_accessors!();
    fn did_update_widget(self: Handle<Self>, app: &mut App, old: &Splitter) {
        if old.panels.len() != self.widget(app).panels.len() {
            let sizes = self.widget(app).initial_sizes();
            app.get_mut(self).sizes = sizes;
        }
    }
    fn build(self: Handle<Self>, app: &mut App, _: BuildContext) -> WidgetRef {
        let w = self.widget(app).clone();
        let sizes = app.get(self).sizes.clone();
        LayoutBuilder::new(move |app, context, constraints| {
            let v = Theme::of(app, context);
            let horizontal = w.orientation == SplitterOrientation::Horizontal;
            let whole = if horizontal {
                constraints.max_width
            } else {
                constraints.max_height
            };
            assert!(
                whole.is_finite(),
                "Splitter requires bounded constraints along its axis"
            );
            let target = v.spacing_2;
            let run = (whole - (w.panels.len() - 1) as f64 * target).max(0.0);
            let mut children = vec![];
            for (i, panel) in w.panels.iter().enumerate() {
                let box_ = SizedBox::new().child(panel.child.clone());
                children.push(if horizontal {
                    box_.width(run * sizes[i] / 100.0).into_widget()
                } else {
                    box_.height(run * sizes[i] / 100.0).into_widget()
                });
                if i + 1 < w.panels.len() {
                    let rule = Container::new().color(v.color_border);
                    let rule = if horizontal {
                        rule.width(v.stroke_hairline)
                    } else {
                        rule.height(v.stroke_hairline)
                    };
                    let pad = (target - v.stroke_hairline) / 2.0;
                    let grip = Padding::new(if horizontal {
                        EdgeInsetsGeometry::symmetric(0.0, pad)
                    } else {
                        EdgeInsetsGeometry::symmetric(pad, 0.0)
                    })
                    .child(rule);
                    let listener = inset_widgets::Listener::new()
                        .behavior(HitTestBehavior::Opaque)
                        .on_pointer_move(Rc::new(move |app, event| {
                            if event.buttons & 1 != 0 && run > 0.0 {
                                let delta = if horizontal {
                                    event.delta.dx()
                                } else {
                                    event.delta.dy()
                                };
                                self.resize(app, i, delta / run * 100.0);
                            }
                        }))
                        .child(grip);
                    children.push(
                        Focus::new(listener)
                            .on_key_event(Rc::new(move |app, _, event| {
                                if matches!(event, KeyEvent::Up(_)) {
                                    return KeyEventResult::Ignored;
                                }
                                let key = event.logical_key();
                                let shift = HardwareKeyboard::instance(app).is_shift_pressed(app);
                                let step = if shift { 10.0 } else { 1.0 };
                                let delta = if key
                                    == if horizontal {
                                        LogicalKeyboardKey::ARROW_RIGHT
                                    } else {
                                        LogicalKeyboardKey::ARROW_DOWN
                                    } {
                                    Some(step)
                                } else if key
                                    == if horizontal {
                                        LogicalKeyboardKey::ARROW_LEFT
                                    } else {
                                        LogicalKeyboardKey::ARROW_UP
                                    }
                                {
                                    Some(-step)
                                } else {
                                    None
                                };
                                if let Some(delta) = delta {
                                    self.resize(app, i, delta);
                                    KeyEventResult::Handled
                                } else {
                                    KeyEventResult::Ignored
                                }
                            }))
                            .into_widget(),
                    );
                }
            }
            Flex::new(if horizontal {
                Axis::Horizontal
            } else {
                Axis::Vertical
            })
            .children(children)
            .into_widget()
        })
        .into_widget()
    }
}
