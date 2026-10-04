// AUTO-GENERATED FILE. DO NOT EDIT. Source: src/views/settings.rs

using System;

namespace Settings;

public abstract record SettingsAction
{
    private SettingsAction() { }

    public sealed record SetLaunchAtStartup(bool Value) : SettingsAction;

    public sealed record SetAutoUpdate(bool Value) : SettingsAction;

    public sealed record Save() : SettingsAction;

    public sealed record Reset() : SettingsAction;
}

/// <summary>Holds what the page reads and forwards what it sends. Until the
/// bridge to Rust is generated, the host supplies where actions go.</summary>
public sealed class SettingsViewModel
{
    private readonly Action<SettingsAction> sink;

    public SettingsViewModel(Action<SettingsAction> send) => sink = send;

    public void Send(SettingsAction action) => sink(action);
}
