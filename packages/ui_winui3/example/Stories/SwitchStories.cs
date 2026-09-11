// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class SwitchStories
{
    public static StoryPage Page { get; } = new("Components", "Switch", [
        new("Overview", theme => ComponentExamples.Switch(theme, "Overview"), StoryLayout.Page),
        new("On", theme => ComponentExamples.Switch(theme, "On"), StoryLayout.Centered),
        new("Off", theme => ComponentExamples.Switch(theme, "Off"), StoryLayout.Centered),
        new("Interactive", theme => ComponentExamples.Switch(theme, "Interactive"), StoryLayout.Centered),
        new("Disabled", theme => ComponentExamples.Switch(theme, "Disabled"), StoryLayout.Centered),
        new("Sizes", theme => ComponentExamples.Switch(theme, "Sizes"), StoryLayout.Centered),
        new("Tints", theme => ComponentExamples.Switch(theme, "Tints"), StoryLayout.Centered),
        new("Against a checkbox", theme => ComponentExamples.Switch(theme, "Against a checkbox"), StoryLayout.Centered),
        new("In settings", theme => ComponentExamples.Switch(theme, "In settings"), StoryLayout.Centered),
    ]);
}
