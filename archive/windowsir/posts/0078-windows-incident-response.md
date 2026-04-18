# Windows Incident Response

- URL: https://windowsir.blogspot.com/2011/10/forensic-scanner.html
- Published: unknown
- Updated: unknown
- Labels: none

Windows Incident Response
 The Windows Incident Response Blog is dedicated to the myriad information surrounding and inherent to the topics of IR and digital analysis of Windows systems. This blog provides information in support of my books; "Windows Forensic Analysis" (1st thru 4th editions), "Windows Registry Forensics",
as well as the book I co-authored with Cory Altheide, "Digital Forensics with Open Source Tools".
 Wednesday, October 05, 2011
 Forensic Scanner

 11 comments:
 Sounds like MIR and EnCase Enterprise?
 How so? I don't have access to, nor have I seen, either one, so I can't speak to them...
 I've seen you talk about the forensic scanner in the past and I think the tool would be useful. Similar to vulnerability scanners, I think one item that would make the forensic scanner really powerful is having the ability to create scanning profiles.

 For example, let's say you need to know what files a user accessed recently. A scanning profile could be configured to run specific plugins across the system such as registry keys with document MRU (Comdiag, Office, etc), Windows Shortcut Files, Jump lists, etc... In a matter of seconds the information could be extracted and presented to you in a report for analysis.

 Personally, this ability would save me a lot of time since I could setup different types of scanning profiles in advanced. When I'm faced with needing to know certain information then I could just run my profile of choice. This is something I can't currently do with my tools without scripting. The project sounds promising.
 Corey,

 Exactly. Remember, this is based on RegRipper...since you can do this with RegRipper, you'll be able to do the same with the scanner.
 This sounds like a more powerful version of what some SCAP/S-CAP scanners are capable of.
 Clint,

 I have no idea what that is...
 Sounds like MIR and EnCase Enterprise?

 I've thought about this comparison, and I don't see that this is the case at all. I was also hesitant to respond to the comment, as I doubt that we'd hear back from "Anonymous".

 It appears that rather than reading and correlating all of the description of the scanner, some simply choose to key off on one or two attributes, and draw conclusions. That's okay, as I'm sure it's to be expected. This scanner isn't like either MIR or EE, in part simply due to the fact that it doesn't us a proprietary API...if you can program in Perl, you can create a plugin.

 I also think that the comparison to MIR/EE originates with the mention of F-Response, which indicates that the "Anonymous" poster knows very little about any of the products.

 The scanner is intended to be used with acquired images; the fact that it can be used along with F-Response is not the primary means for using it, but is simply one of the ways it can be used. Also, when you consider the size of the deployed executables when compared to the F-Response agent, what they do/provide, and their cost, the correlation breaks down.

 In short, saying that this is just a variant of EE or MIR is very short sighted.
 Sounds like MIR and EnCase Enterprise?

 Hmmm... I use EE almost every single day. I don't see how this is "like" EE at all. What I do see is a nice open source tool that can, in the right hands, greatly enhance the capabilities of EE and similar tools. With that said, GSI does have a Cybersecurity framework that sits on top of EE that encompasses this type of functionality in its own way. (Plus a whole lot more - of course.)

 Personally, I can't wait to see this tool. I am already a HUGE fan of RegRipper for all the reasons Harlan stated above. If this tool follows the same methodology, then I imagine I'll be huge fan of it as well.

 [Mandatory Disclosure] Even though I work for GSI, these comments are my own and are not to be construed in any way as those of GSI.
 Andy,

 Thanks for the comment. Anonymous is usually short-sighted, and then doesn't come back to provide any elaboration.
 SCAP = http://en.wikipedia.org/wiki/Security_Content_Automation_Protocol

 It is pretty flexible even though most people only use it for specific things. Most people use it to do configuration checks to make sure a system conforms to particular security settings. Nessus, Retina, McAfee ePO can run SCAP scans. There are many things you can do with it such as you can write SCAP sigs to find file system artifacts for malware. You can't look for memory artifacts with it though.
 Clint,

 Can you use this tool to get Registry key LastWrite times and/or values, and correlate those with file system artifacts? How about arbitrary file system (or file) metadata?
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

 ▼ 2011 (109) ► December (9)
 ► November (9)
 ▼ October (10) Stuff
 NoVA Forensics Meetup
 Tools and Links
 Stuff in the Media
 Links, Updates, and WhatNot
 Links
 NoVA Forensic Meetup
 Forensic Scanner
 Documentation
 WFA 3/e update

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
