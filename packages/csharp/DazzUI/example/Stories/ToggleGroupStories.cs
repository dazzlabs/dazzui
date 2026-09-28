// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class ToggleGroupStories
{
    public static StoryPage Page { get; } = new("Components", "ToggleGroup", [
        new("Overview", theme => ComponentExamples.ToggleGroup(theme, "Overview"), StoryLayout.Page),
        new("Glyphs", theme => ComponentExamples.ToggleGroup(theme, "Glyphs"), StoryLayout.Centered),
        new("Words", theme => ComponentExamples.ToggleGroup(theme, "Words"), StoryLayout.Centered),
        new("Single", theme => ComponentExamples.ToggleGroup(theme, "Single"), StoryLayout.Centered),
        new("Variants", theme => ComponentExamples.ToggleGroup(theme, "Variants"), StoryLayout.Centered),
        new("Tints", theme => ComponentExamples.ToggleGroup(theme, "Tints"), StoryLayout.Centered),
        new("Disabled", theme => ComponentExamples.ToggleGroup(theme, "Disabled"), StoryLayout.Centered),
    ]);
}
