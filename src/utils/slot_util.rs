use yew::virtual_dom::{VNode};
use yew::Children;

pub fn get_slots_default(children: Children) -> Option<VNode> {
    for i in children.into_iter() {
        match i {
            VNode::VTag(ref vtag) => {
                match vtag.attributes {
                    yew::virtual_dom::Attributes::Static(vev) => {
                        for _ in vev {
                            return Some(i);
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    None
}

pub fn get_slot_by_name(children: Children, slot_name: String) -> Option<VNode> {
    for i in children.clone().into_iter() {
        match i {
            VNode::VTag(ref vtag) => {
                match vtag.attributes {
                    yew::virtual_dom::Attributes::Static(vev) => {
                        for g in vev {
                            if g.0 == "slot" {
                                // log!(format!("{:?}", g.1));
                                if g.1 == slot_name {
                                    return Some(i);
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    None
}

pub fn get_comp_slot_by_name(children: Children, slot_name: String) -> Option<VNode> {
    for i in children.clone().into_iter() {
        match i {
            VNode::VTag(ref vtag) => {
                match vtag.attributes {
                    yew::virtual_dom::Attributes::Static(vev) => {
                        for g in vev {
                            if g.0 == "slot" {
                                // log!(format!("{:?}", g.1));
                                if g.1 == slot_name {
                                    if vtag.children().is_empty() {
                                        return None;
                                    }
                                    let first = vtag.children().first().clone();
                                    return Some(first.unwrap().clone());
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    None
}

pub fn get_comp_list_slot_by_name(children: Children, slot_name: String) -> Option<VNode> {
    for i in children.clone().into_iter() {
        match i {
            VNode::VTag(ref vtag) => {
                match vtag.attributes {
                    yew::virtual_dom::Attributes::Static(vev) => {
                        for g in vev {
                            if g.0 == "slot" {
                                // log!(format!("{:?}", g.1));
                                if g.1 == slot_name {
                                    if vtag.children().is_empty() {
                                        return None;
                                    }
                                    return Some(VNode::VList(vtag.children().clone()));
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    None
}


