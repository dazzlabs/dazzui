// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class CodeStories
{
    public static StoryPage Page { get; } = new("Components", "Code", [
        new("Overview", theme => ComponentExamples.Code(theme, "Overview"), StoryLayout.Page),
        new("Inline", theme => ComponentExamples.Code(theme, "Inline"), StoryLayout.Centered),
        new("In prose", theme => ComponentExamples.Code(theme, "In prose"), StoryLayout.Centered),
    ]);
}
