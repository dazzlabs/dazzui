// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class SidebarStories
{
    public static StoryPage Page { get; } = new("Components", "Sidebar", [
        new("Overview", theme => ComponentExamples.Sidebar(theme, "Overview"), StoryLayout.Page),
        new("A column", theme => ComponentExamples.Sidebar(theme, "A column"), StoryLayout.Centered),
        new("Groups", theme => ComponentExamples.Sidebar(theme, "Groups"), StoryLayout.Centered),
        new("With a header", theme => ComponentExamples.Sidebar(theme, "With a header"), StoryLayout.Centered),
        new("With a card", theme => ComponentExamples.Sidebar(theme, "With a card"), StoryLayout.Centered),
        new("With an action", theme => ComponentExamples.Sidebar(theme, "With an action"), StoryLayout.Centered),
        new("Resizable", theme => ComponentExamples.Sidebar(theme, "Resizable"), StoryLayout.Centered),
        new("Collapsing", theme => ComponentExamples.Sidebar(theme, "Collapsing"), StoryLayout.Centered),
        new("A fixed width", theme => ComponentExamples.Sidebar(theme, "A fixed width"), StoryLayout.Centered),
    ]);
}
