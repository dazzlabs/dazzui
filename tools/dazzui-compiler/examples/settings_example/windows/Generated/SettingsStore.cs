// AUTO-GENERATED FILE. DO NOT EDIT. Source: src/model.rs

using System;
using System.ComponentModel;
using Microsoft.UI.Dispatching;

namespace SettingsExample;

/// <summary>The latest snapshot of the model <c>settings</c> makes, and a way to
/// send it actions: what a page takes. Whatever thread the model changes on,
/// the snapshot is read again on the thread that made the store.</summary>
public sealed class SettingsStore : INotifyPropertyChanged, IDisposable
{
    private readonly SettingsSource source;
    private readonly DispatcherQueue? dispatcher = DispatcherQueue.GetForCurrentThread();

    public SettingsState State { get; private set; }

    public event PropertyChangedEventHandler? PropertyChanged;

    public SettingsStore() : this(new SettingsSource()) { }

    public SettingsStore(SettingsSource source)
    {
        this.source = source;
        State = source.State();
        source.Subscribe(new Listener(new WeakReference<SettingsStore>(this)));
    }

    public void Send(SettingsAction action) => source.Send(action);

    public void Dispose() => source.Dispose();

    private void Refresh()
    {
        State = source.State();
        PropertyChanged?.Invoke(this, new PropertyChangedEventArgs(nameof(State)));
    }

    /// <summary>Rust calls this back on the thread that changed the state. On the
    /// store's own, it reads it at once, so a model that changes as it takes an
    /// action is shown changed when <c>Send</c> returns; from any other, it reads it
    /// there next. It holds the store weakly: Rust holds it.</summary>
    private sealed class Listener(WeakReference<SettingsStore> store) : SettingsListener
    {
        public void StateChanged()
        {
            if (!store.TryGetTarget(out var s)) return;
            if (s.dispatcher is null || s.dispatcher.HasThreadAccess) s.Refresh();
            else s.dispatcher.TryEnqueue(s.Refresh);
        }
    }
}
