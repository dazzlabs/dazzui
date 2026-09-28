// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class MeterStories
{
    public static StoryPage Page { get; } = new("Components", "Meter", [
        new("Overview", theme => ComponentExamples.Meter(theme, "Overview"), StoryLayout.Page),
        new("Bare", theme => ComponentExamples.Meter(theme, "Bare"), StoryLayout.Centered),
        new("With value", theme => ComponentExamples.Meter(theme, "With value"), StoryLayout.Centered),
        new("Sizes", theme => ComponentExamples.Meter(theme, "Sizes"), StoryLayout.Centered),
        new("Tints", theme => ComponentExamples.Meter(theme, "Tints"), StoryLayout.Centered),
        new("In a quota list", theme => ComponentExamples.Meter(theme, "In a quota list"), StoryLayout.Centered),
    ]);
}
