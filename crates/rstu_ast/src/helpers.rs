// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

fn relink_parent_pointers(node: &mut Node) {
    let self_ptr = Some(NonNull::from(&mut *node));
    for child in &mut node.children {
        child.parent = self_ptr;
        relink_parent_pointers(child);
    }
}

fn relink_descendant_parents(&mut self) {
    let self_ptr = Some(NonNull::from(&mut *self));
    for child in &mut self.children {
        child.parent = self_ptr;
        child.relink_descendant_parents();
    }
}