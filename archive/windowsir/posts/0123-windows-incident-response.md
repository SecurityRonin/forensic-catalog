# Windows Incident Response

- URL: https://windowsir.blogspot.com/2014/07/book-review-art-of-memory-forensics.html
- Published: unknown
- Updated: unknown
- Labels: none

Windows Incident Response
 The Windows Incident Response Blog is dedicated to the myriad information surrounding and inherent to the topics of IR and digital analysis of Windows systems. This blog provides information in support of my books; "Windows Forensic Analysis" (1st thru 4th editions), "Windows Registry Forensics",
as well as the book I co-authored with Cory Altheide, "Digital Forensics with Open Source Tools".
 Wednesday, July 30, 2014
 Book Review: "The Art of Memory Forensics"

 16 comments:
 Thanks for posting your review. I am going to order my copy now.
 Nice review! I've been not so patiently awaiting my copy of the book and it finally arrived yesterday. I've only had time to read the introduction so far, but I have no doubt this will be a great learning experience.
 Hello! Can you help me please, I am working with volatility and can t find ver. 2.4 just released!
 Thnx
 I had a quick read of the content and my option of the book, wait for the errata or the revised version.

 There are errors in the current version book that should not be made by "THE top minds in the field".

 Joachim,

 Can you elaborate?
 @bosti:

 2.4 can be downloaded from here:

 http://www.volatilityfoundation.org/#!24/c12wa

 and the Github repo is here:

 https://github.com/volatilityfoundation/volatility

 @joachim:

 The errata you found is already posted on the book's page:

 http://www.memoryanalysis.net/#!amf/cmg5

 http://downloads.artofmemoryforensics.com/errata.txt
 E.g. have a look at http://wiki.sleuthkit.org/index.php?title=Mactime_output and the same section in the book explaining the mactime output. Tell me what is wrong in the book and tell me why this mistake is problematic in a book with "forensics" in the title.
 Luckily the authors are quick to put an errata online: http://downloads.artofmemoryforensics.com/errata.txt

 But seeing I did not read it detailed attention, I wonder how much more of these are in there.
 Joachim,

 I'm at a bit of a loss here...how does the errata listed at http://downloads.artofmemoryforensics.com/errata.txt invalidate the entire book?

 You suggested in your first comment that folks wait for the corrected book to come out...I'm a bit unclear as to how just those items listed in the errata.txt file would invalidate the book...
 The error made is, from the perspective of digital forensics, so fundamentally incorrect it does not suit to be in a "forensic" book.

 Questioning the validity of the of the book is not the same as "invalidate the book"; not sure how you derived that. My statement is that one is better of for it to be reviewed a bit more, especially from the forensic point of view.

 What if someone copied this and it ended up in a court report?
 What if this book is used as course material and people are thought incorrect?

 You mentioned before that we as a community should take responsibility for mentoring/educating the next generation of DFIR specialist. Let's make sure we do that correctly, and show that forensics is about validating your findings.
 ...not sure how you derived that.

 Pretty simply, actually. You'd said, "...and my option of the book, wait for the errata or the revised version." Given that you had submitted the errata already (which was already available), the only other "option" anyone could come away with is to invalidate the current edition of the book and wait for the revised edition.

 I think that it is also interesting that you'd say, "What if someone copied this and it ended up in a court report?
 What if this book is used as course material and people are thought incorrect?", and then shortly thereafter say, "...show that forensics is about validating your findings."

 I completely agree with your last statement, but I would think that it would obviate the first two...if an analyst were validating their findings, they wouldn't copy it directly into a court report without validating the information first.

 > if an analyst were validating their findings,
 > they wouldn't copy it directly into a court report without validating the information first.

 So then the question becomes what if a "authoritative source" e.g. a forensics book says you should interpret your findings in a certain way. And you've been thought your findings should be interpreted in the same way. And what if a large part of the forensics community gives their public support to the book. Why not just assume the book is correct, since "THE top minds in the field" are saying so.

 In this case there are sources that contradict the book, and you as an analyst can find out about them. But what if there are none?

 > the only other "option" anyone could come away with is to invalidate
 > the current edition of the book and wait for the revised edition.
 Not sure I'd reason in such absolute terms as "only"; it is quibbling about semantics. I think the point is clear. If you're serious about reading this book wait until it has been scrutinized more.

 Hello,

 I read that on Windows 8 ASLR can be disabled in HKLM\SYSTEM\CurrentControlSet\SessionManager\MemoryManagement with inseration of the new Key MoveImages and the values of 0.

 On Windows 8.1 there is MemoryManagement in Registry Key:

 HKLM\CurrentControlSet\Control\SessionManager\Memory Management.

 Then I read on another site that on Windows 8.1 disabling of ASLR is not possible.

 Do you know if this is so?

 I tried to do the same on Windows 8.1 but it doesn’t work.

 Thanks und many Greetings,

 Thorsten Kaufmann
 Thorsten,

 Do you know if this is so?

 No, I don't. Do you have a link or some reference? Have you brought this up with the good folks over at the Volatility Foundation?
 Hi Harlan,

 my sources are

 http://download.microsoft.com/download/9/9/4/994592CB-C248-464F-93A6-A50E339BE19B/Windows%208%20Security%20-%20ASLR.pdf

 and

 http://digital-forensics.sans.org/blog/2014/02/17/malware-analysis-and-aslr-on-windows-8-1

 I brought this up to the Volatility Foundation two days ago, but didn´t get any answer yet.
 Thorsten,

 Maybe you just need to be patient. I know that several of the Volatility team were in Australia recently, teaching for a week. Teaching can be arduous, if not exhausting...couple that with the flights to and from Australia, and I would bet that they're recovering. I remember going to Singapore a couple of years ago, and spent 15 hrs in one seat.

 Two days ago was Friday here on the East Coast of the US...give the Volatility folks some time get back and get settled and rested.
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

 ▼ 2014 (33) ► December (3)
 ► October (5)
 ► September (2)
 ► August (1)
 ▼ July (4) Book Review: "The Art of Memory Forensics"
 File system ops, testing phase 2
 File system ops, effects on MFT records
 Random Stuff

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
