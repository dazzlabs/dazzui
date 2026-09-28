// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class PillTabsStories
{
    public static StoryPage Page { get; } = new("Components", "PillTabs", [
        new("Overview", theme => ComponentExamples.PillTabs(theme, "Overview"), StoryLayout.Page),
        new("Default", theme => ComponentExamples.PillTabs(theme, "Default"), StoryLayout.Centered),
        new("With counts", theme => ComponentExamples.PillTabs(theme, "With counts"), StoryLayout.Centered),
        new("Disabled", theme => ComponentExamples.PillTabs(theme, "Disabled"), StoryLayout.Centered),
        new("Tints", theme => ComponentExamples.PillTabs(theme, "Tints"), StoryLayout.Centered),
        new("Wrapping", theme => ComponentExamples.PillTabs(theme, "Wrapping"), StoryLayout.Centered),
        new("In a pane header", theme => ComponentExamples.PillTabs(theme, "In a pane header"), StoryLayout.Centered),
    ]);
}
