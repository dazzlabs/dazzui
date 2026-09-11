// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class RailStories
{
    public static StoryPage Page { get; } = new("Components", "Rail", [
        new("Overview", theme => ComponentExamples.Rail(theme, "Overview"), StoryLayout.Page),
        new("A column", theme => ComponentExamples.Rail(theme, "A column"), StoryLayout.Centered),
        new("Groups", theme => ComponentExamples.Rail(theme, "Groups"), StoryLayout.Centered),
        new("With an action", theme => ComponentExamples.Rail(theme, "With an action"), StoryLayout.Centered),
        new("Beside a sidebar", theme => ComponentExamples.Rail(theme, "Beside a sidebar"), StoryLayout.Centered),
        new("Scrolling", theme => ComponentExamples.Rail(theme, "Scrolling"), StoryLayout.Centered),
    ]);
}
