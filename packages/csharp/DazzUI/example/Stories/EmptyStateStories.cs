// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class EmptyStateStories
{
    public static StoryPage Page { get; } = new("Components", "EmptyState", [
        new("Overview", theme => ComponentExamples.EmptyState(theme, "Overview"), StoryLayout.Page),
        new("On its own", theme => ComponentExamples.EmptyState(theme, "On its own"), StoryLayout.Centered),
        new("With an action", theme => ComponentExamples.EmptyState(theme, "With an action"), StoryLayout.Centered),
        new("With two actions", theme => ComponentExamples.EmptyState(theme, "With two actions"), StoryLayout.Centered),
        new("Nothing yet", theme => ComponentExamples.EmptyState(theme, "Nothing yet"), StoryLayout.Centered),
        new("Nothing matched", theme => ComponentExamples.EmptyState(theme, "Nothing matched"), StoryLayout.Centered),
        new("In a well", theme => ComponentExamples.EmptyState(theme, "In a well"), StoryLayout.Centered),
        new("In a pane", theme => ComponentExamples.EmptyState(theme, "In a pane"), StoryLayout.Centered),
    ]);
}
