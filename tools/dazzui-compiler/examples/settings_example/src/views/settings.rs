use dazzui_runtime::ui;

use crate::SettingsModel;

ui! {
    page SettingsPage(model: SettingsModel) {
        Col(gap: "6", children: [
            Preferences(children: [
                PreferenceSection(label: "General", children: [
                    PreferenceRow(
                        title: "Launch at startup",
                        child: Switch(
                            checked: model.launch_at_startup,
                            on_checked_change: model.set_launch_at_startup,
                        ),
                    ),
                    PreferenceRow(
                        title: "Update automatically",
                        subtitle: "Download new versions in the background",
                        child: Switch(checked: model.auto_update, on_checked_change: model.set_auto_update),
                    ),
                ]),
                PreferenceSection(
                    label: "Plugins",
                    footer: "Plugins run in a sandbox and can be turned off at any time.",
                    children: [
                        PreferenceRow(
                            title: "Show disabled plugins",
                            child: Switch(
                                checked: model.show_disabled_plugins,
                                on_checked_change: model.set_show_disabled_plugins,
                            ),
                        ),
                    ],
                ),
            ]),
            Col(gap: "2", children: [
                for plugin in model.visible_plugins {
                    Row(gap: "2", children: [
                        Text(plugin.name),
                        if plugin.enabled {
                            Button(on_click: model.toggle_plugin(plugin.id), child: Text("Disable"))
                        } else {
                            Button(
                                variant: ButtonVariant::Filled,
                                on_click: model.toggle_plugin(plugin.id),
                                child: Text("Enable"),
                            )
                        },
                    ])
                },
                if model.has_hidden_plugins {
                    Text("Disabled plugins are hidden", tone: TextTone::Secondary)
                },
            ]),
            Text(model.status, tone: TextTone::Secondary),
            Row(gap: "2", children: [
                Button(
                    variant: ButtonVariant::Filled,
                    disabled: model.save_disabled,
                    on_click: model.save,
                    child: Text("Save"),
                ),
                Button(on_click: model.reset, child: Text("Reset to defaults")),
            ]),
        ])
    }
}
