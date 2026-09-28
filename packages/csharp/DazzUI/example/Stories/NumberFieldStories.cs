// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class NumberFieldStories
{
    public static StoryPage Page { get; } = new("Components", "NumberField", [
        new("Overview", theme => ComponentExamples.NumberField(theme, "Overview"), StoryLayout.Page),
        new("Bare", theme => ComponentExamples.NumberField(theme, "Bare"), StoryLayout.Centered),
        new("With unit", theme => ComponentExamples.NumberField(theme, "With unit"), StoryLayout.Centered),
        new("Sizes", theme => ComponentExamples.NumberField(theme, "Sizes"), StoryLayout.Centered),
        new("States", theme => ComponentExamples.NumberField(theme, "States"), StoryLayout.Centered),
        new("In a form", theme => ComponentExamples.NumberField(theme, "In a form"), StoryLayout.Centered),
        new("In settings", theme => ComponentExamples.NumberField(theme, "In settings"), StoryLayout.Centered),
    ]);
}
