# Windows Incident Response

- URL: https://windowsir.blogspot.com/2009/12/df-and-disclosure.html
- Published: unknown
- Updated: unknown
- Labels: none

Windows Incident Response
 The Windows Incident Response Blog is dedicated to the myriad information surrounding and inherent to the topics of IR and digital analysis of Windows systems. This blog provides information in support of my books; "Windows Forensic Analysis" (1st thru 4th editions), "Windows Registry Forensics",
as well as the book I co-authored with Cory Altheide, "Digital Forensics with Open Source Tools".
 Saturday, December 19, 2009
 DF and Disclosure

 6 comments:
 DECAF has just been 100% advertisement for its authors and the community is helping them adding to the adverstisement.
 Their ethics should be questioned indeed.
 They not only used and reversed engineered a LE tool enfringing copyright, but they also by provided a way for bad guys to hide from it.

 By the way COFEE had been leaked more than a year before all the fuzz about it on rapidshare and other similar sites.
 Anonymous,

 Thanks for weighing in.

 Just to be clear, though...as expressed, these are your thoughts and comments, and while I share the sentiment, this isn't necessarily how I would have conveyed that sentiment.

 What I mean is that I don't want folks who read this to construe what's been said as having been said by me. I wouldn't make a comment like Their ethics should be questioned without having signed my name to the comment.

 Just wanted to be clear. I've received a number of emails over the years I've been blogging from folks who've said, "...you said..." and then quoted an Anonymous comment.

 Thanks.
 @Anonymous,

 Bringing up "reverse engineering" is just going to derail the conversation down another, counter productive, path. Security researchers conduct research into the behavior and capabilities of software and operating systems to determine how they function. This in turn assists the developers strengthen their tools and increase the tool's capabilities. They would call this research and not reversing. Six in one hand....yada, yada, yada.

 Personally I think it is great that these guys worked on this stuff and exposed some of the functionality and capabilities (although I do not agree with the phone-home capabilities if they exist). Perhaps this research will help the developers understand that there are bad guys out there who are already working on these types of detection mechanisms for their tools. We have seen it other places (vm, malware), so why is this shocking here? Mayhap this will help Microsoft start thinking along the lines of mixing up some of their techniques. For instance, randomizing the names of the tools run, running tools in slightly different order, or providing compilation capabilities to mix up the signatures of the tools to avoid detection.

 What this really boils down to is understanding the Order of Volatility. Knowing that if something can change then an analyst should expect it to change. Whether before or "during" information gathering. Heck, if you are writing to a default location then you should also be worried about changes "after" data collection.

 Could the crew of DECAF have handled this better? Yes. But in a day and age when speed of release is seemingly more important than functionality and capabilities I can understand why they jumped the gun before the COFEE issue retreated into the background again. Hopefully this activity has attracted the attention of the COFEE developers and they now have some ammunition they can use to justify continued and more in-depth development. I hope that they can push the case for being able to provide in-depth information into the inner workings of the Windows operating system so that it is easier for law enforcement, digital analysts, and incident responders to gather and analyze the activity that has occurred on a system with more detail and accuracy. Also, these efforts have probably taught the DECAF crew a few lessons. Hopefully they will use this knowledge for positive research and tool releases in the future.

 Go forth and do good things,
 Don C. Weber
 Don,

 COFEE is nothing more than a glorified batch file...which means that rather than finding an issue or vulnerability and fixing it, the DECAF guys took an approach that could potentially affect all such frameworks. Any tool can be fingerprinted...even memory dumping tools.

 Also, keep in mind that there is a pretty big misconception about COFEE...while it has the MS label on it, the last time I had a close look it, COFEE was developed FOR folks who don't really do IR, BY folks who don't really do IR, and the folks who created COFEE (God bless 'em) are not the entirety of MS. Rather, they're a small part of MS.

 How many tools from MS are really for IR? Look at the SysInternals tools and how many of them create a Registry entry the first time you use them on a system...is this really something someone looks for in an IR tool?
 Hey everyone, I respect everyone's comments whether truthful or lack thereof.

 The blog post was very good which is why I decided to take some time and post my two-cents.

 DECAF was not well thought through. We knew we needed to get out the door before someone else did. Our intentions were not malicious and the opportunity for DECAF to phone home was nothing more for version checking and for monitoring usage.

 We NEVER reverse engineered COFEE and although "Anonymous"'s assumption is incorrect, DECAF did monitor for COFEE's presence through other identification.

 Needless to say our WHOLE agenda was to put emphasis on the security seen at large. Whether we did it right or wrong, it worked.

 Anyhow, the COFEE leak was major when it should have been minor. DECAF news road that wave, it wasn't anything great. Its over.

 :)

 - Mike
 DECAF Developer
 Mike,

 Thanks for the comment!
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

 ► 2013 (64) ► December (4)
 ► November (3)
 ► October (2)
 ► September (5)
 ► July (14)
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

 ▼ 2009 (166) ▼ December (15) Lions, and tigers, and DECAF...oh, my!
 Investigating Breaches, pt. II
 Investigating Breaches
 Links and Stuff
 Using RegRipper
 The Trojan Defense
 DF and Disclosure
 When a tool is just a tool, pt II
 When a tool is just a tool, pt I
 Incident Prep, part deux
 Link-idy link-idy
 Some New Stuff
 Plugin Browser - New RegRipper Tool
 Best Practices: What is 'best'?
 Linkaliciousness

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
