// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class MenubarStories
{
    public static StoryPage Page { get; } = new("Components", "Menubar", [
        new("Overview", theme => ComponentExamples.Menubar(theme, "Overview"), StoryLayout.Page),
        new("Bare", theme => ComponentExamples.Menubar(theme, "Bare"), StoryLayout.Centered),
        new("With checks", theme => ComponentExamples.Menubar(theme, "With checks"), StoryLayout.Centered),
        new("Disabled", theme => ComponentExamples.Menubar(theme, "Disabled"), StoryLayout.Centered),
    ]);
}
