# Questions I've Been Asked

- URL: https://windowsir.blogspot.com/2026/01/questions-ive-been-asked.html
- Published: 2026-01-05T12:21:00.001-05:00
- Updated: 2026-01-05T12:21:06.774-05:00
- Labels: none

Sometimes I'll get questions via different routes...webinars or podcasts, via social media, DM, or even email. Getting questions is good, because it keeps me aware that I'm in somewhat of a bubble, given the work I do and the environment in which I do it. Given the nature of "social" media (hint: it's rarely "social"), it's tough to draw a bead on where you are at any given moment, so questions can be invaluable.
 Here's an interesting question I got from Brian Carrier during a webinar he invited me to...
 If you have the entire Registry and limited time, what do you do?
 The Cyber Triage LinkedIn post has 9 pages, and as you can see from the first one, my answer to the above question is:
 I cheat.
 For me, it's pretty simple. Beginning with the second slide from that LinkedIn post, I explain what I mean by " I cheat ". I have my parsing tools (which I've shared ), and I enrich or "decorate" the output based on what I've seen on previous engagements, or gathered from write-ups and information shared online.
 For example, this recent blog post references a write up that describes how Valley RAT stores configuration information and plugins with a specific Registry path. Rather than parsing the entire Registry and looking through that massive amount of information, hoping that I'll remember the Registry path mentioned, I write a plugin that searches for it specifically, and alerts me if it's found. That way, not only do I not have to remember a ton of paths, keys and values, I now have a documented plugin with a publish (and update) date, links/URLs pointing to online references, etc. As many who use RegRipper are aware, I've including Analysis Notes in plugins, describing what to look for, and how the output of the plugin can be used.
 In addition, almost 2 1/2 yrs ago, I got Yara running via RegRipper , as well, significantly expanding the capabilities of both tool sets. So, in a way, I'm "cheating" by leveraging a unique capability.

 How does your methodology change when you are investigating a domain controller or server versus a typical workstation?
 My answer was, for the the most part...it doesn't. Yes, artifacts may vary depending upon the nature of the endpoint (i.e., server vs workstation, Windows 11, etc.) and what's installed, but the methodology generally doesn't change, unless my analysis goals change. And I've used the same essential methodology for almost 2 decades now, expanding and developing it along the way.
 Another question:
 When will you write a book on Linux forensics?
 Yes, I actually received this question, and not just recently. And I've received it before. Look, there's nothing to indicate, anywhere, that I know enough about performing forensics on Linux systems to write a book, let alone a blog post. I did just talk to someone yesterday (I was serving as a greeter at church) about a Xenix firewall I examined years ago; I was able to image the hard drive, and then open the image in EnCase, but I had no idea where to begin with analysis, particularly given that logging appeared to have been disabled.
 Also, there are a LOT of books available on the topic already, so why would I want to put in the effort and expense to just become part of the background noise?
 Here's another question I've received a number of times over the years:
 Where can I find a list of free images from real incidents?
 This one is easy. First, it's unlikely that you're going to find images...disk, memory, triage...from "real" incidents anywhere online. If you do, please let me know. I've been aware of images shared for some time that are contrived scenarios and/or part of CTFs, but I can't image someone would share an image, even a triage collection, from a real, live incident. If you know of someone who did, and the link is still valid, I'd love it if you could share it.

 Second, I've posted lists of available images multiple times in this blog, the most recent of which can be found here .
