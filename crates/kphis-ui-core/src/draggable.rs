// Drag And Drop
// from TabOrganizer https://github.com/Pauan/tab-organizer

use dominator::{Dom, DomBuilder, clone, events, html};
use futures_signals::{
    signal::{Mutable, Signal},
    signal_vec::MutableVec,
};
use std::{
    rc::Rc,
    sync::atomic::{AtomicU32, Ordering},
};
use wasm_bindgen::JsCast;
use web_sys::Element;

static ID_COUNTER: AtomicU32 = AtomicU32::new(0);

pub const DRAGGABLE_STYLE: &str = r#"
.draggable-container {
    display: grid;
    gap: 3px;
    margin-bottom: 0;
    padding-bottom: 1rem;
    border: 3px solid transparent;
    border-radius: .5em
}
.draggable-container.over {
    border: 3px dotted #777
}
.draggable-box {
    border: 3px solid transparent;
    border-radius: .25rem;
    opacity: 1
}
.draggable-box.over {
    border: 3px dotted #777
}
.draggable-box.dragging {
    opacity: 0.4
}
.draggable-box.selected {
    border-color: #17e
}"#;

#[derive(Debug)]
pub struct Dragable<T> {
    pub state: Mutable<Option<T>>,
    pub id: u32,
    pub selected: Mutable<bool>,
    pub dragging: Mutable<bool>,
    pub overing: Mutable<bool>,
}

impl<T: Clone + Default + 'static> Dragable<T> {
    pub fn new(state: Option<T>) -> Rc<Self> {
        Rc::new(Self {
            state: Mutable::new(state),
            id: ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            selected: Mutable::new(false),
            dragging: Mutable::new(false),
            overing: Mutable::new(false),
        })
    }

    pub fn new_clone(&self) -> Rc<Self> {
        Rc::new(Self {
            state: self.state.clone(),
            id: ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            selected: Mutable::new(false),
            dragging: Mutable::new(false),
            overing: Mutable::new(false),
        })
    }

    pub fn render(group: Rc<Group<T>>, draggable: Rc<Self>, drag_state: Mutable<Option<DragState<T>>>, inner: Dom, can_drag: bool) -> Dom {
        html!("li", {
            .class("draggable-box")
            .class("input-list-item")
            .apply_if(can_drag, |dom| { dom
                .class_signal("selected", draggable.selected.signal())
                .class_signal("dragging", draggable.dragging.signal())
                .class_signal("over", draggable.overing.signal())
                .event(clone!(group, draggable, drag_state => move |e: events::MouseDown| {
                    if let Some(elm) = e.target().and_then(|target| target.dyn_into::<Element>().ok()) {
                        if elm.tag_name().as_str() == "DRAG-HANDLE" {
                            let shift = e.shift_key();
                            let ctrl = e.ctrl_key();
                            let alt = e.alt_key();
                            if let events::MouseButton::Left = e.button() {
                                if ctrl && !shift && !alt {
                                    group.ctrl_select_draggable(&draggable);
                                } else if !ctrl && shift && !alt {
                                    group.shift_select_draggable(&draggable);
                                } else if !ctrl && !shift && !alt {
                                    group.click_draggable(&draggable);
                                    Self::drag_start(e.mouse_x(), e.mouse_y(), &group, &draggable, drag_state.clone());
                                }
                            }
                        }
                    }
                }))
                .event(clone!(group, draggable, drag_state => move |_: events::MouseEnter| {
                    Self::drag_over(&group, &draggable, drag_state.clone());
                }))
                .event(clone!(draggable => move |_: events::MouseLeave| {
                    draggable.overing.set(false);
                }))
            })
            .child(inner)
        })
    }

    fn get_draggable_index(draggables: &[Rc<Self>], draggable_id: u32) -> Option<usize> {
        draggables.iter().position(|x| x.id == draggable_id)
    }

    fn unwrap_draggable_index(draggables: &[Rc<Self>], draggable_id: u32) -> usize {
        Self::get_draggable_index(draggables, draggable_id).unwrap_or(draggables.len())
    }

    /// set DragState::DragStart
    fn drag_start(mouse_x: i32, mouse_y: i32, group: &Rc<Group<T>>, draggable: &Rc<Self>, drag_state: Mutable<Option<DragState<T>>>) {
        let mut dragging = drag_state.lock_mut();

        if dragging.is_none() {
            let draggable_index = Self::unwrap_draggable_index(&group.draggables.lock_ref(), draggable.id);
            *dragging = Some(DragState::DragStart {
                mouse_x,
                mouse_y,
                group: group.clone(),
                draggable: draggable.clone(),
                draggable_index,
            });
        }
    }

    /// drag over draggable of some group -> update DragState's group and draggable_index
    pub fn drag_over(new_group: &Rc<Group<T>>, new_draggable: &Rc<Self>, drag_state: Mutable<Option<DragState<T>>>) {
        let mut dragging = drag_state.lock_mut();

        if let Some(DragState::Dragging {
            ref mut group,
            ref mut draggable_index,
            ..
        }) = *dragging
        {
            new_draggable.overing.set(true);

            let draggables = new_group.draggables.lock_ref();
            let len = draggables.len();
            let new_index = Self::get_draggable_index(&draggables, new_draggable.id).unwrap_or(len);
            let new_draggable_index = if new_group.id == group.id {
                // over draggable in the same group
                let old_index = draggable_index.unwrap_or(len);
                if new_index >= old_index {
                    // move downward => try insert below dragovered one
                    let new_index = new_index + 1;
                    if new_index < len {
                        // not last item
                        Some(new_index)
                    } else {
                        // last item
                        None
                    }
                } else {
                    // move upward
                    Some(new_index)
                }
            } else {
                // to different group
                if new_index == (len - 1) {
                    // last item
                    None
                } else {
                    // not last item -> insert above dragovered one
                    Some(new_index)
                }
            };

            *group = new_group.clone();
            *draggable_index = new_draggable_index;
        }
    }
}

