# Program Execution

- URL: https://windowsir.blogspot.com/2023/04/program-execution.html
- Published: 2023-04-27T19:36:00.003-05:00
- Updated: 2023-04-27T19:40:22.058-05:00
- Labels: none

By now, I hope you've had a chance to read and consider the posts I've written discussing the need for  validation of findings ( third one here ). Part of the reason for this series was a pervasive over-reliance on single artifacts as a source of findings that I and others have seen within the community over the past 2+ decades. One of the most often repeated examples of this is relying on ShimCache or AmCache artifacts as evidence of program execution.
 ShimCache
 ShimCache, or AppCompatCache (the name of the Registry value where the data is found) is often looked to as evidence of program execution when really what it demonstrates is that the file was on the system.
 From this blog post from Mandiant :
 It is important to understand there may be entries in the Shimcache that were not actually executed.
 There you go. That's from 2015. And this is why we need to incorporate artifacts such as the ShimCache into an overall constellation, rather then viewing artifacts such as these in isolation. This 13Cubed video provides a clear explanation regarding the various aspects of the ShimCache artifact as it relates to Windows 10; note that the title of the video includes "the most misunderstood artifact".
 AmCache
 AmCache is another one of those artifacts that is often offered up as "evidence of program execution", as seen in this LinkedIn post . However, the first referenced URL in that post belies the fact that this artifact is "evidence of program execution", as well as other statements in the post (i.e., that AmCache is "populated after system shutdown"). From the blog post:

 During these tests, it was found that the Amcache hive may have artifacts for executables that weren’t executed at all.
 A bit more extensive treatment of the AmCache artifact can be found here . While you may look at the PDF and think, "TL;DR", the short version is that an entry in the AmCache does not explicitly mean, by itself, that the file was executed.
 The point is that research demonstrates that, much like the ShimCache artifact, we cannot simply look at an entry and state, "oh, that is evidence of program execution". Even if you don't want to take the time reach and digest either the blog post or the PDF, simply understand that by itself, an AmCache entry does not demonstrate evidence of program execution.
 So, again...let's all agree to stop looking just to ShimCache or just to AmCache as evidence of program execution, and instead look to multiple data sources and to artifact constellations to establish whether a program was executed or not.
 For some insight as to how ShimCache and AmCache can be used together, check out this blog post from WithSecure .
 Keep in mind that even when combining these two artifacts, it still doesn't provide clear indications that the identified executable was launched, and successfully executed. We need to seek other artifacts (Windows Event Log, Registry, etc.) to determine this aspect of the executable.
 PCA
 Earlier this year, AboutDFIR.com published a blog post regarding a new artifact (new to Windows 11) that appears to demonstrate evidence of program execution. Much like other artifacts (see above), this one has nuances or conditions , in that you cannot look to it to demonstrate execution of all programs; rather this one seems to apply to either GUI programs, or CLI program launched via a GUI. This is important to remember, whether you see an application of interest listed in one of the artifacts, or if you don't...context matters.
 The blog post provides insight into the artifacts, as well as images of the artifacts, and samples you can download and examine yourself. This YouTube video mentions another associated artifact; specifically, Windows Event Log records of interest. Adding the artifacts to a timeline is a pretty trivial exercise; the text-based artifacts are easy to script, and the process for adding Windows Event Logs to a timeline is something that already exists.
