// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class NavItemStories
{
    public static StoryPage Page { get; } = new("Components", "NavItem", [
        new("Overview", theme => ComponentExamples.NavItem(theme, "Overview"), StoryLayout.Page),
        new("Idle", theme => ComponentExamples.NavItem(theme, "Idle"), StoryLayout.Centered),
        new("Active", theme => ComponentExamples.NavItem(theme, "Active"), StoryLayout.Centered),
        new("Disabled", theme => ComponentExamples.NavItem(theme, "Disabled"), StoryLayout.Centered),
        new("States", theme => ComponentExamples.NavItem(theme, "States"), StoryLayout.Centered),
        new("Sizes", theme => ComponentExamples.NavItem(theme, "Sizes"), StoryLayout.Centered),
        new("With an icon", theme => ComponentExamples.NavItem(theme, "With an icon"), StoryLayout.Centered),
        new("Tints", theme => ComponentExamples.NavItem(theme, "Tints"), StoryLayout.Centered),
        new("Action", theme => ComponentExamples.NavItem(theme, "Action"), StoryLayout.Centered),
        new("Selecting", theme => ComponentExamples.NavItem(theme, "Selecting"), StoryLayout.Centered),
        new("In a sidebar", theme => ComponentExamples.NavItem(theme, "In a sidebar"), StoryLayout.Centered),
        new("With a count", theme => ComponentExamples.NavItem(theme, "With a count"), StoryLayout.Centered),
    ]);
}
