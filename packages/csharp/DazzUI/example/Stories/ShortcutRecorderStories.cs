// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class ShortcutRecorderStories
{
    public static StoryPage Page { get; } = new("Components", "ShortcutRecorder", [
        new("Overview", theme => ComponentExamples.ShortcutRecorder(theme, "Overview"), StoryLayout.Page),
        new("Bound", theme => ComponentExamples.ShortcutRecorder(theme, "Bound"), StoryLayout.Centered),
        new("Unbound", theme => ComponentExamples.ShortcutRecorder(theme, "Unbound"), StoryLayout.Centered),
        new("Error state", theme => ComponentExamples.ShortcutRecorder(theme, "Error state"), StoryLayout.Centered),
        new("Disabled", theme => ComponentExamples.ShortcutRecorder(theme, "Disabled"), StoryLayout.Centered),
        new("States", theme => ComponentExamples.ShortcutRecorder(theme, "States"), StoryLayout.Centered),
        new("Modified", theme => ComponentExamples.ShortcutRecorder(theme, "Modified"), StoryLayout.Centered),
        new("Word keys", theme => ComponentExamples.ShortcutRecorder(theme, "Word keys"), StoryLayout.Centered),
        new("Against a key cap", theme => ComponentExamples.ShortcutRecorder(theme, "Against a key cap"), StoryLayout.Centered),
        new("Tints", theme => ComponentExamples.ShortcutRecorder(theme, "Tints"), StoryLayout.Centered),
        new("Conflicts", theme => ComponentExamples.ShortcutRecorder(theme, "Conflicts"), StoryLayout.Centered),
        new("In settings", theme => ComponentExamples.ShortcutRecorder(theme, "In settings"), StoryLayout.Centered),
    ]);
}
