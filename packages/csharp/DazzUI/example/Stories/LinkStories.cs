// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class LinkStories
{
    public static StoryPage Page { get; } = new("Components", "Link", [
        new("Overview", theme => ComponentExamples.Link(theme, "Overview"), StoryLayout.Page),
        new("Underlines", theme => ComponentExamples.Link(theme, "Underlines"), StoryLayout.Centered),
        new("External", theme => ComponentExamples.Link(theme, "External"), StoryLayout.Centered),
        new("Tints", theme => ComponentExamples.Link(theme, "Tints"), StoryLayout.Centered),
        new("Disabled", theme => ComponentExamples.Link(theme, "Disabled"), StoryLayout.Centered),
        new("In prose", theme => ComponentExamples.Link(theme, "In prose"), StoryLayout.Centered),
    ]);
}
