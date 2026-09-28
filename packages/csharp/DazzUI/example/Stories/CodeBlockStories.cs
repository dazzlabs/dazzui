// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class CodeBlockStories
{
    public static StoryPage Page { get; } = new("Components", "CodeBlock", [
        new("Overview", theme => ComponentExamples.CodeBlock(theme, "Overview"), StoryLayout.Page),
        new("Plain", theme => ComponentExamples.CodeBlock(theme, "Plain"), StoryLayout.Centered),
        new("With head", theme => ComponentExamples.CodeBlock(theme, "With head"), StoryLayout.Centered),
        new("Line numbers", theme => ComponentExamples.CodeBlock(theme, "Line numbers"), StoryLayout.Centered),
        new("Copyable", theme => ComponentExamples.CodeBlock(theme, "Copyable"), StoryLayout.Centered),
        new("Wrapped", theme => ComponentExamples.CodeBlock(theme, "Wrapped"), StoryLayout.Centered),
        new("With actions", theme => ComponentExamples.CodeBlock(theme, "With actions"), StoryLayout.Centered),
    ]);
}
