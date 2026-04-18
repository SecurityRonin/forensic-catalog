# Windows Incident Response

- URL: https://windowsir.blogspot.com/2007/11/alternative-methods-of-analysis.html
- Published: unknown
- Updated: unknown
- Labels: none

Windows Incident Response
 The Windows Incident Response Blog is dedicated to the myriad information surrounding and inherent to the topics of IR and digital analysis of Windows systems. This blog provides information in support of my books; "Windows Forensic Analysis" (1st thru 4th editions), "Windows Registry Forensics",
as well as the book I co-authored with Cory Altheide, "Digital Forensics with Open Source Tools".
 Wednesday, November 21, 2007
 Alternative Methods of Analysis

 8 comments:
 I agree that live acquisition can be essential in some cases, but it's not going to happen in the vast majority of LE investigations. The philosophy envisions a trained examiner being on the scene of every search in which a computer may be running.

 The hypothetical Q&A in the article simply brings home what happens during every moderately astute cross examination. It's exaggerated. You can't prove a negative conclusively in most instances. You and I could easily construct some similar Q&As to attack an examiner who did a proper live acquisition:

 Defense Attorney: So, when you ran your live acquisition tool, you overwrote some memory that contained another person's confession.

 The article has merit, but is idealistic in today's world. Tomorrow, it may be realistic.

 Using best practices best defends the trojan defense. Still, what do you say when asked whether a root kit might have existed at one time? All the "possibility" questions may not achieve their advocate's intent if he or she doesn't present some proof that Bug X existed. I've been there, and a competent trial attorney will recap the case and emphasize testimony and fact over speculation. You may recall a recent post on a well respected forum that failed to produce any findings of any malware ever having downloaded child pornography to a system. The problem, however, is what would you say, if I asked you whether it is possible?

 As you said, "greater knowledge" is the key. Your blog, help, and writings advance that goal.
 Jimmy,

 Thanks. I agree that collecting volatile data isn't always possible or practical...even in the non-LEO world. There are many times when the system has been taken offline, scanned, rebooted (several times), etc. Live acquisitions turn out to be more common for me, it seems...the system cannot be taken down, there is no write-blocker for the hard drives, etc.

 I've always had an issue with arguments that included "a competent defense attorney would tear that apart"...largely b/c a competent prosecutor would ensure that the case did not hinge on a single piece of "evidence".

 Your blog, help, and writings advance that goal.

 Any thoughts on how to improve any of those would be greatly appreciated.
 Any thoughts on how to improve any of those would be greatly appreciated.

 I think that a paper or guide on event logs would help many of us, who may not routinely review this data. Something like your registry spreadsheet comes to mind, if there can be a straightforward reference. Vista, of course, expands the value of these logs, so they may be more important than before.
 Jimmy,

 I think that a paper or guide on event logs would help many of us...

 What suggestions would you have toward modifying or updating the material in my book?
 I haven't had a chance to check out the web sites you cited as references on event IDs, but that's largely what I had in mind. (I didn't want to ignore your question.) A reference on activities/events that are logged may be more important. That may be overly broad, but I'd guess that many folks are unaware of the value of logs because they don't know what can be found. Event logs may not be essential to many cases that LE typically work, but perhaps their value is under estimated. Most training programs only touch upon logs superficially.
 A reference on activities/events that are logged may be more important. That may be overly broad, but I'd guess that many folks are unaware of the value of logs because they don't know what can be found.

 It all comes back to knowledge, and a willingness to develop and learn.

 Event logs may not be essential to many cases that LE typically work, but perhaps their value is under estimated (sic).

 I think that in many cases, the Event Logs are simply not considered as a source of valuable information, and b/c they aren't examined or analyzed regularly, most likely consider it too time consuming an endeavor anyway.

 Most training programs only touch upon logs superficially.

 That's unfortunate. I did some work not to long ago, and the only source of data to answer a question was the Event Logs.

 Event Log analysis goes, at least in part, hand-in-hand with Registry analysis. The Security Registry hive file will tell you what the audit policy was on the system, as well as when it was modified. The System hive file will give you info regarding the size and retention of the Event Log files.

 Because Event Log analysis isn't being performed regularly now, it is likely that the potential value of the Vista Event Logs will be lost entirely.
 I agree. However, many of us are willing to learn, but depend on others to provide resources. "Log File Forensics." A book? A four-hour course? An entire program could revolve around Vista log analysis.

 The logs can be a valuable source of information, but I don't think that, most likely consider it too time consuming an endeavor . I do think that many don't realize the value of the logs, because, in part, there hasn't been a effort to develop some training in this aspect of forensics. Those of us who focus on image analysis of single machines may need some examples of what the logs can provide.

 I have checked out EventID.net and found it very helpful. At least there's a source to explain the events in a friendly format. Recognizing which events warrant study is the key.
 However, many of us are willing to learn, but depend on others to provide resources.

 I can completely understand that, Jimmy, but I have to admit, I don't see a great deal of questions along these lines. I monitor a couple of lists and forums, and accept direct emails, and I don't see much in the way of questions about Event Logs. In fact, I haven't seen any.

 ...there hasn't been a effort to develop some training in this aspect of forensics.

 Puh-lease don't say that it's a case of "someone didn't tell us this was important"! ;-)

 But seriously...who would sign up for the training. One of the biggest issues I've seen is that while I've got the material and would be more than happy to provide the training, it is extremely difficult to break even, revenue-wise.

 However, I do see the direction you're going. The biggest issue is that for someone to provide that kind of material...descriptions of what the Event Log (Win2K3 and Vista) can provide...it's going to require some considerable resources.
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
 ▼ November (7) Alternative Methods of Analysis
 Windows Memory Analysis
 Jesse's back!
 Upcoming Speaking Engagement
 Pimp my...Registry analysis
 Pimp my...live acquisition
 Pimp my...forensics analysis

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
