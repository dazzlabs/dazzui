// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class TagStories
{
    public static StoryPage Page { get; } = new("Components", "Tag", [
        new("Overview", theme => ComponentExamples.Tag(theme, "Overview"), StoryLayout.Page),
        new("Selectable", theme => ComponentExamples.Tag(theme, "Selectable"), StoryLayout.Centered),
        new("Removable", theme => ComponentExamples.Tag(theme, "Removable"), StoryLayout.Centered),
        new("Sizes", theme => ComponentExamples.Tag(theme, "Sizes"), StoryLayout.Centered),
        new("Variants", theme => ComponentExamples.Tag(theme, "Variants"), StoryLayout.Centered),
        new("Tints", theme => ComponentExamples.Tag(theme, "Tints"), StoryLayout.Centered),
        new("Disabled", theme => ComponentExamples.Tag(theme, "Disabled"), StoryLayout.Centered),
        new("A filter bar", theme => ComponentExamples.Tag(theme, "A filter bar"), StoryLayout.Centered),
    ]);
}
