# Question on Open Source Tools

- URL: https://windowsir.blogspot.com/2025/12/question-on-open-source-tools.html
- Published: 2025-12-29T13:16:00.002-05:00
- Updated: 2025-12-29T13:16:17.477-05:00
- Labels: none

Is there any update on reg tool?
 After a little more back and forth, I was able to tease out that the question was about RegRipper, and the question was really directed more at asking about reg keys updates for win11 . My response was the usual, that everything's online. After all, in addition to my blog, the GitHub repo is publicly available, so anyone can take a look at it and see what's new. I mean, I don't have to do your Googling for you. If you don't see something specific that you're looking for, RegRipper was designed from the beginning to be extensible; you can either write your own plugin, or ask for assistance in doing so (I've turned working plugins around in an hour or less). If you choose to go that route (most don't), it usually helps if you can clearly articulate what you're looking for, and even more so if you can provide a Registry hive for testing.
 But the question itself got me to thinking, is this what we've come to?
 The answer is pretty simple. No, this is how things have always been, that the initial interest in any tool, especially one that's freely available is, is there anything new and shiny? More so, the majority of this attention seems to be just asking the high level question, with no articulation of anything specific. Just, "what's new?"
 In the time I've been in the industry, there's an inordinate focus on what's "new" over mastering what's already out there and available.
 On the topic of what's new, in Aug 2023, I published this blog post regarding updates I'd made to RegRipper, specifically version 4.0 . While there were no specific plugins called out in the blog post, and nothing specific about updates for Registry keys or values specific to Windows 11, I did describe writing a plugin to write output in JSON format, and incorporating Yara into RegRipper.
 In October 2024, I spoke at the inaugural From The Source conference , basically walking through my Aug 2023 blog post.
 My question is, is 2026 the year that some commercial tool provider will want to incorporate RegRipper functionality directly into the application?
 In 2018, while I was at Nuix, extensions were created to incorporate Yara and RegRipper into the Nuix Workstation product . Here's my blog post describing this capability. I know that in June 2022, RegRipper v3.0 was added to Paraben's E3 product, albeit at that time, on a very limited basis.
