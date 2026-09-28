// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class SectionLabelStories
{
    public static StoryPage Page { get; } = new("Components", "SectionLabel", [
        new("Overview", theme => ComponentExamples.SectionLabel(theme, "Overview"), StoryLayout.Page),
        new("Default", theme => ComponentExamples.SectionLabel(theme, "Default"), StoryLayout.Centered),
        new("Faint", theme => ComponentExamples.SectionLabel(theme, "Faint"), StoryLayout.Centered),
        new("Tints", theme => ComponentExamples.SectionLabel(theme, "Tints"), StoryLayout.Centered),
        new("In a panel", theme => ComponentExamples.SectionLabel(theme, "In a panel"), StoryLayout.Centered),
        new("In a column", theme => ComponentExamples.SectionLabel(theme, "In a column"), StoryLayout.Centered),
    ]);
}
