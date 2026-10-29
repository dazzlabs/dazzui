using System.Text.Json.Nodes;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Automation.Peers;
using Microsoft.UI.Xaml.Automation.Provider;
using Microsoft.UI.Xaml.Media;
using Microsoft.UI.Xaml.Media.Imaging;
using UI = DazzUI;

namespace DazzUI.Example;

/// <summary>Exercise the public components in a real window, including their native automation contracts.</summary>
internal static class BehaviorTests
{
    private static void Check(bool condition, string message)
    {
        if (!condition) throw new InvalidOperationException(message);
    }
    private static IEnumerable<DependencyObject> Walk(DependencyObject root)
    {
        yield return root;
        for (var i = 0; i < VisualTreeHelper.GetChildrenCount(root); i++)
            foreach (var child in Walk(VisualTreeHelper.GetChild(root, i))) yield return child;
    }
    private static T Pattern<T>(FrameworkElement element, PatternInterface pattern) where T : class
    {
        var peer = FrameworkElementAutomationPeer.CreatePeerForElement(element);
        var result = peer?.GetPattern(pattern);
        return result as T ?? throw new InvalidOperationException($"Missing {pattern} on {element.GetType().Name}: peer={peer?.GetType().FullName}, result={result?.GetType().FullName}, loaded={element.IsLoaded}, enabled={(element as Control)?.IsEnabled}.");
    }
    private static async Task Settle(Shell shell)
    {
        await Task.Delay(80);
        shell.UpdateLayout();
        try { await new RenderTargetBitmap().RenderAsync(shell).AsTask().WaitAsync(TimeSpan.FromSeconds(30)); }
        catch (TimeoutException error)
        {
            throw new TimeoutException($"Native snapshot timed out: host visible={shell.XamlRoot?.IsHostVisible}, fixture={shell.CurrentPage?.GetType().Name}.", error);
        }
    }
    public static async Task RunAsync(Shell shell, string output)
    {
        File.WriteAllText(Path.Combine(output, "progress.txt"), "Interactive controls, inherited themes and popup behavior");
        // Let initial navigation selection finish before installing fixtures.
        await Settle(shell);
        await TableTests.RunAsync(shell, output);
        Check(new UI.Button().Variant == UI.ControlVariant.Recessed, "Button's public default must match React's recessed variant.");
        var quietIcon = new UI.IconButton();
        Check(quietIcon.Variant == UI.ControlVariant.Plain && quietIcon.Tint == UI.WidgetTint.Neutral && quietIcon.Size == UI.ControlSize.Tiny, "IconButton's public defaults must match React's quiet toolbar control.");
        var preference = new UI.PreferenceRow { Label = "Open details", Description = "Native keyboard and automation activation", Width = 320 };
        var preferenceOpened = 0;
        EventHandler onOpen = (_, _) => preferenceOpened++;
        shell.ShowFixture(preference); await Settle(shell);
        Check(!Walk(preference).OfType<Microsoft.UI.Xaml.Controls.Button>().Any(), "An inert preference row became an action.");
        preference.Opened += onOpen; await Settle(shell);
        var preferenceAction = Walk(preference).OfType<Microsoft.UI.Xaml.Controls.Button>().Single();
        Check(preferenceAction.IsTabStop, "An interactive preference row is missing from keyboard navigation.");
        Pattern<IInvokeProvider>(preferenceAction, PatternInterface.Invoke).Invoke(); await Settle(shell);
        Check(preferenceOpened == 1, "Native preference invocation did not report its action once.");
        preference.Opened -= onOpen; await Settle(shell);
        Check(!Walk(preference).OfType<Microsoft.UI.Xaml.Controls.Button>().Any(), "Removing the action left a focusable preference button.");
        var annotation = new UI.Tag { Text = "Before" };
        shell.ShowFixture(annotation); await Settle(shell);
        annotation.Text = "After"; await Settle(shell);
        Check(annotation.Text == "After" && Walk(annotation).OfType<TextBlock>().Any(label => label.Text == "After" && Math.Abs(label.LineHeight - annotation.Selection.FontSize * 1.4) < .001),
            $"Updating tag text lost its label or declared leading: text={annotation.Text}, size={annotation.Selection.FontSize}, rendered={string.Join(", ", Walk(annotation).OfType<TextBlock>().Select(label => $"{label.Text}:{label.LineHeight}"))}.");
        var meter = new UI.Meter { Minimum = 20, Maximum = 60, Value = 50, ShowValue = true, Label = "Usage", Width = 320 };
        shell.ShowFixture(meter); await Settle(shell);
        var meterRange = Pattern<IRangeValueProvider>(meter, PatternInterface.RangeValue);
        Check(meterRange.Minimum == 20 && meterRange.Maximum == 60 && meterRange.Value == 50, "Meter automation lost its custom range.");
        Check(Walk(meter).OfType<TextBlock>().Any(label => label.Text == "75%" && label.FontFamily.Source == meter.Theme.CodeFont.Source), "Meter did not format its range in the shared code face.");
        var boundedNumber = new UI.NumberField { Minimum = 10, Maximum = 12, Value = 12, Width = 160 };
        shell.ShowFixture(boundedNumber); await Settle(shell);
        var steps = new[] { "Increase", "Decrease" }.Select(name => Walk(boundedNumber).OfType<Microsoft.UI.Xaml.Controls.Button>().Single(button => Microsoft.UI.Xaml.Automation.AutomationProperties.GetName(button) == name)).ToArray();
        Check(!steps[0].IsEnabled && steps[1].IsEnabled, "Number field did not disable the increment at its maximum.");
        boundedNumber.Value = 10; await Settle(shell);
        Check(steps[0].IsEnabled && !steps[1].IsEnabled, "Number field did not update its boundary actions after the value changed.");
        Pattern<IInvokeProvider>(steps[0], PatternInterface.Invoke).Invoke(); await Settle(shell);
        Check(boundedNumber.Value == 11, "Native stepper activation did not change the numeric value.");
        var choiceState = new SceneState(new JsonObject { ["person"] = "grace" });
        var choice = (UI.Select)Nodes.Build(JsonNode.Parse("""
            {"$el":"Select","props":{"value":{"$state":"person"},"onValueChange":{"$do":[{"set":"person","to":{"$arg":0}}]},"options":[{"value":"none","label":"Unassigned"},{"label":"People","options":[{"value":"ada","label":"Ada"},{"value":"grace","label":"Grace","disabled":true}]}]}}
            """), new Context(choiceState))!;
        shell.ShowFixture(choice); await Settle(shell);
        Check(choice.Value == "grace" && choice.Items.Count == 4 && choice.Items.OfType<ComboBoxItem>().Count(item => !item.IsEnabled) == 2, "Select lost its mixed groups, disabled option or initial controlled value.");
        choice.Value = "ada"; await Settle(shell);
        Check(choiceState.Read("person")!.GetValue<string>() == "ada", "Selecting an option did not update the controlled scene.");
        choiceState.Write("person", JsonValue.Create("none")); await Settle(shell);
        Check(choice.Value == "none", "An external controlled value did not reach the existing native selector.");
        choice.Mono = true; choice.Invalid = true; await Settle(shell);
        Check(choice.FontFamily.Source == choice.Theme.CodeFont.Source && ((SolidColorBrush)choice.BorderBrush).Color == choice.Theme.Color("color.danger.600"), "Select did not apply live typography and error changes.");
        var tree = new UI.Tree { Width = 240 };
        tree.Nodes.Add(new UI.TreeNode("empty", "Empty folder", Children: [],
            IconView: () => new Border { Width = 16, Height = 16 },
            LabelView: () => new UI.Badge { Content = "Rich label" },
            MetaView: () => new UI.Badge { Content = "M" }));
        tree.Nodes.Add(new UI.TreeNode("leaf", "Leaf"));
        shell.ShowFixture(tree); await Settle(shell);
        Check(Walk(tree).OfType<UI.Badge>().Count() == 2, "Tree lost a rich label or metadata slot.");
        Check(Walk(tree).OfType<UI.Icon>().Count() == 1, "An empty branch must retain its twisty.");
        tree.SelectedId = "leaf"; await Settle(shell);
        Check(Walk(tree).OfType<UI.Badge>().Count() == 2, "Tree slots were lost after selection rebuilt the rows.");
        tree.Nodes.Add(new UI.TreeNode("parent", "Parent", Children: [new UI.TreeNode("child", "Child")], Expanded: false));
        tree.ExpandedIds = ["parent"]; await Settle(shell);
        Check(Walk(tree).OfType<TextBlock>().Any(label => label.Text == "Child"), "Controlled tree expansion did not open the branch.");
        tree.ExpandedIds = []; await Settle(shell);
        Check(!Walk(tree).OfType<TextBlock>().Any(label => label.Text == "Child"), "Controlled tree expansion did not close the branch.");
        foreach (var themeName in new[] { "studio-light", "bright-light", "bright-dark", "graphite-light", "studio-dark" })
        {
            tree.Theme = new UI.Theme(themeName);
            foreach (var size in new[] { UI.ControlSize.Small, UI.ControlSize.Medium })
            {
                tree.Size = size;
                foreach (var width in new[] { 240d, 16d, 240d })
                {
                    tree.Width = width; await Settle(shell);
                    foreach (var row in ((StackPanel)tree.Content).Children.OfType<Grid>())
                    {
                        var expected = new CornerRadius(Math.Min(tree.Theme.Number("radius.small"), Math.Min(row.ActualWidth, row.ActualHeight) / 2));
                        Check(row.CornerRadius == expected,
                            $"{themeName}: {size} tree row at width {width} must have circular corners ({expected}), got {row.CornerRadius}.");
                    }
                    Check(tree.SelectedId == "leaf" && !Walk(tree).OfType<TextBlock>().Any(label => label.Text == "Child"),
                        "Tree theme and size changes lost selection or controlled expansion.");
                }
                if (themeName.StartsWith("bright-")) await SmokeTest.SaveAsync(tree, output, $"{themeName}-tree-{size}.png");
            }
        }
        var buttonSamples = new StackPanel { Spacing = 16, HorizontalAlignment = HorizontalAlignment.Center, VerticalAlignment = VerticalAlignment.Center };
        foreach (var size in Enum.GetValues<UI.ControlSize>())
        {
            var row = new StackPanel { Orientation = Orientation.Horizontal, Spacing = 16 };
            foreach (var label in new[] { "Save changes", "BUTTON", "Ag jyp", "保存更改" })
                row.Children.Add(new UI.Button { Content = label, Size = size });
            buttonSamples.Children.Add(row);
        }
        shell.ShowFixture(buttonSamples); await Settle(shell);
        foreach (var button in Walk(buttonSamples).OfType<UI.Button>())
        {
            var labelPresenter = Walk(button).OfType<ContentPresenter>().First();
            Check(labelPresenter.TextLineBounds == TextLineBounds.Tight && labelPresenter.VerticalContentAlignment == VerticalAlignment.Center,
                "Single-line button labels must use optically centered font bounds.");
            Check(button.ActualHeight == button.Theme.Number($"control.{button.Size.ToString().ToLowerInvariant()}.size"), "Label alignment changed button height.");
        }
        await SmokeTest.SaveAsync(shell, output, "button-labels.png");
        var changingButton = Walk(buttonSamples).OfType<UI.Button>().First();
        changingButton.Content = "First\nSecond";
        await Settle(shell);
        Check(Walk(changingButton).OfType<ContentPresenter>().First().TextLineBounds == TextLineBounds.Full, "Multiline button labels lost full line spacing.");
        foreach (var size in Enum.GetValues<UI.ControlSize>())
        {
            foreach (var input in new UI.TextField[] { new(), new UI.SearchField(), new UI.FieldValue() })
            {
                input.Size = size; input.Width = 240; input.PlaceholderText = "Placeholder";
                shell.ShowFixture(input); await Settle(shell);
                if (input is not UI.SearchField)
                    Check(input.Padding.Left == input.Theme.Number("spacing.3") && input.Padding.Right == input.Theme.Number("spacing.3"), $"{input.GetType().Name} {size}: field inherited a button inset.");
                Check(input.IsLoaded && ReferenceEquals(shell.CurrentPage, input), "Input fixture was replaced before layout checks.");
                foreach (var value in new[] { "", "Ag 文本" })
                {
                    input.Text = value; await Settle(shell);
                    if (value.Length == 0)
                    {
                        var placeholder = Walk(input).OfType<TextBlock>().First(element => element.Name == "PlaceholderTextContentPresenter");
                        var line = new TextBlock { Text = placeholder.Text, FontFamily = placeholder.FontFamily, FontSize = placeholder.FontSize, FontWeight = placeholder.FontWeight };
                        line.Measure(new Windows.Foundation.Size(double.PositiveInfinity, double.PositiveInfinity));
                        var top = placeholder.TransformToVisual(input).TransformPoint(new Windows.Foundation.Point()).Y + placeholder.Padding.Top;
                        var bottom = input.ActualHeight - top - line.DesiredSize.Height;
                        Check(Math.Abs(top - bottom) <= 1, $"{input.GetType().Name} {size}: unequal placeholder insets {top}/{bottom}.");
                    }
                    else
                    {
                        var editor = Walk(input).OfType<ScrollViewer>().First(element => element.Name == "ContentElement");
                        Check(editor.Padding.Top == input.Padding.Top && editor.Padding.Bottom == input.Padding.Bottom,
                            "The native editor did not receive the same vertical padding as the placeholder.");
                    }
                    await SmokeTest.SaveAsync(shell, output, $"input-{input.GetType().Name}-{size}-{value.Length}.png");
                }
                input.AcceptsReturn = true; await Settle(shell);
                Check(input.Padding.Top == input.Theme.Number("spacing.2.5"), "Multiline input lost its vertical padding.");
            }
        }
        var checkbox = new UI.Checkbox { Content = "Keep formatting" };
        foreach (var size in Enum.GetValues<UI.ControlSize>())
        {
            foreach (var input in new UI.Surface[] { new UI.Autocomplete { PlaceholderText = "Placeholder" }, new UI.Combobox { PlaceholderText = "Placeholder" }, new UI.NumberField { PlaceholderText = "Placeholder" }, new UI.OtpField(), new UI.OtpField { Masked = true } })
            {
                input.Size = size; input.Width = 300;
                shell.ShowFixture(input); await Settle(shell);
                var editors = Walk(input).OfType<Control>().Where(control => control is TextBox or PasswordBox).ToArray();
                Check(editors.Length > 0, $"{input.GetType().Name} has no native editor.");
                foreach (var editor in editors)
                {
                    Check(editor.Padding.Top > 0 && editor.Padding.Top == editor.Padding.Bottom,
                        $"{input.GetType().Name} {size}: editor padding {editor.Padding} is not symmetric.");
                    Check(editor.ActualHeight <= input.Theme.Number($"control.{size.ToString().ToLowerInvariant()}.size") + 1,
                        $"{input.GetType().Name} {size}: native editor height {editor.ActualHeight} exceeds the control size.");
                }
                await SmokeTest.SaveAsync(shell, output, $"input-{input.GetType().Name}-{size}-{(input is UI.OtpField otpInput && otpInput.Masked ? "masked" : "normal")}.png");
                if (input is UI.Autocomplete autocomplete) autocomplete.Text = "Ag 文本";
                if (input is UI.NumberField number) number.Value = 42;
                if (input is UI.OtpField code) code.Value = "123456";
                input.Theme = new UI.Theme("studio-dark");
                await Settle(shell);
                Check(Walk(input).OfType<Control>().Where(control => control is TextBox or PasswordBox).All(editor => editor.Padding.Top > 0 && editor.Padding.Top == editor.Padding.Bottom),
                    $"{input.GetType().Name}: changing theme reset the editor padding.");
                await SmokeTest.SaveAsync(shell, output, $"input-{input.GetType().Name}-{size}-{(input is UI.OtpField maskedInput && maskedInput.Masked ? "masked" : "normal")}-filled.png");
            }
        }
        var radioA = new UI.Radio { Content = "Automatic", Tag = "auto", IsChecked = true };
        var radioB = new UI.Radio { Content = "Manual", Tag = "manual" };
        var group = new UI.RadioGroup(); group.Items.Add(radioA); group.Items.Add(radioB);
        var text = new UI.TextField { Width = 240 };
        var field = new UI.FormField { Label = "Name", Required = true, Field = text };
        var form = new UI.Form { Validate = () => string.IsNullOrWhiteSpace(text.Text) ? new Dictionary<string, string> { ["Name"] = "Enter a name" } : new Dictionary<string, string>() };
        form.Fields.Add(field);
        var otp = new UI.OtpField { Value = "12" };
        var range = new UI.RangeSlider { Width = 240 };
        var action = new UI.Button { Content = "Open" };
        var scope = new UI.ThemeScope { Theme = new("studio-light"), Content = new StackPanel { Spacing = 12, Children = { checkbox, group, form, otp, range, action } } };
        shell.ShowFixture(scope); await Settle(shell);

        Pattern<IToggleProvider>(checkbox, PatternInterface.Toggle).Toggle();
        Check(checkbox.IsChecked == true, "Checkbox automation did not toggle the checked state.");
        Pattern<ISelectionItemProvider>(radioB, PatternInterface.SelectionItem).Select();
        Check(radioB.IsChecked == true && radioA.IsChecked == false, "Radio group did not retain exclusive selection.");
        Check(!form.Submit() && field.Error is not null, "Form did not reject a missing required field.");
        await Settle(shell);
        // WinUI implements TextBox's Value/Text providers in native code, not
        // as projected Microsoft.UI.Xaml.Automation.Provider interfaces.
        // https://learn.microsoft.com/windows/windows-app-sdk/api/winrt/microsoft.ui.xaml.automation.peers.textboxautomationpeer
        Check(FrameworkElementAutomationPeer.CreatePeerForElement(text).GetAutomationControlType() == AutomationControlType.Edit, "TextField lost its native edit automation role.");
        text.Text = "Handbook";
        var submitted = false; form.Submitted += (_, _) => submitted = true;
        Check(form.Submit() && submitted && field.Error is null, "Form failed to submit valid input or clear its error.");

        var completions = 0; otp.Completed += (_, _) => completions++;
        var firstDigit = Walk(otp).OfType<UI.TextField>().First();
        firstDigit.Text = "987654";
        await Settle(shell);
        Check(otp.Value == "987654" && completions == 1, $"OTP paste did not distribute digits exactly once: value={otp.Value}, events={completions}.");
        Pattern<IRangeValueProvider>(range.Lower, PatternInterface.RangeValue).SetValue(25);
        Pattern<IRangeValueProvider>(range.Upper, PatternInterface.RangeValue).SetValue(75);
        Pattern<IRangeValueProvider>(range.Lower, PatternInterface.RangeValue).SetValue(90);
        Check(range.Lower.Value == 75 && range.Upper.Value == 75, "Range handles crossed each other.");
        range.Lower.Value = 25;
        Check(Math.Abs(Canvas.GetTop(range.Lower) - Canvas.GetTop(range.Upper)) < .1, "Range handles do not share one track.");

        VisualStateManager.GoToState(action, "PointerOver", false);
        scope.Theme = new("bright-dark"); await Settle(shell);
        Check(checkbox.IsChecked == true && text.Text == "Handbook" && otp.Value == "987654", "Changing theme reset editing or selection state.");
        Check(action.Theme.Name == "bright-dark", "A nested control did not inherit the new theme.");
        var presenter = Walk(action).OfType<ContentPresenter>().First();
        Check(((SolidColorBrush)presenter.Background).Color == action.Theme.Recipe(action.Variant, action.Tint, "surface", "hovered").Color, "A live theme change left the active button state stale.");
        await SmokeTest.SaveAsync(shell, output, "interactive.png");

        var menuInvoked = false;
        var menu = new UI.Menu(); menu.Items.Add(new("Run", () => menuInvoked = true));
        menu.Show(action); await Settle(shell);
        Check(menu.Flyout.IsOpen, "Menu did not open.");
        Pattern<IInvokeProvider>((FrameworkElement)menu.Flyout.Items[0], PatternInterface.Invoke).Invoke();
        await Settle(shell); Check(menuInvoked, "Menu action did not invoke."); menu.Flyout.Hide();
        var popover = new UI.Popover { Content = new UI.TextField { PlaceholderText = "Rename" } };
        popover.Show(action); await Settle(shell);
        Check(popover.IsOpen && UI.Appearance.GetTheme(popover.Content!).Name == "bright-dark", "Popover did not open with its anchor's theme.");
        popover.Hide(); await Settle(shell);

        var dialog = new UI.Dialog { Title = "Confirm export", Content = new TextBlock { Text = "Export the document?" } };
        var shown = dialog.ShowAsync(action); await Settle(shell);
        var primary = Walk(dialog.Native).OfType<Microsoft.UI.Xaml.Controls.Button>().FirstOrDefault(button => button.Name == "PrimaryButton");
        Check(primary is not null, "Dialog primary action was not rendered.");
        await SmokeTest.SaveAsync(dialog.Native, output, "dialog.png");
        Pattern<IInvokeProvider>(primary!, PatternInterface.Invoke).Invoke();
        Check(await shown == ContentDialogResult.Primary, "Dialog did not return its primary action result.");
        await Settle(shell);
        foreach (var side in Enum.GetValues<UI.DrawerSide>())
        {
            var drawer = new UI.Drawer { Side = side, Title = "Settings", Content = new UI.Checkbox { Content = "Remember this choice" } };
            var opened = drawer.ShowAsync(action); await Settle(shell);
            var panel = Walk(drawer.Native).OfType<FrameworkElement>().First(element => element.Name == "BackgroundElement");
            var size = action.XamlRoot.Size;
            Check(side is UI.DrawerSide.Left or UI.DrawerSide.Right ? Math.Abs(panel.ActualHeight - size.Height) < 1 : Math.Abs(panel.ActualWidth - size.Width) < 1, $"{side} drawer did not span its window edge.");
            await SmokeTest.SaveAsync(drawer.Native, output, $"drawer-{side}.png");
            drawer.Hide(); await opened; await Settle(shell);
        }
        // A calendar across two months: eighty-four cells, one tab stop, and a
        // day that stands in both months painted in both.
        var today = new DateOnly(2026, 10, 16);
        var calendar = new UI.Calendar { Mode = UI.CalendarMode.Range, Months = 2, Size = UI.ControlSize.Small, Today = today, Range = new UI.DateRange(new DateOnly(2026, 10, 28), new DateOnly(2026, 11, 3)) };
        shell.ShowFixture(calendar); await Settle(shell);
        var days = Walk(calendar).OfType<Microsoft.UI.Xaml.Controls.Button>().Where(button => button is not UI.Button).ToArray();
        Check(days.Length == 84, $"Two months must draw eighty-four days, not {days.Length}.");
        Check(days.Count(day => day.IsTabStop) == 1, "The grid must be one tab stop.");
        var filled = calendar.Theme.Recipe(UI.ControlVariant.Filled, calendar.Tint, "surface").Color;
        Check(days.Count(day => day.Background is SolidColorBrush brush && brush.Color == filled) == 4, "Each end of the range must be filled in every month it stands in.");
        await SmokeTest.SaveAsync(calendar, output, "calendar-two-months.png");
        var ranged = 0; calendar.RangeChanged += (_, _) => ranged++;
        var ninth = days.First(day => Microsoft.UI.Xaml.Automation.AutomationProperties.GetName(day) == "Friday, October 9, 2026");
        Pattern<IInvokeProvider>(ninth, PatternInterface.Invoke).Invoke(); await Settle(shell);
        Check(ranged == 1 && calendar.Range.Start == new DateOnly(2026, 10, 9) && calendar.Range.End is null, "A press on a finished range did not start a new one.");
        // The picker: its sheet opens from the trigger, a press moves the
        // draft and not the value, and Done writes the draft back.
        var picker = new UI.DateRangePicker { Today = today, Value = new UI.DateRangeValue(new DateTime(2026, 10, 16, 14, 30, 0), new DateTime(2026, 10, 16, 15, 30, 0)), Width = 320 };
        shell.ShowFixture(picker); await Settle(shell);
        picker.Sheet.ShowAt(picker.Trigger); await Settle(shell);
        Check(picker.Sheet.IsOpen && picker.Sheet.Content is FrameworkElement, "The picker's sheet did not open.");
        var sheet = (FrameworkElement)picker.Sheet.Content!;
        var sheetCalendar = Walk(sheet).OfType<UI.Calendar>().Single();
        Check(Walk(sheet).OfType<UI.Select>().Count() == 2 && Walk(sheet).OfType<UI.Toggle>().Count() == 4, "The sheet lost its time lists or its duration chips.");
        await SmokeTest.SaveAsync(sheet, output, "date-range-picker-sheet.png");
        var committed = 0; picker.ValueChanged += (_, _) => committed++;
        var twentieth = Walk(sheetCalendar).OfType<Microsoft.UI.Xaml.Controls.Button>().First(day => Microsoft.UI.Xaml.Automation.AutomationProperties.GetName(day) == "Tuesday, October 20, 2026");
        Pattern<IInvokeProvider>(twentieth, PatternInterface.Invoke).Invoke(); await Settle(shell);
        Check(committed == 0 && picker.Value!.Value.Start.Day == 16, "The draft reached the value before Done.");
        var done = Walk(sheet).OfType<UI.Button>().Single(button => Equals(button.Content, "Done"));
        Pattern<IInvokeProvider>(done, PatternInterface.Invoke).Invoke(); await Settle(shell);
        Check(committed == 1 && picker.Value == new UI.DateRangeValue(new DateTime(2026, 10, 20, 14, 30, 0), new DateTime(2026, 10, 20, 15, 30, 0)) && !picker.Sheet.IsOpen,
            $"Done did not commit the draft and close the sheet: {picker.Value}, open={picker.Sheet.IsOpen}.");
        var masked = new UI.OtpField { Masked = true, Value = "654321" };
        shell.ShowFixture(masked); await Settle(shell);
        Check(Walk(masked).OfType<PasswordBox>().Count() == 6 && masked.Value == "654321", "Masked OTP did not use native password inputs.");
        await File.WriteAllTextAsync(Path.Combine(output, "behavior.txt"), "PASS: checkbox, exclusive radio selection, form validation/submission, native edit role, OTP paste/masking, range automation, theme/state preservation, menu action, popover, dialog primary result, four drawer edges, calendar across two months, date range picker draft and Done.\n");
    }
}
