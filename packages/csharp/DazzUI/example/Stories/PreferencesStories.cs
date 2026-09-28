// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class PreferencesStories
{
    public static StoryPage Page { get; } = new("Components", "Preferences", [
        new("Overview", theme => ComponentExamples.Preferences(theme, "Overview"), StoryLayout.Page),
        new("A section", theme => ComponentExamples.Preferences(theme, "A section"), StoryLayout.Centered),
        new("Row shapes", theme => ComponentExamples.Preferences(theme, "Row shapes"), StoryLayout.Centered),
        new("Controls", theme => ComponentExamples.Preferences(theme, "Controls"), StoryLayout.Centered),
        new("Section headings", theme => ComponentExamples.Preferences(theme, "Section headings"), StoryLayout.Centered),
        new("Unlabelled", theme => ComponentExamples.Preferences(theme, "Unlabelled"), StoryLayout.Centered),
        new("A group", theme => ComponentExamples.Preferences(theme, "A group"), StoryLayout.Centered),
        new("A settings page", theme => ComponentExamples.Preferences(theme, "A settings page"), StoryLayout.Centered),
    ]);
}
