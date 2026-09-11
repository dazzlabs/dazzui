using DazzUI;
using Microsoft.UI.Xaml;

namespace DazzUI.Playground.Storybook;

public enum StoryLayout { Centered, Page, Fullscreen }
public sealed record Story(string Name, Func<Theme, UIElement> Build, StoryLayout Layout = StoryLayout.Centered);
public sealed record StoryPage(string Group, string Name, IReadOnlyList<Story> Stories);
