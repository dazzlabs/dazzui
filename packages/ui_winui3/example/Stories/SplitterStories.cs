// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class SplitterStories
{
    public static StoryPage Page { get; } = new("Components", "Splitter", [
        new("Overview", theme => ComponentExamples.Splitter(theme, "Overview"), StoryLayout.Page),
        new("Horizontal", theme => ComponentExamples.Splitter(theme, "Horizontal"), StoryLayout.Centered),
        new("Vertical", theme => ComponentExamples.Splitter(theme, "Vertical"), StoryLayout.Centered),
        new("Three panels", theme => ComponentExamples.Splitter(theme, "Three panels"), StoryLayout.Centered),
        new("Constrained", theme => ComponentExamples.Splitter(theme, "Constrained"), StoryLayout.Centered),
        new("Nested", theme => ComponentExamples.Splitter(theme, "Nested"), StoryLayout.Centered),
    ]);
}
