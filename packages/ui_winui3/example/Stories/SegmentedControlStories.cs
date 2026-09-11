// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class SegmentedControlStories
{
    public static StoryPage Page { get; } = new("Components", "SegmentedControl", [
        new("Overview", theme => ComponentExamples.SegmentedControl(theme, "Overview"), StoryLayout.Page),
        new("Raised", theme => ComponentExamples.SegmentedControl(theme, "Raised"), StoryLayout.Centered),
        new("Filled", theme => ComponentExamples.SegmentedControl(theme, "Filled"), StoryLayout.Centered),
        new("Sizes", theme => ComponentExamples.SegmentedControl(theme, "Sizes"), StoryLayout.Centered),
        new("Tints", theme => ComponentExamples.SegmentedControl(theme, "Tints"), StoryLayout.Centered),
        new("Filled tints", theme => ComponentExamples.SegmentedControl(theme, "Filled tints"), StoryLayout.Centered),
        new("Stretch", theme => ComponentExamples.SegmentedControl(theme, "Stretch"), StoryLayout.Centered),
        new("Disabled", theme => ComponentExamples.SegmentedControl(theme, "Disabled"), StoryLayout.Centered),
        new("Two segments", theme => ComponentExamples.SegmentedControl(theme, "Two segments"), StoryLayout.Centered),
        new("In a form", theme => ComponentExamples.SegmentedControl(theme, "In a form"), StoryLayout.Centered),
    ]);
}
