// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class ComboboxStories
{
    public static StoryPage Page { get; } = new("Components", "Combobox", [
        new("Overview", theme => ComponentExamples.Combobox(theme, "Overview"), StoryLayout.Page),
        new("Bare", theme => ComponentExamples.Combobox(theme, "Bare"), StoryLayout.Centered),
        new("Grouped", theme => ComponentExamples.Combobox(theme, "Grouped"), StoryLayout.Centered),
        new("Sizes", theme => ComponentExamples.Combobox(theme, "Sizes"), StoryLayout.Centered),
        new("States", theme => ComponentExamples.Combobox(theme, "States"), StoryLayout.Centered),
        new("In a form", theme => ComponentExamples.Combobox(theme, "In a form"), StoryLayout.Centered),
    ]);
}
