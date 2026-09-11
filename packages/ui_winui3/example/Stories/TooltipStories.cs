// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class TooltipStories
{
    public static StoryPage Page { get; } = new("Components", "Tooltip", [
        new("Overview", theme => ComponentExamples.Tooltip(theme, "Overview"), StoryLayout.Page),
        new("Default", theme => ComponentExamples.Tooltip(theme, "Default"), StoryLayout.Centered),
        new("Open", theme => ComponentExamples.Tooltip(theme, "Open"), StoryLayout.Centered),
        new("Sides", theme => ComponentExamples.Tooltip(theme, "Sides"), StoryLayout.Centered),
        new("On icon buttons", theme => ComponentExamples.Tooltip(theme, "On icon buttons"), StoryLayout.Centered),
        new("A toolbar", theme => ComponentExamples.Tooltip(theme, "A toolbar"), StoryLayout.Centered),
        new("A long label", theme => ComponentExamples.Tooltip(theme, "A long label"), StoryLayout.Centered),
    ]);
}