/// update DragState
pub fn drag_move<T>(new_x: i32, new_y: i32, drag_state: Mutable<Option<DragState<T>>>, selected_draggables: Mutable<Vec<Rc<SelectedDragable<T>>>>)
where
    T: Clone + Default + 'static,
{
    let mut dragging = drag_state.lock_mut();

    let new_dragging = match *dragging {
        Some(DragState::DragStart {
            mouse_x,
            mouse_y,
            ref group,
            ref draggable,
            draggable_index,
        }) => {
            let mouse_x = (mouse_x - new_x) as f64;
            let mouse_y = (mouse_y - new_y) as f64;

            // drag at least
            if mouse_x.hypot(mouse_y) > 7.0 {
                let old_selected_draggables: Vec<Rc<Dragable<T>>> = if draggable.selected.get() {
                    // drag all selected draggables
                    group.selected_draggables()
                } else {
                    // drag a single draggable
                    vec![draggable.clone()]
                };

                if !old_selected_draggables.is_empty() {
                    // update dragging + create SelectedDragable
                    let new_selected_draggables = old_selected_draggables
                        .into_iter()
                        .map(|draggable| {
                            draggable.dragging.set_neq(true);
                            SelectedDragable::new(draggable)
                        })
                        .collect();

                    selected_draggables.set(new_selected_draggables);

                    // new DragState with old group
                    Some(DragState::Dragging {
                        group: group.clone(),
                        draggable_index: Some(draggable_index),
                    })
                } else {
                    None
                }
            } else {
                None
            }
        }
        Some(DragState::Dragging { .. }) => None,
        None => None,
    };

    if new_dragging.is_some() {
        *dragging = new_dragging;
    }
}

pub fn drag_end<T>(
    drag_state: Mutable<Option<DragState<T>>>,
    selected_draggables: Mutable<Vec<Rc<SelectedDragable<T>>>>,
    all_single_group: &[Mutable<Rc<Group<T>>>],
    all_multiple_group: &[Mutable<Rc<Group<T>>>],
    changed_mutable: Mutable<bool>,
) where
    T: Clone + Default + 'static,
{
    let mut dragging = drag_state.lock_mut();
    let mut selected_draggables = selected_draggables.lock_mut();

    if let Some(DragState::Dragging { ref group, draggable_index, .. }) = *dragging {
        drag_draggables_to(&selected_draggables, group, draggable_index, all_single_group, all_multiple_group, changed_mutable);
    }

    // cleaning
    if dragging.is_some() {
        *dragging = None;
    }
    if !selected_draggables.is_empty() {
        for selected in selected_draggables.iter() {
            selected.draggable.dragging.set_neq(false);
        }
        *selected_draggables = vec![];
    }
}

