// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class FormStories
{
    public static StoryPage Page { get; } = new("Components", "Form", [
        new("Overview", theme => ComponentExamples.Form(theme, "Overview"), StoryLayout.Page),
        new("Bare", theme => ComponentExamples.Form(theme, "Bare"), StoryLayout.Centered),
        new("With fieldsets", theme => ComponentExamples.Form(theme, "With fieldsets"), StoryLayout.Centered),
        new("Server errors", theme => ComponentExamples.Form(theme, "Server errors"), StoryLayout.Centered),
    ]);
}
