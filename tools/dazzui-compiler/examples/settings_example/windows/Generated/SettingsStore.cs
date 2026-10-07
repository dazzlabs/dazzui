// AUTO-GENERATED FILE. DO NOT EDIT. Source: src/model.rs

using System;
using System.ComponentModel;
using Microsoft.UI.Dispatching;

namespace SettingsExample;

/// <summary>The latest value of a <c>SettingsModel</c>, observable, and its methods:
/// what a page takes, as a <c>ListenableBuilder</c> hands a widget. Whatever
/// thread the model changes on, the value is read again on the thread that
/// made the store.</summary>
public sealed class SettingsStore : INotifyPropertyChanged, IDisposable
{
    private readonly DispatcherQueue? dispatcher = DispatcherQueue.GetForCurrentThread();
    private readonly ulong listener;

    public SettingsModel Model { get; }

    /// <summary>The model's methods, as a page calls them.</summary>
    public ISettingsActions Actions { get; }

    public SettingsState State { get; private set; }

    public event PropertyChangedEventHandler? PropertyChanged;

    public SettingsStore() : this(new SettingsModel()) { }

    public SettingsStore(SettingsModel model)
    {
        Model = model;
        Actions = new Forward(model);
        State = model.Value();
        listener = model.AddListener(new Listener(new WeakReference<SettingsStore>(this)));
    }

    public void Dispose()
    {
        Model.RemoveListener(listener);
        Model.Dispose();
    }

    private void Refresh()
    {
        State = Model.Value();
        PropertyChanged?.Invoke(this, new PropertyChangedEventArgs(nameof(State)));
    }

    private sealed class Forward(SettingsModel model) : ISettingsActions
    {
        public void SetLaunchAtStartup(bool on) => model.SetLaunchAtStartup(on);
        public void SetAutoUpdate(bool on) => model.SetAutoUpdate(on);
        public void SetShowDisabledPlugins(bool on) => model.SetShowDisabledPlugins(on);
        public void TogglePlugin(uint id) => model.TogglePlugin(id);
        public void Save() => model.Save();
        public void Reset() => model.Reset();
    }

    /// <summary>Rust calls this back on the thread that changed the value. On the
    /// store's own, it reads it at once, so a model that changes as a method runs
    /// is shown changed when the call returns; from any other, it reads it there
    /// next. It holds the store weakly: Rust holds it.</summary>
    private sealed class Listener(WeakReference<SettingsStore> store) : SettingsListener
    {
        public void Changed()
        {
            if (!store.TryGetTarget(out var s)) return;
            if (s.dispatcher is null || s.dispatcher.HasThreadAccess) s.Refresh();
            else s.dispatcher.TryEnqueue(s.Refresh);
        }
    }
}
