// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class TableStories
{
    public static StoryPage Page { get; } = new("Components", "Table", [
        new("Overview", theme => ComponentExamples.Table(theme, "Overview"), StoryLayout.Page),
        new("Selectable", theme => ComponentExamples.Table(theme, "Selectable"), StoryLayout.Centered),
        new("Inert", theme => ComponentExamples.Table(theme, "Inert"), StoryLayout.Centered),
        new("Alignment", theme => ComponentExamples.Table(theme, "Alignment"), StoryLayout.Centered),
        new("With controls", theme => ComponentExamples.Table(theme, "With controls"), StoryLayout.Centered),
        new("Tints", theme => ComponentExamples.Table(theme, "Tints"), StoryLayout.Centered),
        new("Empty", theme => ComponentExamples.Table(theme, "Empty"), StoryLayout.Centered),
    ]);
}
