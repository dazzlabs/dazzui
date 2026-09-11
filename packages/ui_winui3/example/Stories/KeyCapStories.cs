// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class KeyCapStories
{
    public static StoryPage Page { get; } = new("Components", "KeyCap", [
        new("Overview", theme => ComponentExamples.KeyCap(theme, "Overview"), StoryLayout.Page),
        new("Hint", theme => ComponentExamples.KeyCap(theme, "Hint"), StoryLayout.Centered),
        new("Strong", theme => ComponentExamples.KeyCap(theme, "Strong"), StoryLayout.Centered),
        new("Key", theme => ComponentExamples.KeyCap(theme, "Key"), StoryLayout.Centered),
        new("All variants", theme => ComponentExamples.KeyCap(theme, "All variants"), StoryLayout.Centered),
        new("Sizes", theme => ComponentExamples.KeyCap(theme, "Sizes"), StoryLayout.Centered),
        new("A chord", theme => ComponentExamples.KeyCap(theme, "A chord"), StoryLayout.Centered),
        new("A joined chord", theme => ComponentExamples.KeyCap(theme, "A joined chord"), StoryLayout.Centered),
        new("Word keys", theme => ComponentExamples.KeyCap(theme, "Word keys"), StoryLayout.Centered),
        new("As a search hint", theme => ComponentExamples.KeyCap(theme, "As a search hint"), StoryLayout.Centered),
        new("In a settings row", theme => ComponentExamples.KeyCap(theme, "In a settings row"), StoryLayout.Centered),
        new("In a menu", theme => ComponentExamples.KeyCap(theme, "In a menu"), StoryLayout.Centered),
    ]);
}
