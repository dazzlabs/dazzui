// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class CollapsibleStories
{
    public static StoryPage Page { get; } = new("Components", "Collapsible", [
        new("Overview", theme => ComponentExamples.Collapsible(theme, "Overview"), StoryLayout.Page),
        new("Shut", theme => ComponentExamples.Collapsible(theme, "Shut"), StoryLayout.Centered),
        new("Open", theme => ComponentExamples.Collapsible(theme, "Open"), StoryLayout.Centered),
        new("With meta", theme => ComponentExamples.Collapsible(theme, "With meta"), StoryLayout.Centered),
        new("Disabled", theme => ComponentExamples.Collapsible(theme, "Disabled"), StoryLayout.Centered),
        new("Interactive", theme => ComponentExamples.Collapsible(theme, "Interactive"), StoryLayout.Centered),
    ]);
}
