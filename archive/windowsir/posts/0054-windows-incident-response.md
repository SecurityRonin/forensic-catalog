# Windows Incident Response

- URL: https://windowsir.blogspot.com/2009/11/in-news.html
- Published: unknown
- Updated: unknown
- Labels: none

Windows Incident Response
 The Windows Incident Response Blog is dedicated to the myriad information surrounding and inherent to the topics of IR and digital analysis of Windows systems. This blog provides information in support of my books; "Windows Forensic Analysis" (1st thru 4th editions), "Windows Registry Forensics",
as well as the book I co-authored with Cory Altheide, "Digital Forensics with Open Source Tools".
 Wednesday, November 11, 2009
 In The News

 And that may be another reason why black hats are flocking to the cloud.

 10 comments:
 The locating of issues is easy enough, the actual grading e.g. categorisation of pornographic material from pictures of trees/cars/houses etc is a very time intensive process, one which unfortunately takes far too much of an investigators time, especially when you consider the volume of cases involving indecent pornography.
 Understood...but on a system with, say, 20,000 images, how many does one actually have to categorize? Would it be enough to meet the federal statute? Or how about just 100? Would the time be better spent figuring out whether or not a Trojan really did it, or tying the images to a specific user account?
 Well for one "customer" (in the UK) it is 10000 images, so it takes a while.
 I have also seen where HTML email from the desktop client ends up in the TIF directory. That really confused me the first time I saw it.
 JM...interesting. Can you elaborate?
 JM...interesting. Can you elaborate?
 Using IEHV from NirSoft, I was reviewing the history files and found *a lot* of local file browsing in the user temp folders, the contents of which were all emails (and the temp folder resembled the name of the email client itself "xpgrpwise" along with the internal email domain and post office of the user).

 I confess that I committed a cardinal sin and didn't really pursue it much further, simply assuming, er "concluding" that the local client was using the "IE engine" to render HTML email. (In my defense, I was really just testing the iehv tool and had no investigation that would require verification at the time. Still - I should've run it to ground.)

 Now it seems more logical that the client was probably just using the WinInet APIs. And this time I will at least attempt to verify that. :-)
 Bingo. Using procexp, I see the wininet.dll file loaded by the email client, and see the system calls via procmon.

 Another successful day of learning. :-)
 Understood...but on a system with, say, 20,000 images, how many does one actually have to categorize?

 20,000 images are nothing! I probably can view that many thumbs and categorize them properly in about an hour. I've had a case with one million, but that's another story :-). It's really not a matter of "how many are enough?" Regardless of the fact that one or five can be an offense, the idea, after all, is to protect children. We send all c-p to NCMEC for the database and stopping short, in any but an extreme case, is unacceptable. We can, however, provide the images to the case agent or an analyst for review.

 The EID feature is an add-on that is not free unless you buy an extended subscription. I find that disappointing in an expensive tool. A similar feature comes with XWF. That said, I rarely use it; I can't take a chance of missing anything, and these tools are imperfect.

 First, those TIF subfolders aren't created by IE, they're created by the use of the WinInet APIs, which IE uses.

 You got me here, so please forgive my ignorance of the subject. Would it follow, then, that it's almost always the API or the function of some library or the like that does something and not the app? Of course, absent any indication of infection, it's a matter of what's in the folders and how it got there. Perhaps too many cases are founded on evidence that lends itself to the trojan defense.
 Jimmy,

 Would it follow, then, that it's almost always the API or the function of some library or the like that does something and not the app?

 No, that's not a blanket statement you can use. It's apparently the case...but only in this cases, as far as I know.

 For instance, when a PE file is launched, if you "watch" it with ProcMon, you'll see the Image File Execution Options Registry key checked. This isn't a function of the PE file itself...this is a function of how the OS manages the launching of the PE file. However, with MRU lists, those are, in most cases, a function of the app...how/if they're written, how they're maintained, etc.

 Perhaps too many cases are founded on evidence that lends itself to the trojan defense.

 I think that the first instances of the use of the defense were more of "..this stuff is so technical, they'll never prove otherwise.."; now, it might be more of a gamble on how good the examiners for both sides are.

 The interesting thing about your statement, though, is that there is evidence that may lend itself very well to the Trojan Defense...but at the same time, there's other information available that can lend a greater level of context and granularity to that evidence.
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

 ▼ 2009 (166) ► December (15)
 ▼ November (14) Incident Preparation
 More Timeline Creation Techniques
 Even More Linky Goodness...
 Working with Volume Shadow Copies
 It's about time...
 Some Analysis Coolness
 In The News
 Happy Birthday, VMI!
 Happy Birthday, Marines!
 p0wnage
 More Linky Goodness, plus
 Link-alicious
 The Future of RegRipper
 Into The Boxes

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
