// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class DialogStories
{
    public static StoryPage Page { get; } = new("Components", "Dialog", [
        new("Overview", theme => ComponentExamples.Dialog(theme, "Overview"), StoryLayout.Page),
        new("A sheet", theme => ComponentExamples.Dialog(theme, "A sheet"), StoryLayout.Fullscreen),
        new("An alert", theme => ComponentExamples.Dialog(theme, "An alert"), StoryLayout.Fullscreen),
        new("A confirmation", theme => ComponentExamples.Dialog(theme, "A confirmation"), StoryLayout.Centered),
        new("Danger", theme => ComponentExamples.Dialog(theme, "Danger"), StoryLayout.Centered),
        new("A form", theme => ComponentExamples.Dialog(theme, "A form"), StoryLayout.Centered),
        new("Widths", theme => ComponentExamples.Dialog(theme, "Widths"), StoryLayout.Centered),
        new("Body only", theme => ComponentExamples.Dialog(theme, "Body only"), StoryLayout.Centered),
        new("Without a subtitle", theme => ComponentExamples.Dialog(theme, "Without a subtitle"), StoryLayout.Centered),
        new("Behind a trigger", theme => ComponentExamples.Dialog(theme, "Behind a trigger"), StoryLayout.Fullscreen),
    ]);
}
