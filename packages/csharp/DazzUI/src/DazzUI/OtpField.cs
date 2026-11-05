using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Windows.System;

namespace DazzUI;

public sealed class OtpField : Surface
{
    private readonly StackPanel row = new() { Orientation = Orientation.Horizontal };
    private readonly List<Control> inputs = [];
    private string[] digits = [];
    private string pendingValue = "";
    private bool updating;
    private bool builtMasked;
    public int Length { get; set; } = 6;
    public int GroupSize { get; set; }
    public bool Masked { get; set; }
    public event EventHandler<string>? ValueChanged;
    public event EventHandler<string>? Completed;
    public string Value
    {
        get => inputs.Count == 0 ? pendingValue : string.Concat(digits);
        set
        {
            pendingValue = new string(value.Where(char.IsDigit).Take(Length).ToArray());
            updating = true;
            for (var i = 0; i < inputs.Count; i++)
            {
                digits[i] = i < pendingValue.Length ? pendingValue[i].ToString() : "";
                Write(inputs[i], digits[i]);
            }
            updating = false;
        }
    }
    public OtpField() { Content = row; }
    private static string Read(Control field) => field is PasswordBox password ? password.Password : ((TextBox)field).Text;
    private static void Write(Control field, string value)
    {
        if (field is PasswordBox password) password.Password = value;
        else ((TextBox)field).Text = value;
    }
    protected override void RefreshAppearance()
    {
        if (Length < 1) throw new ArgumentOutOfRangeException(nameof(Length));
        row.Spacing = Theme.Number("spacing.2");
        if (inputs.Count != Length || builtMasked != Masked)
        {
            var previous = Value;
            inputs.Clear(); row.Children.Clear(); digits = Enumerable.Repeat("", Length).ToArray(); builtMasked = Masked;
            for (var index = 0; index < Length; index++)
            {
                var position = index;
                Control field;
                if (Masked)
                {
                    var password = new PasswordBox { MaxLength = Length, PasswordRevealMode = PasswordRevealMode.Hidden };
                    password.PasswordChanged += (_, _) => Edited(position); field = password;
                }
                else
                {
                    var text = new TextField { TextAlignment = TextAlignment.Center, MaxLength = Length };
                    text.TextChanged += (_, _) => Edited(position); field = text;
                }
                Microsoft.UI.Xaml.Automation.AutomationProperties.SetName(field, $"Digit {index + 1} of {Length}");
                field.KeyDown += (_, args) =>
                {
                    if ((args.Key == VirtualKey.Back && Read(field).Length == 0 || args.Key == VirtualKey.Left) && position > 0)
                    { inputs[position - 1].Focus(FocusState.Keyboard); args.Handled = true; }
                    else if (args.Key == VirtualKey.Right && position + 1 < Length)
                    { inputs[position + 1].Focus(FocusState.Keyboard); args.Handled = true; }
                };
                Skin.Input(field, Theme, Size);
                inputs.Add(field); row.Children.Add(field);
            }
            Value = previous;
        }
        for (var i = 0; i < inputs.Count; i++)
        {
            var field = inputs[i];
            Appearance.SetSize(field, Size);
            Skin.Input(field, Theme, Size);
            field.Width = field.Height = Theme.Number($"control.{Theme.Key(Size)}.size");
            field.Margin = new Thickness(GroupSize > 0 && i > 0 && i % GroupSize == 0 ? Theme.Number("spacing.3") : 0, 0, 0, 0);
        }
    }
    private void Edited(int position)
    {
        if (updating || position >= inputs.Count) return;
        // TextBox queues TextChanged; ignore notifications from our own writes
        // even when they arrive after the synchronous update guard is cleared.
        if (Read(inputs[position]) == digits[position]) return;
        var value = new string(Read(inputs[position]).Where(char.IsDigit).ToArray());
        updating = true;
        digits[position] = value.Length == 0 ? "" : value[..1];
        Write(inputs[position], digits[position]);
        for (var offset = 1; offset < value.Length && position + offset < Length; offset++)
        {
            digits[position + offset] = value[offset].ToString();
            Write(inputs[position + offset], digits[position + offset]);
        }
        updating = false;
        if (value.Length > 0) inputs[Math.Min(position + value.Length, Length - 1)].Focus(FocusState.Programmatic);
        pendingValue = Value; ValueChanged?.Invoke(this, Value);
        if (digits.All(digit => digit.Length == 1)) Completed?.Invoke(this, Value);
    }
}
