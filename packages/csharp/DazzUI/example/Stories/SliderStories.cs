// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class SliderStories
{
    public static StoryPage Page { get; } = new("Components", "Slider", [
        new("Overview", theme => ComponentExamples.Slider(theme, "Overview"), StoryLayout.Page),
        new("Bare", theme => ComponentExamples.Slider(theme, "Bare"), StoryLayout.Centered),
        new("With value", theme => ComponentExamples.Slider(theme, "With value"), StoryLayout.Centered),
        new("Range", theme => ComponentExamples.Slider(theme, "Range"), StoryLayout.Centered),
        new("Sizes", theme => ComponentExamples.Slider(theme, "Sizes"), StoryLayout.Centered),
        new("Tints", theme => ComponentExamples.Slider(theme, "Tints"), StoryLayout.Centered),
        new("Disabled", theme => ComponentExamples.Slider(theme, "Disabled"), StoryLayout.Centered),
        new("Interactive", theme => ComponentExamples.Slider(theme, "Interactive"), StoryLayout.Centered),
        new("In settings", theme => ComponentExamples.Slider(theme, "In settings"), StoryLayout.Centered),
    ]);
}
