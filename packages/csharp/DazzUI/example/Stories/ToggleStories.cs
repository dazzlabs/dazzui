// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class ToggleStories
{
    public static StoryPage Page { get; } = new("Components", "Toggle", [
        new("Overview", theme => ComponentExamples.Toggle(theme, "Overview"), StoryLayout.Page),
        new("Pressed", theme => ComponentExamples.Toggle(theme, "Pressed"), StoryLayout.Centered),
        new("Sizes", theme => ComponentExamples.Toggle(theme, "Sizes"), StoryLayout.Centered),
        new("Variants", theme => ComponentExamples.Toggle(theme, "Variants"), StoryLayout.Centered),
        new("Tints", theme => ComponentExamples.Toggle(theme, "Tints"), StoryLayout.Centered),
        new("Disabled", theme => ComponentExamples.Toggle(theme, "Disabled"), StoryLayout.Centered),
        new("A toolbar", theme => ComponentExamples.Toggle(theme, "A toolbar"), StoryLayout.Centered),
    ]);
}
