// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class AsideStories
{
    public static StoryPage Page { get; } = new("Components", "Aside", [
        new("Overview", theme => ComponentExamples.Aside(theme, "Overview"), StoryLayout.Page),
        new("Beside a pane", theme => ComponentExamples.Aside(theme, "Beside a pane"), StoryLayout.Centered),
        new("An inspector", theme => ComponentExamples.Aside(theme, "An inspector"), StoryLayout.Centered),
        new("With an action", theme => ComponentExamples.Aside(theme, "With an action"), StoryLayout.Centered),
        new("In a three-column window", theme => ComponentExamples.Aside(theme, "In a three-column window"), StoryLayout.Centered),
    ]);
}
