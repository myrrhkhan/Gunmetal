# Gunmetal: A Sophisticated Environment Variable Editor

Gunmetal is a program to view all of your computer's consistent environment variables, similar to the Edit Environment Variables dialog in Windows, but cross-platform and modern.

## Why?

As much as I prefer performing many tasks in the terminal, I missed having the option of viewing and editing all of my computer's environment variables with a GUI, like I used to on Windows before I switched to Mac.

I wanted to learn a little bit of frontend development and get into Rust at the same time. When I learned about the [Tauri](https://www.youtube.com/watch?v=-X8evddpu7M) framework,
I found out it was the perfect way to learn both technologies, and decided that an environment variable editor would be the perfect first project, as it combines many different technologies, and can be simple but is also easy to scale up.

## Is it functional?

For Windows, yes. For other platforms, also yes, but it's wonky with the PATH variable, which is probably the most important variable. It can read environment variables when the user gives a path to one shell profile, but I am having a couple of issues, and it might be due to the fact that I might just need to find all of the files that may contain necessary attributes of the variables.

## Why the name?

I'm bad at naming things, but I decided that I had to name this thing something other than "Environment Variable Editor."
There is this C++ tutorial playlist on YouTube that I use often, and the guy who runs it (a channel called The Cherno whom I'd highly recommend) has videos where he details a game engine he's making called Hazel. I thought "why not name it after a color?"
I went onto a color palette generator and kept picking color palettes until I found one with gunmetal, and thought "that's a cool name."
The added benefit is that I know have a color scheme I can start implementing.
