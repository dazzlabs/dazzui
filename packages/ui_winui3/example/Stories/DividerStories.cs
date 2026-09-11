// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class DividerStories
{
    public static StoryPage Page { get; } = new("Components", "Divider", [
        new("Overview", theme => ComponentExamples.Divider(theme, "Overview"), StoryLayout.Page),
        new("Horizontal", theme => ComponentExamples.Divider(theme, "Horizontal"), StoryLayout.Centered),
        new("Muted", theme => ComponentExamples.Divider(theme, "Muted"), StoryLayout.Centered),
        new("Tones", theme => ComponentExamples.Divider(theme, "Tones"), StoryLayout.Centered),
        new("Vertical", theme => ComponentExamples.Divider(theme, "Vertical"), StoryLayout.Centered),
        new("In a toolbar", theme => ComponentExamples.Divider(theme, "In a toolbar"), StoryLayout.Centered),
        new("In a card", theme => ComponentExamples.Divider(theme, "In a card"), StoryLayout.Centered),
        new("Between rows", theme => ComponentExamples.Divider(theme, "Between rows"), StoryLayout.Centered),
    ]);
}
