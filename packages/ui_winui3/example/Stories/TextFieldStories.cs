// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class TextFieldStories
{
    public static StoryPage Page { get; } = new("Components", "TextField", [
        new("Overview", theme => ComponentExamples.TextField(theme, "Overview"), StoryLayout.Page),
        new("Default", theme => ComponentExamples.TextField(theme, "Default"), StoryLayout.Centered),
        new("Placeholder", theme => ComponentExamples.TextField(theme, "Placeholder"), StoryLayout.Centered),
        new("Disabled", theme => ComponentExamples.TextField(theme, "Disabled"), StoryLayout.Centered),
        new("Error state", theme => ComponentExamples.TextField(theme, "Error state"), StoryLayout.Centered),
        new("States", theme => ComponentExamples.TextField(theme, "States"), StoryLayout.Centered),
        new("Sizes", theme => ComponentExamples.TextField(theme, "Sizes"), StoryLayout.Centered),
        new("Tints", theme => ComponentExamples.TextField(theme, "Tints"), StoryLayout.Centered),
        new("Mono", theme => ComponentExamples.TextField(theme, "Mono"), StoryLayout.Centered),
        new("Beside a button", theme => ComponentExamples.TextField(theme, "Beside a button"), StoryLayout.Centered),
        new("A read-only value", theme => ComponentExamples.TextField(theme, "A read-only value"), StoryLayout.Centered),
        new("Multiline", theme => ComponentExamples.TextField(theme, "Multiline"), StoryLayout.Centered),
        new("Multiline mono", theme => ComponentExamples.TextField(theme, "Multiline mono"), StoryLayout.Centered),
        new("Multiline in a form", theme => ComponentExamples.TextField(theme, "Multiline in a form"), StoryLayout.Centered),
        new("Read-only", theme => ComponentExamples.TextField(theme, "Read-only"), StoryLayout.Centered),
        new("In a form field", theme => ComponentExamples.TextField(theme, "In a form field"), StoryLayout.Centered),
    ]);
}
