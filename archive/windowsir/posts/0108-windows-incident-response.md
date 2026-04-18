# Windows Incident Response

- URL: https://windowsir.blogspot.com/2013/07/howto-determine-program-execution.html
- Published: unknown
- Updated: unknown
- Labels: none

Windows Incident Response
 The Windows Incident Response Blog is dedicated to the myriad information surrounding and inherent to the topics of IR and digital analysis of Windows systems. This blog provides information in support of my books; "Windows Forensic Analysis" (1st thru 4th editions), "Windows Registry Forensics",
as well as the book I co-authored with Cory Altheide, "Digital Forensics with Open Source Tools".
 Saturday, July 06, 2013
 HowTo: Determine Program Execution

 29 comments:
 I like these HowTo posts, especially this one. What about AV logs, or Windows Firewall exceptions?
 What about AV logs, or Windows Firewall exceptions?

 I'm including those in a different post.

 Are there any other topics that you think would be of interest?
 A post covering persistence mechanisms that Autoruns misses, and how to find them.

 Some posts on Windows forensics that's more relevant to the larger network, such as; how to determine the scope of an incident, how to find and track lateral movement, or how to use intelligence from one victim to detect other victims on the network.

 How to use Excel for analysis? I thought colorizing a timeline in the link below was creative. I wouldn't have thought of that, so if there are other creative and useful ways to use Excel for analyzing timelines or other digital evidence, a post on that would be helpful.

 http://computer-forensics.sans.org/blog/2012/01/25/digital-forensic-sifting-colorized-super-timeline-template-for-log2timeline-output-files#
 A post covering persistence mechanisms that Autoruns misses, and how to find them.

 Do you have some particular ones in mind? I have some ideas regarding the DLL Search Order issue, but what other persistence mechanisms would you like to see addressed?

 The rest are interesting...I'll see what I can come up with.
 Harlan,

 These posts have been amazing and a daily read for me every morning when there is a new one. As a recent grad just starting in the field, it's really helpful to not only have a recap of some topics, but also to see things from a different perspective.

 I agree that a topic on advanced excel analysis would be very useful. Perhaps another interesting topic could be data exfiltration via cloud services and email, how to determine whether or not it occurred.

 Looking forward to see whats next!
 Harlan,

 These posts have been amazing and a daily read for me every morning when there is a new one. As a recent grad just starting in the field, it's really helpful to not only have a recap of some topics, but also to see things from a different perspective.

 I agree that a topic on advanced excel analysis would be very useful. Perhaps another interesting topic could be data exfiltration via cloud services and email, how to determine whether or not it occurred.

 Looking forward to see whats next!
 Ethan,

 What do you mean by advanced excel analysis ?

 Thanks.
 Similar to how anonymous stated,

 How to use Excel for analysis? I thought colorizing a timeline in the link below was creative. I wouldn't have thought of that, so if there are other creative and useful ways to use Excel for analyzing timelines or other digital evidence, a post on that would be helpful

 I personally underestimated how useful excel is. For example, a common thing I've come across so far is the need to manipulate paths to fit my encase naming convention in order to create successful conditions. Often, compressed archive paths tend to export out with >32,kdj| in the path. Importing this and using delimiters on > and |, replacing those with the appropriate name, such as \zip volume\, and concatenating these rows has been my standard method. Perhaps there is another, easier, way to do this though.

 I suppose by advanced I merely mean techniques that you take advantage of and use commonly in excel, whether its tools like concatenate, vlookup, or others.

 Hopefully this clarified it a little more.
 ... techniques that you take advantage of and use commonly in excel...

 If by "you", you mean me, I don't use Excel for analysis, per se. I know of folks who use manual examination processes to locate artifacts of interest, and the add them to a spreadsheet, but this is a slow, laborious process. I also know of folks who use automated means to populate spreadsheets with timeline information, and then add color-coding, but I really don't think that you can do analysis by saying, "...find two red lines, followed by a grey one, and then a blue one..."...
 DLL Search Order hijacking, MBR, binary patching (IAT hooks), rootkits that hide it, or "transient" persistence mechanisms that are placed on shutdown and removed at startup. This "Wipe the Drive" presentation has some unusual ones that I haven't heard of before:

 https://www.youtube.com/watch?v=QCZ_7C9C5AU

 If you don't use Excel much, that's fine. It could be less popular than I had thought. But I doubt Rob Lee and others are analyzing timelines just by "finding two red lines, followed by a grey one, and then a blue one." It's probably similar to malware analysts using IDA and looking for patterns of assembly, as well as reading line by line when necessary...

 As Ethan mentioned, a how to determine if sensitive data was exfiltrated would also be interesting.
 DLL Search Order hijacking, MBR, binary patching (IAT hooks)...

 I've already addressed those in this blog but I can see the value in consolidating them. Thanks for the input.

 ...or "transient" persistence mechanisms that are placed on shutdown and removed at startup

 Do you have an example of this?

 I doubt Rob Lee and others are analyzing timelines just by "finding two red lines...

 I agree with that statement, and to be honest, I'm not really clear as to why you'd say that.

 My point was about how analysis is done, in general...not how it's done by folks of expert caliber. If someone really doesn't have a detailed understanding of what artifacts and data structures are being parsed and included in a timeline, and don't understand the context of those artifacts, then putting all of that into an Excel spreadsheet and adding color-codes really isn't going to help a whole lot.

 ...a how to determine if sensitive data was exfiltrated...

 The only way to know definitively if data was exfiltrated includes having a full network capture from the time of the incident.
 Also, have you given any thought to sharing your own analysis processes?
 I unfortunately forgot where I first heard about "transient" persistence mechanisms, but I later heard the term in a SANS webcast. From what I remember, I'm pretty sure it was something she has encountered at Mandiant. Then again, I could definitely be wrong.

 https://www.sans.org/webcasts/detecting-persistence-mechanisms-96090

 It's also technique #7 here: https://isc.sans.edu/diary/Wipe+the+drive!++Stealthy+Malware+Persistence+-+Part+4/15460

 I completely misunderstood you on the color timeline. I think we're on the same page now.

 I agree to definitively determine if data was exfiltrated you need full network data. From the perspective of management, it's my understanding they may need the analyst to determine if it's "reasonably" believed data was exfiltrated to comply with certain laws.

 As an amateur with little experience, my own analysis processes won't do much good. Right now I'm more suited to reading your books and asking questions. =)

 Okay, I have a better idea of what you're asking for. Thanks for taking the time and effort to clarify that.

 Just out of curiosity, are you sure that Autoruns doesn't check the WinLogon\Notify entries?

 ... my understanding they may need the analyst to determine if it's "reasonably" believed data was exfiltrated to comply with certain laws.

 When I was doing PCI exams, I know that unless you could determine exactly which records had been exposed, you had to report on all records. Now, PCI requirements aren't laws, but the state notification laws with respect to PII are relatively easy to check. For example, here is some information I found linked from a university web site.

 As an amateur with little experience, my own analysis processes won't do much good.

 That's an all-too-common comment, and I really think that viewpoint is not only entirely incorrect, but detrimental to the community at large. The fact is that no one knows everything, and withholding information because you feel that you're too junior or don't know enough simply prevents you and everyone else from developing.

 I find that I don't learn as much if I'm being entirely passive...
 Wait a minute, you're right about WinLogon\Notify. I guess that Wipe the Drive technique is a little different than transient persistence mechanisms. I just watched the end of that SANS presentation again (1:05:00) and she said she's heard about it, but hasn't seen it. It will apparently move registry keys to the page file, or delete keys when it detects a response tool or certain user activity.

 Then a HowTo on determining which, if any, PCI data was exposed would be a good read.

 You have a point about being too passive. I was thinking a while ago about starting a blog on analyzing malware. I should probably just put up a disclaimer and try it, but from what I've heard... bloggin' ain't easy.
 One more thing. I don't know how many of these HowTo posts you have planned, but if they were combined they might be called the "Windows Forensic Cookbook."
 @Anonymous,

 ...she's heard about it, but hasn't seen it.

 Yeah, that makes it a little difficult to do much with.

 ...a HowTo on determining which, if any, PCI data was exposed would be a good read.

 Ultimately, I don't want these to be just "a good read"...I'd like others to add some of their own thoughts. Comments are good, but I can also add things to the blog that someone emails to me, giving them credit, or, if they like, not giving attribution (in the case where they want to remain anonymous).

 ..bloggin' ain't easy.

 I guess it depends on who you are. In my career, both in the military and out, I have been in positions that have required me to write, and sometimes, something like a blog is a great way to keep in practice.

 ...if they were combined...

 That's just it...I already did this sort of thing in WFA 2/e, but most folks either missed that chapter, or just forgot.

 I don't want these to be just "a good read"...I'd like others to add some of their own thoughts.

 I understand. I kind of suggested adding AV logs, or Windows Firewall exceptions to your program execution list.

 That's just it...I already did this sort of thing in WFA 2/e, but most folks either missed that chapter, or just forgot.

 Awesome. I've had WFA 3/e for a while. I actually just got the second edition a week ago and haven't had time to read it yet. I'm looking forward to that part now, thanks.
 ...suggested adding AV logs, or Windows Firewall exceptions...

 And I responded to that, thanking whomever (it's hard to tell when it seems as if several folks are posting as "anonymous" without signing their posts/comments...) shared that.

 Thanks for purchasing the books, I hope you find them useful.

 FYI, I'm currently working on an update to 3/e, and the publisher is considering using 4/e as a test case for providing updated content in near real-time.
 Harlan,

 First thanks for starting the "HowTo" posts. They are very useful as quick reference based on categories. I love it as it also helps refresh what I read/knew already.

 Can you list or talk about program installation. I saw you discuss about it under "Event Logs" section and it is new information that I am going to look right now.

 Thanks again.

 Lakshmi N
 Lakshmi,

 Thanks for the comment.

 Can you list or talk about program installation.

 Sure.

 Can you share what you currently do?
 Great How Tos and I like the new format - the colored tip boxes etc. I think these will be a valuable resource and make a great check list while working on exams.

 I did notice under the Windows shortcuts/LNK files and Jump Lists section it looks like the last sentence got cut off? Not sure if that was the last sentence or if there is supposed to be more after that?
 Mari,

 Good catch! After all the comments, you're apparently the only one who's actually read the post! ;-)
 Sure, I work in private sector and conduct computer forensic examinations on policy violation matters. Why ? Is there an issue ?
 Lakshmi,

 I'm sorry, I don't follow your question...is there an issue with respect to what?
 Harlan,

 I was trying to answer your previous question on what I do currently. I had earlier asked you to blog about "Program Installation".

 L
 Lakshmi,

 I was asking what you currently do to go about examining a system for program installation, not what you currently do for work.

 Thanks.
 As part of examining program installation or execution of setup files, I review the following areas of the registry by running the following regripper plugins,

 1. Userassist
 2. Appcompactcache
 3. Muicache
 4. Apppaths
 5. Uninstall
 6. Review installed apps under Microsoft\Windows\CurrentVersion\App Paths. Do the same under the Wow6432node.
 7. do the same by reviewing the apps found under the Software hive and NTUser.dat\software.
 8. LNK and Prefetch files that points to the applications launched to access the files.

 Regards,

 Lakshmi N
 Lakshmi,

 Great stuff, thanks for sharing. It doesn't look like you need me to write these things up...you've got a handle on it. Now, you just need to share it.

 Just FYI...#4 and #6 are redundant.
 Post a Comment
 Pages
 Home
 Timelines
 Books
 Malware
 FOSS Tools
 Subscribe To WindowsIR
 WindowsIR Blog List
 Open Source DFIR Plaso 20260119 released 4 days ago
 Brett Shavers AI Won’t Replace DFIR Investigators. But It Will Replace Those Who Don’t
