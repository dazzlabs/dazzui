// Story names synchronized with the Flutter/SwiftUI playgrounds.
using DazzUI.Playground.Storybook;

namespace DazzUI.Playground.Stories;

public static class AvatarStories
{
    public static StoryPage Page { get; } = new("Components", "Avatar", [
        new("Overview", theme => ComponentExamples.Avatar(theme, "Overview"), StoryLayout.Page),
        new("Initials", theme => ComponentExamples.Avatar(theme, "Initials"), StoryLayout.Centered),
        new("Image", theme => ComponentExamples.Avatar(theme, "Image"), StoryLayout.Centered),
        new("Square", theme => ComponentExamples.Avatar(theme, "Square"), StoryLayout.Centered),
        new("Sizes", theme => ComponentExamples.Avatar(theme, "Sizes"), StoryLayout.Centered),
        new("Tints", theme => ComponentExamples.Avatar(theme, "Tints"), StoryLayout.Centered),
        new("Groups", theme => ComponentExamples.Avatar(theme, "Groups"), StoryLayout.Centered),
    ]);
}
