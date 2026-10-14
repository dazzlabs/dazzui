//! Read the rounded geometry GTK actually submitted to its renderer.
//! A child control's corner is never borrowed for a larger parent box.
use dazzui_gtk4::{gtk, prelude::*};

fn same_size(rect: &gtk::graphene::Rect, width: f32, height: f32) -> bool {
    (rect.width() - width).abs() < 0.5 && (rect.height() - height).abs() < 0.5
}

fn rounded(node: &gtk::gsk::RenderNode, width: f32, height: f32) -> Option<[f32; 4]> {
    use gtk::gsk;
    if let Some(border) = node.downcast_ref::<gsk::BorderNode>() {
        let rect = border.outline();
        if same_size(rect.bounds(), width, height) {
            return Some(rect.corner().map(|corner| corner.width()));
        }
    }
    if let Some(clip) = node.downcast_ref::<gsk::RoundedClipNode>() {
        let rect = clip.clip();
        if same_size(rect.bounds(), width, height) {
            return Some(rect.corner().map(|corner| corner.width()));
        }
        return rounded(&clip.child(), width, height);
    }
    if let Some(container) = node.downcast_ref::<gsk::ContainerNode>() {
        for i in 0..container.n_children() {
            if let Some(corners) = rounded(&container.child(i), width, height) {
                return Some(corners);
            }
        }
    }
    if let Some(transform) = node.downcast_ref::<gsk::TransformNode>() {
        return rounded(&transform.child(), width, height);
    }
    if let Some(clip) = node.downcast_ref::<gsk::ClipNode>() {
        return rounded(&clip.child(), width, height);
    }
    if let Some(opacity) = node.downcast_ref::<gsk::OpacityNode>() {
        return rounded(&opacity.child(), width, height);
    }
    if let Some(debug) = node.downcast_ref::<gsk::DebugNode>() {
        return rounded(&debug.child(), width, height);
    }
    None
}

/// Top-left, top-right, bottom-right, bottom-left, in logical pixels.
/// No rounded paint for this box is unread; a square border reads as zero.
#[allow(deprecated)] // GTK4's public CSS snapshot API isolates a container's own paint.
pub fn corners(widget: &impl IsA<gtk::Widget>) -> Option<[f32; 4]> {
    let widget = widget.as_ref();
    let bounds = widget.compute_bounds(widget)?;
    let snapshot = gtk::Snapshot::new();
    if widget.is::<gtk::Box>() {
        // A clear layout can be exactly as large as its only child (a meter
        // without a label, for example). Its complete snapshot then contains
        // that child's rounded track. Draw only this box's computed CSS
        // background and frame so the child's corner never becomes its own.
        let style = widget.style_context();
        snapshot.render_background(
            &style,
            0.,
            0.,
            bounds.width() as f64,
            bounds.height() as f64,
        );
        snapshot.render_frame(
            &style,
            0.,
            0.,
            bounds.width() as f64,
            bounds.height() as f64,
        );
        return rounded(&snapshot.to_node()?, bounds.width(), bounds.height());
    }
    gtk::WidgetPaintable::new(Some(widget)).snapshot(
        &snapshot,
        bounds.width() as f64,
        bounds.height() as f64,
    );
    rounded(&snapshot.to_node()?, bounds.width(), bounds.height())
}
