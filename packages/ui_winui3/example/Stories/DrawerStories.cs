// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class DrawerStories
{
    public static StoryPage Page { get; } = new("Components", "Drawer", [
        new("Overview", theme => ComponentExamples.Drawer(theme, "Overview"), StoryLayout.Page),
        new("Bare", theme => ComponentExamples.Drawer(theme, "Bare"), StoryLayout.Centered),
        new("Sides", theme => ComponentExamples.Drawer(theme, "Sides"), StoryLayout.Centered),
        new("With footer", theme => ComponentExamples.Drawer(theme, "With footer"), StoryLayout.Centered),
        new("Controlled", theme => ComponentExamples.Drawer(theme, "Controlled"), StoryLayout.Centered),
        new("Wide", theme => ComponentExamples.Drawer(theme, "Wide"), StoryLayout.Centered),
    ]);
}
