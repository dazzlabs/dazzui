// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class ToastStories
{
    public static StoryPage Page { get; } = new("Components", "Toast", [
        new("Overview", theme => ComponentExamples.Toast(theme, "Overview"), StoryLayout.Page),
        new("Default", theme => ComponentExamples.Toast(theme, "Default"), StoryLayout.Centered),
        new("Tints", theme => ComponentExamples.Toast(theme, "Tints"), StoryLayout.Centered),
        new("Shapes", theme => ComponentExamples.Toast(theme, "Shapes"), StoryLayout.Centered),
        new("Truncation", theme => ComponentExamples.Toast(theme, "Truncation"), StoryLayout.Centered),
        new("A stack", theme => ComponentExamples.Toast(theme, "A stack"), StoryLayout.Fullscreen),
        new("In a top viewport", theme => ComponentExamples.Toast(theme, "In a top viewport"), StoryLayout.Fullscreen),
        new("Posting one", theme => ComponentExamples.Toast(theme, "Posting one"), StoryLayout.Fullscreen),
    ]);
}
