# Windows Incident Response

- URL: https://windowsir.blogspot.com/2013/09/data-structures-revisited.html
- Published: unknown
- Updated: unknown
- Labels: none

Windows Incident Response
 The Windows Incident Response Blog is dedicated to the myriad information surrounding and inherent to the topics of IR and digital analysis of Windows systems. This blog provides information in support of my books; "Windows Forensic Analysis" (1st thru 4th editions), "Windows Registry Forensics",
as well as the book I co-authored with Cory Altheide, "Digital Forensics with Open Source Tools".
 Monday, September 02, 2013
 Data Structures, Revisited

 4 comments:
 "do we understand enough about the file's underlying data structures to understand the true nature and context of the data?”

 Another question might be: With all the data structures out there, is it even possible to truly understand them all?

 Furthermore, how do you determine if you understand it? Years ago, most would say you truly understood the Registry, then regslack was discovered. We have little hope of not missing evidence from data structures when you miss evidence in the Registry. ;)

 Computers are complex, and if history is any indication of the future, we’re still likely missing a lot of important evidence. Look at timeline and memory analysis improvements over the years.

 You've given great examples as to why it's important to understand data structures, but what about the times you, and everyone else, miss those critical artifacts-- and never know about it?
 Another question might be: With all the data structures out there, is it even possible to truly understand them all?

 Good question...without the vendor's assistance, no, I don't believe that this an absolute.

 However, the fact is that many data structures are, in fact, documented...at least to some degree. Again, Joachim Metz and others have done a great job with this. Even when the structures are documented, there's still a "so what" factor...analysts may not be able to process the information in a manner in which they can incorporate it into their day-to-day work. The first step to this is awareness...if you don't know that something is out there, and why it's important and valuable to _you_, there's really no point.

 No one can know everything...this is why we have references. However, references are no good if they aren't used. The ForensicsWiki is a great site for keeping and maintaining this sort of information; however, I don't think that analysts use it. This could be because they can't find what they're looking for, so instead of asking for assistance, they simply stop using the site.

 The fact is that in many cases, the information is out there. If you're aware of it, but can't follow it or don't understand how to use it, and you've read what's available, ask. I've met analysts who've spent weeks and in some cases months "noodling something over", whereas if they'd asked someone, they would've received a pretty complete education in 20 minutes.

 Can we possibly know every data structure? No. Can we better understand the ones that are known? Yes. Can we ask for assistance and direct attention to data structures that were not previously known? Of course.

 ...but what about the times you, and everyone else, miss those critical artifacts-- and never know about it?

 Exactly. If the "critical artifacts" are unknown, how do you know that you missed them?
 "Exactly. If the "critical artifacts" are unknown, how do you know that you missed them?"

 I guess we don’t. If security professionals can only mitigate risk, maybe forensic analysts can only mitigate doubt. No security professional should say they’re 100% secure. No forensic analyst should say they’re 100% certain, especially when dealing with negative evidence. It’s up to the analyst to understand the evidence, tools, and limits as best as possible.

 http://rationalwiki.org/wiki/Negative_evidence

 Learning to identify and interpret the "critical artifacts" is one of those things that turns the novice into a master and separates the hack from the expert.

 There are those who constantly dig into platforms and applications, who run tools to document system and program behavior, and who, when they see unexpected results in their work, chase down the reasons why. And there are those who don't.

 To succeed in digital forensics, you need to accept a level of uncertainty, all the while working to reduce that uncertainty. You can't know everything, but you certainly can work to know more than you now know. Digital forensics is pretty much like any other profession in that regard.

 If you approach forensics as mostly a collection of tips and tricks to be mastered, then you will probably fail at some point or work at a lower level of proficiency. If you view forensics as an evolving, dynamic subject matter that has to be continually examined and mastered, you will be better served. Good forensics takes attention and a work.

 Good forensics involves learning the fundamentals of the platform one will be examining. If you examine Windows systems, then you should absorb Windows Internals (the books), and be intimately familiar with TechNet, MSDN, etc.. Absorb what the professional community already knows, by reading blogs such as this, and reading the best of forensics books.

 Constantly challenge what you think you know. As you study forensics, train, go about your work, or look at the forensic import of a new OS or program, consider whether whatever you are focused on is "stateful," i.e., whether it remembers prior states, or reflects current states of a feature or program. Stateful suggests potential artifacts. Using techniques of behavior analysis (e.g., process monitor, registry differencing, etc.) identify all the files and places (registry keys, databases, log files, scratch files, etc.) that the feature or program touches--especially writes. Those are your artifacts.

 Interpreting artifacts requires 1) being able to "read" an artifact and then 2) being able to determine its significance. In the Windows world, there are a finite number of file formats. In fact, Windows relies heavily on a handful of file types and data structures--file headers. Learn to identify these on sight. When you recognize a file type or data structure, you can apply the correct tool or algorithm to make sense of it. Thanks to an active, hard working community, you can find multiple parsers for almost all Microsoft file formats.

 Determining the significance of data in an artifact will require additional thought and testing. The question you want to answer is, "what does the artifact prove?" You also have to consider what the artifact does not prove. It is as important to understand the limits of the evidence offered by an artifact as it is to understand what it may prove. (For example, pictures in thumbcache files don't mean that those pictures where ever opened by a user.)

 Then, rinse and repeat.

 T
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
 ▼ September (5) Links - Malware Edition
 Shell Item Artifacts
 Forensic Perspective
 Links
 Data Structures, Revisited

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
