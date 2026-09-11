// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class OtpFieldStories
{
    public static StoryPage Page { get; } = new("Components", "OtpField", [
        new("Overview", theme => ComponentExamples.OtpField(theme, "Overview"), StoryLayout.Page),
        new("Bare", theme => ComponentExamples.OtpField(theme, "Bare"), StoryLayout.Centered),
        new("Filled", theme => ComponentExamples.OtpField(theme, "Filled"), StoryLayout.Centered),
        new("Grouped", theme => ComponentExamples.OtpField(theme, "Grouped"), StoryLayout.Centered),
        new("Sizes", theme => ComponentExamples.OtpField(theme, "Sizes"), StoryLayout.Centered),
        new("Masked", theme => ComponentExamples.OtpField(theme, "Masked"), StoryLayout.Centered),
        new("Disabled", theme => ComponentExamples.OtpField(theme, "Disabled"), StoryLayout.Centered),
        new("In a form", theme => ComponentExamples.OtpField(theme, "In a form"), StoryLayout.Centered),
    ]);
}
