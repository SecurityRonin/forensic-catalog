# Post Compilation

- URL: https://windowsir.blogspot.com/2022/11/post-compilation.html
- Published: 2022-11-28T20:47:00.004-05:00
- Updated: 2022-11-28T20:49:17.104-05:00
- Labels: none

Investigating Windows Systems
 It's the time of year again when folks are looking for stocking stuffers for the DFIR nerd in their lives, and my recommendation is a copy of Investigating Windows Systems ! The form factor for the book makes it a great stocking stuffer, and the content is well worth it!
 Yes, I know that book was published in 2018, but when I set out to write the book, I wanted to do something different from the recipe of most DFIR books to that point, including my own. I wanted to write something that addressed the analysis process, so the book is full of pivot and decision points, etc. So, while artifacts may change over time...some come and go, others become new and change in format over time, others suddenly appear...it's the analysis process that doesn't change.
 For example, chapter 4 addresses the analysis of a compromised web server , one that includes a memory dump. One of the issues I've run into over the past couple of years, since well after the book was published, is that there more than a few DFIR analysts who seem to believe that running a text search of a memory dump for IP addresses is "sufficient"; it's not. IP addresses are not often stored in ASCII format; as such, you'd likely want to use Volatility and bulk_extractor to locate the specific structures that include the binary representation of the IP address. As each tool looks for different structures, I recommend using them both...just look at ch 4 of IWS and see how different the information is between the two tools.
 There's a lot of really good content in the book, such as "file system tunneling", covered beginning on pg 101.
 While some of the images used as the basis of analysis in the book are no longer available online, several are still available, and the overall analysis process applies regardless of the image.
 Analysis
 Speaking of analysis processes, I ran across this blog post recently, and it touched on a couple of very important concepts, particularly:
 This highlights the risk of interpreting single artefacts (such as an event record, MFT entry, etc) in isolation, as it doesn't provide any context and is (potentially) subject to misinterpretation.
 Exactly! When we view artifacts in isolation, we're missing critical factors such as context, and in a great many instances, grossly misinterpreting the "evidence" . This misinterpretation happens a lot more than we'd like to think, not due to a lack of visibility, but due to it simply being the DFIR culture.
 Another profound statement from the author was:

 ...instead of fumbling and guessing, I reached out to @randomaccess and started discussing plausible scenarios.
 Again...exactly! Don't guess. Don't spackle gaps in analysis over with assumption and speculation. It's okay to fumble, as long as you learn from it. However, most importantly, there's no shame in asking for help. In fact, it's quite the opposite. Don't listen to that small voice insider of you that's giving you excuses, like, "...oh, they're too busy...", or "...I could never ask them...". Instead, listen the roaring Gunnery Sergeant Hartmann (from "Full Metal Jacket") who's screaming at you to reach out and ask someone, Private Joker!!
 For me, it's very validating to see others within the industry advocating the same approach I've been sharing for several years. Cyber defense is a team sport folks, and going it alone just means that we, and our customers, are going to come up short.
 Tools for Memory Analysis
 In addition to the tools for memory analysis mentioned earlier in this blog post, several others have popped over time. For example, here're two:

 MemProcFS
 ProcMemScan
 Now, I haven't tried either one of these tools, but they seem pretty great.
 Additional Resources:
 CyberHacktics - Win10 Memory Analysis

 Proactive Defense
 "Proactive defense" means moving "left of bang", taking steps to inhibit or even obviate the threat actor, before or shortly after they gain initial access. For example, TheHackerNews recently reported on the Black Basta Ransomware gang , indicating that one means of gaining access is to coerce or trick a user into mounting a disk image (IMG) file and launching the VBS script embedded within it, to initially infect the system with Qakbot. Many have seen a similar technique to infect systems with Qakbot, sending ISO files with embedded LNK files.
 So, think about it...do your users require the ability to mount disk image files simply by double-clicking them? If not, consider taking these steps to address this issue; doing so will still allow your users to programmatically access disk image files, but will prevent them from mounting them by double-clicking, or by right-clicking and choosing "Mount" from the context menu. This quite literally cuts the head off of the attack, stopping the threat actor in their tracks.
 Taking proactive security steps...creating an accurate asset inventory (of both systems and applications), reducing your attack surface, and configuring systems beyond the default...means that you're going to have higher fidelity alerts, with greater context, which in turn helps alleviate alert fatigue for your SOC analysts.
 Open Reporting
 Lots of us pursue/review open reporting when it comes to researching issues. I've done this more than a few times, searching for unique terms I find (i.e., Registry value names, etc.), first doing a wide search, then narrowing it a bit to try to find more specific information.
 However, I strongly caveat this approach, in part due to open reporting like this write-up on Raspberry Robin , specifically due to the section on Persistence. That section starts with (emphasis added by me):

 Raspberry Robin installs itself into a registry “run” key in the Windows user’s hive , for example:
 However, the key pointed to is "software\microsoft\windows\currentversion\runonce\". The Run key is very different from the RunOnce key, particularly regarding how it's handled by the OS.
 Within that section are two images, neither of which is numbered. The caption for the second image reads:

 Raspberry Robin persistence process following an initial infection and running at each machine boot
 Remember where I bolded "user's hive" above? Simply by the fact that persistence is written to a user's hive means that the process starts following the next time that user logs in, not "at each machine boot".
 Open reporting can be very valuable during analysis, and can provide insight that an analyst may not have otherwise. However, open reporting does need to be reviewed with a critical eye, and not simply taken at face value.
