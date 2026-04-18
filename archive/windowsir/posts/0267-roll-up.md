# Roll-up

- URL: https://windowsir.blogspot.com/2023/11/roll-up_28.html
- Published: 2023-11-28T18:06:00.000-05:00
- Updated: 2023-11-28T18:06:47.803-05:00
- Labels: none

One of the things I love about the industry is that it's like fashion...given enough time, the style that came and went comes back around again. Much like the fashion industry, we see things time and again...just wait.
 A good example of this is the finger application. I first encountered finger toward the end of 1994,

 Jump forward about 29 years to just recently, and I saw finger.exe, on a Windows system, used for data exfiltration . John Page/hyp3rlinx wrote an advisory (published 2020-09-11) describing how to do this, and yes, from the client side, what I saw looked like it was taken directly from John's advisory.
 What this means to us is that the things we learn may feel like they fade with time, but wait long enough, and you'll see them, or some variation, again. I've seen this happen with ADSs; more recently, the specific MotW variations have taken precedence. I've also seen it happen with shell items (i.e., the "building blocks" of LNK files, JumpLists, and shellbags), as well as with the OLE file format. You may think, "...man, I spent all that time learning about that thing, and now it's no longer used..."; wait. It'll come back, like bell bottoms.
 Deleted Things
 In DFIR, we often say that just because you delete something, that doesn't mean that it's gone. For files, Registry keys and values, etc., this is all very true.
 Scheduled Tasks
 A while back, I blogged about an ops debrief call that I'd joined, and listened to an analyst discuss their findings from their engagement. At the beginning of the call, they'd mentioned something, almost in passing, glossing over it like it was inconsequential; however, some research revealed that it was actually an extremely high-fidelity indicator based on specific threat actor TTPs.
 In many instances, threat actors will create Scheduled Tasks as a means of persisting on endpoints. In fact, not too long ago, I saw a threat actor create two Scheduled Tasks for the same command; one to run based on a time trigger, and the other to run ONSTART.
 In the case this analyst was discussing, the threat actor had created a Scheduled Task on a Windows 7 (like I said, this was a while back) system. The task was for a long-running application; essentially, the application would run until it was specifically stopped, either directly or by the system being turned off. Once the application was launched, the threat actor deleted the Scheduled Task, removing to the XML and binary task files; Windows 7 used a combination of the XML-format task files we see today on Windows 10 and 11 endpoints, as well as the binary *.job file format we saw on Windows XP.
 Volume Shadow Copies
 About 7 yrs ago or so, I published a blog post that included a reference to a presentation from 2016, and to a Carbon Black blog post that had been published in August, 2015. The short version of what was discussed in both was that a threat actor performed the following:
 1. Copied their malware EXE to the root of a file system.
 2. Created a Volume Shadow Copy (VSC).
 3. Mounted the VSC they'd created, and launched the malware/Trojan EXE from within the mounted VSC.
 4. Deleted the VSC they'd created, leaving the malware EXE running in memory.
 I tried replicating this...and it worked. Not a great persistence mechanism...reboot the endpoint and it's no longer infected...but fascinating nonetheless. What's interesting about this approach is that if the endpoint hadn't had an EDR agent installed, all a responder would have available to them by dumping process information from the live endpoint, or by grabbing a memory dump, is a process command line with a file path that didn't actually exist on the endpoint.
 WSL
 We've known about the Windows Subsystem for Linux (WSL) for a while.
 Not too long ago, an academic paper addressing WSL2 forensics was published illustrating artifacts associated with the installation and use of Linux distributions. The authors reference the use of RegRipper (version 3.0, apparently) in several locations, particularly when examining the System and Software Registry hives; for some reason, they chose to not use RegRipper to parse the AmCache.hve file.
 Now, let's keep our eyes open for a similar paper on the Windows Subsystem for Android...just sayin'...
