use leptos::{html::ElementDescriptor, *};
use std::{ops::Deref, time::Duration};
use thaw_utils::{add_event_listener, use_next_frame, EventListenerHandle};

/// # CSS Transition
///
/// Reference to https://vuejs.org/guide/built-ins/transition.html
#[component]
pub fn CSSTransition<T, CF, IV>(
    node_ref: NodeRef<T>,
    #[prop(into)] show: MaybeSignal<bool>,
    #[prop(into)] name: MaybeSignal<String>,
    #[prop(optional)] appear: bool,
    #[prop(optional, into)] on_before_enter: Option<Callback<()>>,
    #[prop(optional, into)] on_enter: Option<Callback<()>>,
    #[prop(optional, into)] on_after_enter: Option<Callback<()>>,
    #[prop(optional, into)] on_before_leave: Option<Callback<()>>,
    #[prop(optional, into)] on_leave: Option<Callback<()>>,
    #[prop(optional, into)] on_after_leave: Option<Callback<()>>,
    children: CF,
) -> impl IntoView
where
    T: ElementDescriptor + Clone + 'static,
    CF: FnOnce(ReadSignal<Option<&'static str>>) -> IV + 'static,
    IV: IntoView,
{
    let display = create_rw_signal((!show.get_untracked()).then_some("display: none;"));

    node_ref.on_load(move |node_el| {
        let any_el = node_el.clone().into_any();
        let el = any_el.deref().clone();
        let class_list = el.class_list();
        let next_frame = use_next_frame();
        let end_handle = StoredValue::new(None::<EventListenerHandle>);
        let end_count = StoredValue::new(None::<usize>);
        let finish = StoredValue::new(None::<Callback<()>>);

        let on_end = Callback::new(move |remove: Callback<()>| {
            let Some(CSSTransitionInfo {
                types,
                prop_count,
                timeout,
            }) = get_transition_info(&el)
            else {
                remove.call(());
                return;
            };

            finish.set_value(Some(Callback::new(move |_| {
                end_count.set_value(None);
                remove.call(());
                end_handle.update_value(|h| {
                    h.take().map(|h| {
                        h.remove();
                    });
                });
            })));

            set_timeout(
                move || {
                    finish.try_update_value(|v| {
                        v.take().map(|f| f.call(()));
                    });
                },
                Duration::from_millis(timeout + 1),
            );

            end_count.set_value(Some(0));
            let event_listener = move || {
                end_count.update_value(|v| {
                    let Some(v) = v else {
                        return;
                    };
                    *v += 1;
                });
                if end_count.with_value(|v| {
                    let Some(v) = v else {
                        return false;
                    };
                    *v >= prop_count
                }) {
                    finish.update_value(|v| {
                        v.take().map(|f| f.call(()));
                    });
                }
            };
            let handle = match types {
                AnimationTypes::Transition => {
                    add_event_listener(any_el.clone(), ev::transitionend, move |_| event_listener())
                }
                AnimationTypes::Animation => {
                    add_event_listener(any_el.clone(), ev::animationend, move |_| event_listener())
                }
            };
            end_handle.set_value(Some(handle));
        });

        let on_finish = move || {
            finish.update_value(|v| {
                v.take().map(|f| f.call(()));
            });
        };

        let on_enter_fn = {
            let class_list = class_list.clone();
            Callback::new(move |name: String| {
                if let Some(on_before_enter) = on_before_enter {
                    on_before_enter.call(());
                }
                let enter_from = format!("{name}-enter-from");
                let enter_active = format!("{name}-enter-active");
                let enter_to = format!("{name}-enter-to");

                let _ = class_list.add_2(&enter_from, &enter_active);
                display.set(None);

                let class_list = class_list.clone();
                next_frame.run(move || {
                    let _ = class_list.remove_1(&enter_from);
                    let _ = class_list.add_1(&enter_to);

                    let remove = Callback::new(move |_| {
                        let _ = class_list.remove_2(&enter_active, &enter_to);
                        if let Some(on_after_enter) = on_after_enter {
                            on_after_enter.call(());
                        }
                    });
                    on_end.call(remove);

                    if let Some(on_enter) = on_enter {
                        on_enter.call(());
                    }
                });
            })
        };

        let on_leave_fn = {
            let class_list = class_list.clone();
            Callback::new(move |name: String| {
                if let Some(on_before_leave) = on_before_leave {
                    on_before_leave.call(());
                }
                let leave_from = format!("{name}-leave-from");
                let leave_active = format!("{name}-leave-active");
                let leave_to = format!("{name}-leave-to");

                let _ = class_list.add_2(&leave_from, &leave_active);

                let class_list = class_list.clone();
                next_frame.run(move || {
                    let _ = class_list.remove_1(&leave_from);
                    let _ = class_list.add_1(&leave_to);

                    let remove = Callback::new(move |_| {
                        let _ = class_list.remove_2(&leave_active, &leave_to);
                        display.set(Some("display: none;"));
                        if let Some(on_after_leave) = on_after_leave {
                            on_after_leave.call(());
                        }
                    });
                    on_end.call(remove);
                    if let Some(on_leave) = on_leave {
                        on_leave.call(());
                    }
                });
            })
        };

        create_render_effect(move |prev: Option<bool>| {
            let show = show.get();
            let prev = if let Some(prev) = prev {
                prev
            } else if show && appear {
                false
            } else {
                return show;
            };

            let name = name.get_untracked();

            if show && !prev {
                on_finish();
                on_enter_fn.call(name);
            } else if !show && prev {
                on_finish();
                on_leave_fn.call(name);
            }

            show
        });

        on_cleanup(move || {
            end_handle.update_value(|handle| {
                if let Some(handle) = handle.take() {
                    handle.remove();
                }
            });
        })
    });

    children(display.read_only())
}

#[derive(PartialEq)]
enum AnimationTypes {
    Transition,
    Animation,
}

struct CSSTransitionInfo {
    types: AnimationTypes,
    prop_count: usize,
    timeout: u64,
}

fn get_transition_info(el: &web_sys::HtmlElement) -> Option<CSSTransitionInfo> {
    let styles = window().get_computed_style(el).ok().flatten()?;

    let get_style_properties = |property: &str| {
        styles
            .get_property_value(property)
            .unwrap_or_default()
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
    };

    let transition_delays = get_style_properties("transition-delay");
    let transition_durations = get_style_properties("transition-duration");
    let transition_timeout = get_timeout(transition_delays, &transition_durations);
    let animation_delays = get_style_properties("animation-delay");
    let animation_durations = get_style_properties("animation-duration");
    let animation_timeout = get_timeout(animation_delays, &animation_durations);

    let timeout = u64::max(transition_timeout, animation_timeout);
    let (types, prop_count) = if timeout > 0 {
        if transition_timeout > animation_timeout {
            (AnimationTypes::Transition, transition_durations.len())
        } else {
            (AnimationTypes::Animation, animation_durations.len())
        }
    } else {
        return None;
    };

    Some(CSSTransitionInfo {
        types,
        prop_count,
        timeout,
    })
}

fn get_timeout(mut delays: Vec<String>, durations: &Vec<String>) -> u64 {
    while delays.len() < durations.len() {
        delays.append(&mut delays.clone())
    }

    fn to_ms(s: &String) -> u64 {
        if s == "auto" || s.is_empty() {
            return 0;
        }

        let s = s.split_at(s.len() - 1).0;

        (s.parse::<f32>().unwrap_or_default() * 1000.0).floor() as u64
    }

    durations
        .iter()
        .enumerate()
        .map(|(i, d)| to_ms(d) + to_ms(&delays[i]))
        .max()
        .unwrap_or_default()
}
