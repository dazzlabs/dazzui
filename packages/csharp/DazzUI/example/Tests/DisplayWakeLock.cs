using System.Runtime.InteropServices;

namespace DazzUI.Example;

/// <summary>Keep native rendering available during a test without changing power settings.</summary>
internal sealed class DisplayWakeLock : IDisposable
{
    private readonly uint previous = SetThreadExecutionState(0x80000003);
    public void Dispose() => SetThreadExecutionState(previous == 0 ? 0x80000000 : previous);

    [DllImport("kernel32.dll")]
    private static extern uint SetThreadExecutionState(uint flags);
}
