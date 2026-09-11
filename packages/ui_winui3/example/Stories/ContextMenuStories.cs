// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class ContextMenuStories
{
    public static StoryPage Page { get; } = new("Components", "ContextMenu", [
        new("Overview", theme => ComponentExamples.ContextMenu(theme, "Overview"), StoryLayout.Page),
        new("Bare", theme => ComponentExamples.ContextMenu(theme, "Bare"), StoryLayout.Centered),
        new("Checked", theme => ComponentExamples.ContextMenu(theme, "Checked"), StoryLayout.Centered),
        new("On a row", theme => ComponentExamples.ContextMenu(theme, "On a row"), StoryLayout.Centered),
    ]);
}
