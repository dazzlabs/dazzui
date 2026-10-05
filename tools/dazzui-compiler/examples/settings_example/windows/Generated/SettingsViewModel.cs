// AUTO-GENERATED FILE. DO NOT EDIT. Source: src/views/settings.rs

using System;
using System.ComponentModel;

namespace SettingsExample;

/// <summary>Holds the latest snapshot of the Rust state and sends actions to
/// Rust's <c>update</c>, which runs on the calling thread before <c>Send</c> returns.</summary>
public sealed class SettingsViewModel : INotifyPropertyChanged, IDisposable
{
    private readonly SettingsApp app = new();

    public SettingsState State { get; private set; }

    public event PropertyChangedEventHandler? PropertyChanged;

    public SettingsViewModel()
    {
        State = app.State();
        app.Subscribe(new Listener(new WeakReference<SettingsViewModel>(this)));
    }

    public void Send(SettingsAction action) => app.Send(action);

    public void Dispose() => app.Dispose();

    /// <summary>Rust calls this back on the thread that sent the action. It holds
    /// the model weakly: Rust holds it.</summary>
    private sealed class Listener(WeakReference<SettingsViewModel> model) : SettingsListener
    {
        public void StateChanged()
        {
            if (!model.TryGetTarget(out var m)) return;
            m.State = m.app.State();
            m.PropertyChanged?.Invoke(m, new PropertyChangedEventArgs(nameof(State)));
        }
    }
}
