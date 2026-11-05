//! Compile the shared recursive node data without replacing rich slot widgets.
use super::{scene, state::Context};
use dazzui_gtk4::{gtk, prelude::*, *};
use serde_json::Value;

fn strings(value: &Value) -> Vec<String> {
    value
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_owned)
        .collect()
}
fn nodes(
    raw: &Value,
    specimens: &mut Vec<(usize, gtk::Widget)>,
    ctx: &Context,
    bindings: &mut Vec<(String, Value)>,
) -> Result<Vec<TreeNode>, String> {
    raw.as_array()
        .ok_or("Tree items must be a committed array")?
        .iter()
        .map(|raw| {
            let value = ctx.resolve(raw, &[]);
            let id = value["id"]
                .as_str()
                .ok_or("Tree ids must be strings")?
                .to_owned();
            let label = scene::render_with(&raw["label"], specimens, ctx)?;
            let text = value["text"]
                .as_str()
                .or_else(|| value["label"].as_str())
                .unwrap_or("");
            let mut node = TreeNode::new(&id, &label, text);
            for name in ["icon", "meta"] {
                if let Some(slot) = raw.get(name)
                    && !ctx.resolve(slot, &[]).is_null()
                {
                    let child = scene::render_with(slot, specimens, ctx)?;
                    if name == "icon" {
                        node.icon = Some(child);
                    } else {
                        node.meta = Some(child);
                    }
                }
            }
            node.disabled = value["disabled"].as_bool().unwrap_or(false);
            node.children = raw
                .get("children")
                .map(|children| nodes(children, specimens, ctx, bindings))
                .transpose()?;
            bindings.push((id, raw.clone()));
            Ok(node)
        })
        .collect()
}
pub(super) fn render(
    raw: &Value,
    specimens: &mut Vec<(usize, gtk::Widget)>,
    ctx: &Context,
) -> Result<gtk::Widget, String> {
    let source = &raw["props"];
    let props = ctx.resolve(source, &[]);
    let mut bindings = vec![];
    let items = nodes(&source["items"], specimens, ctx, &mut bindings)?;
    let tint = match props["tint"].as_str().unwrap_or("primary") {
        "neutral" => Tint::Neutral,
        "info" => Tint::Info,
        "success" => Tint::Success,
        "warning" => Tint::Warning,
        "danger" => Tint::Danger,
        _ => Tint::Primary,
    };
    let options = TreeOptions {
        expanded: source.get("expanded").map(|_| strings(&props["expanded"])),
        default_expanded: strings(&props["defaultExpanded"]),
        selected: props["selected"].as_str().map(str::to_owned),
        size: if props["size"] == "small" {
            TreeSize::Small
        } else {
            TreeSize::Medium
        },
        tint,
    };
    let tree = tree_content(items, options)?;
    let context = ctx.clone();
    let action = source["onSelectedChange"].clone();
    tree.connect_selected_changed(move |_, id| context.run(&action, &[Value::String(id.into())]));
    let context = ctx.clone();
    let action = source["onExpandedChange"].clone();
    tree.connect_expanded_changed(move |_, ids| context.run(&action, &[serde_json::json!(ids)]));
    let source = source.clone();
    ctx.bind(&tree, move |tree, ctx| {
        let props = ctx.resolve(&source, &[]);
        tree.set_selected(props["selected"].as_str());
        if source.get("expanded").is_some() {
            tree.set_expanded(&strings(&props["expanded"]));
        }
        for (id, raw) in &bindings {
            let value = ctx.resolve(raw, &[]);
            tree.set_item_disabled(id, value["disabled"].as_bool().unwrap_or(false));
            tree.set_item_text(
                id,
                value["text"]
                    .as_str()
                    .or_else(|| value["label"].as_str())
                    .unwrap_or(""),
            );
        }
    });
    Ok(tree.upcast())
}
