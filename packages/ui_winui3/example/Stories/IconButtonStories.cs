// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class IconButtonStories
{
    public static StoryPage Page { get; } = new("Components", "IconButton", [
        new("Overview", theme => ComponentExamples.IconButton(theme, "Overview"), StoryLayout.Page),
        new("Plain", theme => ComponentExamples.IconButton(theme, "Plain"), StoryLayout.Centered),
        new("Filled", theme => ComponentExamples.IconButton(theme, "Filled"), StoryLayout.Centered),
        new("Variants", theme => ComponentExamples.IconButton(theme, "Variants"), StoryLayout.Centered),
        new("Disabled", theme => ComponentExamples.IconButton(theme, "Disabled"), StoryLayout.Centered),
        new("Sizes", theme => ComponentExamples.IconButton(theme, "Sizes"), StoryLayout.Centered),
        new("Tints", theme => ComponentExamples.IconButton(theme, "Tints"), StoryLayout.Centered),
        new("A toolbar", theme => ComponentExamples.IconButton(theme, "A toolbar"), StoryLayout.Centered),
        new("In a list header", theme => ComponentExamples.IconButton(theme, "In a list header"), StoryLayout.Centered),
    ]);
}