fn drag_draggables_to<T>(
    selected_draggables: &[Rc<SelectedDragable<T>>], // Group.selected_draggables == source (draggable + index)s
    group: &Group<T>,                                // DragState::Dragging.group == destination group
    draggable_index: Option<usize>,                  // DragState::Dragging.draggable_index == destination index
    all_single_group: &[Mutable<Rc<Group<T>>>],
    all_multiple_group: &[Mutable<Rc<Group<T>>>],
    changed_mutable: Mutable<bool>,
) where
    T: Clone + Default + 'static,
{
    let ids = selected_draggables.iter().map(|selected| selected.draggable.id).collect::<Vec<u32>>();
    let mut selected_clones = selected_draggables.iter().map(|selected| selected.draggable.new_clone()).collect::<Vec<Rc<Dragable<T>>>>();

    // move selected to target group
    // 1. insert selected draggables into target group/position
    let mut olds = group.draggables.lock_ref().to_vec();
    let olds_len = olds.len();

    let mut need_removing = true;
    if group.single_item {
        // target is single_item => swap single draggable
        if ids.len() == 1 {
            // pop old draggable and push old draggable to destination
            if let Some(old_draggable) = group.draggables.lock_mut().pop() {
                for multiple_group in all_multiple_group {
                    let group_lock = multiple_group.lock_ref();
                    let mut lock = group_lock.draggables.lock_mut();
                    if lock.iter().any(|d| ids.contains(&d.id)) {
                        lock.push_cloned(old_draggable.clone());
                    }
                }
            }
            // push new draggable to new group
            if let Some(new_draggable) = selected_clones.pop() {
                group.draggables.lock_mut().push_cloned(new_draggable);
            }
        } else {
            // multiple selected to single_item group => do nothing
            need_removing = false;
        }
    } else if let Some(position) = draggable_index {
        if position == 0 {
            selected_clones.extend(olds);
            group.draggables.lock_mut().replace_cloned(selected_clones);
        } else if position < olds_len {
            olds.splice(position..position, selected_clones);
            group.draggables.lock_mut().replace_cloned(olds);
        } else {
            olds.extend(selected_clones);
            group.draggables.lock_mut().replace_cloned(olds);
        }
    } else {
        olds.extend(selected_clones);
        group.draggables.lock_mut().replace_cloned(olds);
    }

    // 2. remove old selected draggables
    if need_removing {
        for single_group in all_single_group {
            let group_lock = single_group.lock_ref();
            let mut lock = group_lock.draggables.lock_mut();
            if lock.iter().any(|d| ids.contains(&d.id)) {
                lock.replace_cloned(vec![Dragable::new(None)]);
            }
        }
        for multiple_group in all_multiple_group {
            multiple_group.lock_ref().remove_draggables(&ids);
        }
        changed_mutable.set_neq(true);
    }
    // 3. clear
    for single_group in all_single_group {
        single_group.lock_ref().unovering_all_draggables();
    }
    for multiple_group in all_multiple_group {
        multiple_group.lock_ref().unovering_all_draggables();
    }
}

#[derive(Debug, Default)]
pub struct Group<T> {
    pub id: u32,
    pub has_empty: Mutable<bool>,
    pub draggables: MutableVec<Rc<Dragable<T>>>,
    pub last_selected_draggable: Mutable<Option<u32>>, // Dragable.state.id.0
    pub single_item: bool,
    pub overing: Mutable<bool>,
}