Investigate. 2 weeks ago
 The Philosophy of DFIR The Case Against Limited-Scope Warrants for Digital Evidence 1 month ago
 dfirtnt.wordpress.com Introducing Huntable CTI Studio 2 months ago
 c-APT-ure Using NetBIOS names for pivoting and threat clustering 6 months ago
 CyberDefNerd Xworm – Static Analysis (part 3) 8 months ago
 inversecos An inside look at NSA (Equation Group) TTPs from China’s lense 1 year ago
 ForensicITGuy
 Find Evil
 Blog Archive
 ► 2026 (8) ► March (2)
 ► February (1)
 ► January (5)

 ► 2025 (27) ► December (3)
 ► November (8)
 ► October (2)
 ► September (1)
 ► July (1)
 ► June (4)
 ► May (1)
 ► March (3)
 ► February (2)
 ► January (2)

 ► 2024 (22) ► December (1)
 ► November (1)
 ► October (7)
 ► July (1)
 ► June (1)
 ► March (4)
 ► February (2)
 ► January (5)

 ► 2023 (50) ► December (3)
 ► November (2)
 ► October (1)
 ► September (2)
 ► August (7)
 ► July (6)
 ► June (6)
 ► May (4)
 ► April (7)
 ► March (4)
 ► February (6)
 ► January (2)

 ► 2022 (51) ► December (3)
 ► November (4)
 ► October (6)
 ► September (5)
 ► August (5)
 ► July (9)
 ► May (5)
 ► April (5)
 ► March (4)
 ► February (2)
 ► January (3)

 ► 2021 (26) ► December (3)
 ► November (3)
 ► October (3)
 ► September (5)
 ► August (2)
 ► June (4)
 ► April (4)
 ► March (1)
 ► January (1)

 ► 2020 (26) ► November (2)
 ► October (3)
 ► September (1)
 ► August (3)
 ► July (1)
 ► June (2)
 ► May (2)
 ► April (3)
 ► March (2)
 ► February (4)
 ► January (3)

 ► 2019 (43) ► December (5)
 ► November (2)
 ► October (2)
 ► September (3)
 ► August (4)
 ► July (1)
 ► June (1)
 ► May (9)
 ► April (4)
 ► March (2)
 ► February (5)
 ► January (5)

 ► 2018 (49) ► December (4)
 ► November (4)
 ► October (4)
 ► September (7)
 ► August (6)
 ► July (1)
 ► June (4)
 ► May (2)
 ► April (2)
 ► March (7)
 ► February (5)
 ► January (3)

 ► 2017 (25) ► December (2)
 ► October (3)
 ► September (4)
 ► August (3)
 ► July (1)
 ► June (1)
 ► May (1)
 ► April (3)
 ► March (2)
 ► February (2)
 ► January (3)

 ► 2016 (43) ► December (1)
 ► November (1)
 ► October (3)
 ► September (5)
 ► August (3)
 ► July (2)
 ► June (5)
 ► May (5)
 ► April (4)
 ► March (3)
 ► February (5)
 ► January (6)

 ► 2015 (34) ► December (6)
 ► November (1)
 ► October (3)
 ► September (3)
 ► August (2)
 ► July (2)
 ► June (4)
 ► May (3)
 ► April (4)
 ► March (3)
 ► February (1)
 ► January (2)

 ► 2014 (33) ► December (3)
 ► October (5)
 ► September (2)
 ► August (1)
 ► July (4)
 ► June (1)
 ► May (5)
 ► April (5)
 ► March (4)
 ► February (1)
 ► January (2)

 ▼ 2013 (64) ► December (4)
 ► November (3)
 ► October (2)
 ► September (5)
 ▼ July (14) HowTo: Investigate an Online Banking Fraud Incident
 HowTo: Determine/Detect the use of Anti-Forensics ...
 HowTo: Add Intelligence to Analysis Processes
 HowTos
 HowTo: Data Exfiltration
 HowTo: Detecting Persistence Mechanisms
 HowTo: Malware Detection, pt I
 Programming and DFIR
 HowTo: Track Lateral Movement
 HowTo: Determine User Access To Files
 HowTo: Determine Program Execution
 HowTo: Correlate Files To An Application
 HowTo: Determine Users on the System
 HowTo: Correlate an Attached Device to a User

 ► June (5)
 ► May (4)
 ► April (9)
 ► March (5)
 ► February (5)
 ► January (8)

 ► 2012 (73) ► December (3)
 ► November (4)
 ► October (5)
 ► September (4)
 ► August (3)
 ► July (4)
 ► June (8)
 ► May (11)
 ► April (8)
 ► March (7)
 ► February (7)
 ► January (9)

 ► 2011 (109) ► December (9)
 ► November (9)
 ► October (10)
 ► September (15)
 ► August (11)
 ► July (8)
 ► June (10)
 ► May (4)
 ► April (11)
 ► March (9)
 ► February (6)
 ► January (7)

 ► 2010 (90) ► December (12)
 ► November (5)
 ► October (3)
 ► September (2)
 ► August (3)
 ► July (10)
 ► June (9)
 ► May (5)
 ► April (8)
 ► March (10)
 ► February (15)
 ► January (8)

 ► 2009 (166) ► December (15)
 ► November (14)
 ► October (10)
 ► September (9)
 ► August (13)
 ► July (12)
 ► June (13)
 ► May (12)
 ► April (19)
 ► March (22)
 ► February (15)
 ► January (12)

 ► 2008 (108) ► December (9)
 ► November (6)
 ► October (12)
 ► September (9)
 ► August (17)
 ► July (11)
 ► June (9)
 ► May (4)
 ► April (11)
 ► March (4)
 ► February (8)
 ► January (8)

 ► 2007 (83) ► December (6)
 ► November (7)
 ► October (1)
 ► September (3)
 ► August (4)
 ► July (8)
 ► June (10)
 ► May (12)
 ► April (7)
 ► March (11)
 ► February (3)
 ► January (11)

 ► 2006 (118) ► December (1)
 ► November (16)
 ► October (18)
 ► September (15)
 ► August (17)
 ► July (7)
 ► June (8)
 ► May (4)
 ► April (12)
 ► March (3)
 ► February (9)
 ► January (8)

 ► 2005 (163) ► December (5)
 ► November (1)
 ► October (10)
 ► September (21)
 ► August (22)
 ► July (12)
 ► June (15)
 ► May (4)
 ► April (14)
 ► March (21)
 ► February (20)
 ► January (18)

 ► 2004 (16) ► December (16)
