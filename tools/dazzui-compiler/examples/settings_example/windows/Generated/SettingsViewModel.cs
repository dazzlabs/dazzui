// AUTO-GENERATED FILE. DO NOT EDIT. Source: src/views/settings.rs

using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;

namespace SettingsExample;

/// <summary><c>Plugin</c>, from the Rust app core.</summary>
public sealed record Plugin(uint Id, string Name, bool Enabled);

/// <summary><c>SettingsState</c>, as Rust last handed it over.</summary>
public sealed record SettingsState(bool LaunchAtStartup, bool AutoUpdate, bool Saved, uint Saves, string Status, bool ShowDisabledPlugins, IReadOnlyList<Plugin> Plugins, IReadOnlyList<Plugin> VisiblePlugins, bool HasHiddenPlugins);

public abstract record SettingsAction
{
    private SettingsAction() { }

    public sealed record SetLaunchAtStartup(bool Value) : SettingsAction;

    public sealed record SetAutoUpdate(bool Value) : SettingsAction;

    public sealed record Save() : SettingsAction;

    public sealed record Reset() : SettingsAction;

    public sealed record SetShowDisabledPlugins(bool Value) : SettingsAction;

    public sealed record TogglePlugin(uint Value) : SettingsAction;
}

/// <summary>Holds the latest snapshot of the Rust state and sends actions to
/// Rust's <c>update</c>, which runs on the calling thread before <c>Send</c> returns.</summary>
public sealed unsafe class SettingsViewModel : INotifyPropertyChanged, IDisposable
{
    private IntPtr app;
    private GCHandle self;

    public SettingsState State { get; private set; }

    public event PropertyChangedEventHandler? PropertyChanged;

    public SettingsViewModel()
    {
        app = SettingsNative.settings_create();
        self = GCHandle.Alloc(this, GCHandleType.Weak);
        SettingsNative.settings_subscribe(app, &OnStateChanged, GCHandle.ToIntPtr(self));
        State = SettingsNative.ReadState(app);
    }

    public void Send(SettingsAction action) => SettingsNative.Send(app, action);

    public void Dispose()
    {
        if (app == IntPtr.Zero) return;
        SettingsNative.settings_destroy(app);
        app = IntPtr.Zero;
        self.Free();
    }

    /// <summary>Rust calls this back on the thread that sent the action.</summary>
    [UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) })]
    private static void OnStateChanged(IntPtr context)
    {
        if (GCHandle.FromIntPtr(context).Target is not SettingsViewModel model) return;
        model.State = SettingsNative.ReadState(model.app);
        model.PropertyChanged?.Invoke(model, new PropertyChangedEventArgs(nameof(State)));
    }
}