impl<T: Clone + Default + 'static> Group<T> {
    pub fn new() -> Self {
        Self {
            id: ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            ..Default::default()
        }
    }

    pub fn has_empty_draggable(&self) -> impl Signal<Item = bool> + use<T> {
        self.has_empty.signal()
    }

    pub fn set_has_empty_by_state(&self) {
        self.has_empty.set(self.draggables.lock_ref().iter().any(|draggable| draggable.state.lock_ref().is_none()));
    }

    pub fn new_single() -> Self {
        Self {
            id: ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            has_empty: Mutable::new(true),
            draggables: MutableVec::new_with_values(vec![Dragable::new(None)]),
            single_item: true,
            ..Default::default()
        }
    }

    pub fn add_new_draggable(&self) {
        self.has_empty.set(true);
        self.draggables.lock_mut().push_cloned(Dragable::new(None));
    }

    /// drag over any group -> update DragState's group and draggable_index
    pub fn drag_over_group(new_group: &Rc<Self>, drag_state: Mutable<Option<DragState<T>>>) {
        let mut dragging = drag_state.lock_mut();

        if new_group.draggables.lock_ref().is_empty() {
            new_group.overing.set_neq(true);
        }

        if let Some(DragState::Dragging {
            ref mut group,
            ref mut draggable_index,
            ..
        }) = *dragging
        {
            if new_group.id != group.id {
                *group = new_group.clone();
                *draggable_index = Some(0);
            }
        }
    }

    pub fn group_mixins<E>(group: Rc<Self>, drag_state: Mutable<Option<DragState<T>>>) -> impl FnOnce(DomBuilder<E>) -> DomBuilder<E>
    where
        E: std::convert::AsRef<web_sys::Element> + std::convert::AsRef<web_sys::Node> + std::convert::AsRef<web_sys::EventTarget> + 'static,
    {
        #[inline]
        move |dom| {
            dom.class("draggable-container")
                .class_signal("over", group.overing.signal())
                .event(clone!(group, drag_state => move |_: events::MouseEnter| {
                    if drag_state.lock_ref().is_some() {
                        Group::drag_over_group(&group, drag_state.clone())
                    }
                }))
                .event(move |_: events::MouseLeave| {
                    if drag_state.lock_ref().is_some() {
                        group.overing.set_neq(false);
                    }
                })
        }
    }

    pub fn click_draggable(&self, draggable: &Dragable<T>) {
        if !draggable.selected.get() {
            self.unselect_all_draggables();
        }
    }

    pub fn ctrl_select_draggable(&self, draggable: &Rc<Dragable<T>>) {
        let mut selected = draggable.selected.lock_mut();

        *selected = !*selected;

        if *selected {
            self.last_selected_draggable.set_neq(Some(draggable.id));
        } else {
            self.last_selected_draggable.set_neq(None);
        }
    }

    pub fn shift_select_draggable(&self, draggable: &Rc<Dragable<T>>) {
        let mut last_selected_draggable = self.last_selected_draggable.lock_mut();

        let selected = match &*last_selected_draggable {
            Some(last_selected_draggable) => {
                let draggables = self.draggables.lock_ref();
                let mut seen = false;

                for x in draggables.iter() {
                    if x.id == *last_selected_draggable || x.id == draggable.id {
                        x.selected.set_neq(true);

                        if draggable.id != *last_selected_draggable {
                            seen = !seen;
                        }
                    } else if seen {
                        x.selected.set_neq(true);
                    } else {
                        x.selected.set_neq(false);
                    }
                }
                true
            }
            None => false,
        };

        if !selected {
            draggable.selected.set_neq(true);
            *last_selected_draggable = Some(draggable.id);
        }
    }

    pub fn unselect_all_draggables(&self) {
        {
            let draggables = self.draggables.lock_ref();
            for draggable in draggables.iter() {
                draggable.selected.set_neq(false);
            }
        }
        self.last_selected_draggable.set_neq(None);
    }

    pub fn unovering_all_draggables(&self) {
        self.overing.set_neq(false);
        let draggables = self.draggables.lock_ref();
        for draggable in draggables.iter() {
            draggable.overing.set_neq(false);
        }
    }

    pub fn selected_draggables(&self) -> Vec<Rc<Dragable<T>>> {
        let draggables = self.draggables.lock_ref();

        draggables.iter().filter(|draggable| draggable.selected.get()).cloned().collect()
    }

    pub fn remove_draggables(&self, draggable_ids: &[u32]) {
        self.draggables.lock_mut().retain(|draggable| !draggable_ids.contains(&draggable.id))
    }
}

#[derive(Debug)]
pub enum DragState<T> {
    DragStart {
        mouse_x: i32,
        mouse_y: i32,
        group: Rc<Group<T>>,
        draggable: Rc<Dragable<T>>,
        draggable_index: usize,
    },
    Dragging {
        group: Rc<Group<T>>,
        draggable_index: Option<usize>,
    },
}

#[derive(Debug)]
pub struct SelectedDragable<T> {
    pub draggable: Rc<Dragable<T>>,
    // pub index: usize, // position when selected
}

impl<T> SelectedDragable<T> {
    pub fn new(
        // index: usize,
        draggable: Rc<Dragable<T>>,
    ) -> Rc<Self> {
        Rc::new(Self {
            draggable,
            // index,
        })
    }
}
