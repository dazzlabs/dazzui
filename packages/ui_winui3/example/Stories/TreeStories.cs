// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class TreeStories
{
    public static StoryPage Page { get; } = new("Components", "Tree", [
        new("Overview", theme => ComponentExamples.Tree(theme, "Overview"), StoryLayout.Page),
        new("File tree", theme => ComponentExamples.Tree(theme, "File tree"), StoryLayout.Centered),
        new("Sizes", theme => ComponentExamples.Tree(theme, "Sizes"), StoryLayout.Centered),
        new("With meta", theme => ComponentExamples.Tree(theme, "With meta"), StoryLayout.Centered),
        new("Tints", theme => ComponentExamples.Tree(theme, "Tints"), StoryLayout.Centered),
        new("Collapsed", theme => ComponentExamples.Tree(theme, "Collapsed"), StoryLayout.Centered),
        new("Disabled", theme => ComponentExamples.Tree(theme, "Disabled"), StoryLayout.Centered),
    ]);
}
