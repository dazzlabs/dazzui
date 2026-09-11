// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class MenuStories
{
    public static StoryPage Page { get; } = new("Components", "Menu", [
        new("Overview", theme => ComponentExamples.Menu(theme, "Overview"), StoryLayout.Page),
        new("Default", theme => ComponentExamples.Menu(theme, "Default"), StoryLayout.Centered),
        new("With shortcuts", theme => ComponentExamples.Menu(theme, "With shortcuts"), StoryLayout.Centered),
        new("With icons", theme => ComponentExamples.Menu(theme, "With icons"), StoryLayout.Centered),
        new("Checked", theme => ComponentExamples.Menu(theme, "Checked"), StoryLayout.Centered),
        new("Disabled", theme => ComponentExamples.Menu(theme, "Disabled"), StoryLayout.Centered),
        new("Alignment", theme => ComponentExamples.Menu(theme, "Alignment"), StoryLayout.Centered),
        new("From an icon button", theme => ComponentExamples.Menu(theme, "From an icon button"), StoryLayout.Centered),
        new("In a list header", theme => ComponentExamples.Menu(theme, "In a list header"), StoryLayout.Centered),
    ]);
}
