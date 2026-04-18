# Windows Incident Response

- URL: https://windowsir.blogspot.com/2007/07/acpo-guidelines.html
- Published: unknown
- Updated: unknown
- Labels: none

Windows Incident Response
 The Windows Incident Response Blog is dedicated to the myriad information surrounding and inherent to the topics of IR and digital analysis of Windows systems. This blog provides information in support of my books; "Windows Forensic Analysis" (1st thru 4th editions), "Windows Registry Forensics",
as well as the book I co-authored with Cory Altheide, "Digital Forensics with Open Source Tools".
 Wednesday, July 11, 2007
 ACPO Guidelines

 8 comments:
 I've missed your reports! It's pretty much a "given" today that live acquisition is acceptable in a number of circumsance that you know better than I. It seems that the guidelines are dated in some respects, and a qualified examiner should be able to justify a live exam done with tested tools in a manner such as described in your book.

 Consider a tool like X-Ways Capture. It acuires data (live) in an acceptable order. It produces a log of its steps, it's configurable, and leaves a small footprint. The size of one's foot, however, varies from system to system. You can run a few tests and determine the data written to storage devices, e.g., the prefetch file you described. I'm not sure whether you can measure the footprint in RAM with any reliability.

 Playing devil's advocate, perhaps your oppenent will claim that you overwrote key evidence when you consumed unallocated space by writing a prefetch file. Maybe the latest version of a key document was in memory, but overwritten by your acquisition tool. What else? I'm looking for the downside issues, as they may lead to better preparation.

 BTW, I've been told that KNTdd will in fact acquire Vista RAM, at least with the Home editions. There's a discount to LE. There's an interesting thread on the Digital Detective board, where Craig also alerted members to your book. :-)
 I'm not sure whether you can measure the footprint in RAM with any reliability.

 I agree. Further, many tools will complete their task and exit, which means that very quickly, the process will be created, memory allocated and used, and then that memory will be released for use by other processes.

 Not only will the number of memory pages consumed vary based on software load (what's installed and running) and the OS version, but it will be difficult enough to identify on a test system...let alone on the real-world systems we respond to.

 Maybe the latest version of a key document was in memory...

 Perhaps, but when a process is loaded into memory, pages that are actively used are not overwritten...if that were the case, then we'd have all sorts of issues and system failures when someone loaded Solitaire. This means that whatever is available in memory for use will not be associated with an active, running process or thread of execution. The latest version of a key document may be in memory, but if the process that was being used to create it (ie, Excel, Word, Notepad) is still running, the memory pages won't be overwritten...at the worst, they'll be written out to the pagefile.

 ..I've been told that KNTdd will in fact acquire Vista RAM...

 I was in the beta test program, so I can say that yes, kntdd will allow you to acquire RAM from Windows 2003 SP1 and Vista systems. I had to purchase my copy of the tool, and as I've recently installed Vista into a VMWare session, I'll be able to test this.

 ...Craig also alerted members to your book...

 Craig who? I'll have to thank him...
 Craig who? I'll have to thank him...

 Craig Wilson, the list admin and author of NetAnalysis. Craig has just written a beta RAM imager.
 Jimmy...is this beta RAM imager available for preview?
 Yes. Craig has asked for interested persons to email and request a copy of MemGrab. I'll send his address to you off-blog.

 Oh, I forgot to mention that the thread on the DD forum discusses some very interesting findings on acquiring artifacts from memory after the machine has been shut down.
 Jimmy,

 Can you tell me which thread this is? I'm a member of the board, and I'd like to review this...it's great that you point it out, but it would be immensely more helpful if you could tell me *where* it is, rather than just that it is. ;-) Thanks.
 Sorry, Harlan. I didn't know you were on the board, though it was obvious that you'd be a valuable resource. (I still wish there was way to get an email when you/I/anyone comments on a followed blog post. I'd be better able to post back quickly.)

 The link is http://www.digital-detective.co.uk/cgi-bin/digitalboard/YaBB.pl?num=1184139817;start=all
 "Recovery of data from RAM"
 Thanks, I found it...while it is interesting, I do see a lot of the same questions/concerns/misunderstandings with RAM acquisition that I see in other forums...and the same folks addressing and attempting to answer them! ;-)
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

 ▼ 2007 (83) ► December (6)
 ► November (7)
 ► October (1)
 ► September (3)
 ► August (4)
 ▼ July (8) Book Review
 Thoughts on RAM acquisition
 Tool Testing Methodology, Memory
 Tool Testing Methodology
 Are you Security Minded?
 Updates, etc.
 ACPO Guidelines
 Windows Forensic Analysis Book Review Posted

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
