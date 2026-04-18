# Windows Incident Response

- URL: https://windowsir.blogspot.com/2022/03/the-misuse-of-artifact-categories-pt-ii.html
- Published: unknown
- Updated: unknown
- Labels: none

Windows Incident Response
 The Windows Incident Response Blog is dedicated to the myriad information surrounding and inherent to the topics of IR and digital analysis of Windows systems. This blog provides information in support of my books; "Windows Forensic Analysis" (1st thru 4th editions), "Windows Registry Forensics",
as well as the book I co-authored with Cory Altheide, "Digital Forensics with Open Source Tools".
 Sunday, March 06, 2022
 The (Mis)Use of Artifact Categories, pt II
 My previous post on this topic presented my thoughts on how the concept of "artifact categories" were being misused.
 My engagement with artifact categories goes back to 2013 , when Corey Harrell implemented his thoughts on categories via auto_rip . I saw, and continue to see, the value in identifying artifact categories, but as I alluded to in my previous post, it really seems that the categories are being misused. Where the artifacts should be viewed as providing an indication of the categories and requiring further analysis (including, but not limited to the population of artifact constellations), instead, the artifacts are often misinterpreted as being emphatic statements of the event or condition occurring. For example, while an entry in the ShimCache or AmCache.hve file should indicate that the file existed on the system at one point and may have been executed, too often either one is simply interpreted as "program execution", and the analyst moves on with no other validation. Without validation, these "findings" lead to incorrect statements or understanding of the incident itself.
 Program Execution
 There was discussion of the "program execution" category in my previous post, along with discussion of the need to validate that the program did, indeed, execute. Often we'll see some indication of a program or process being launched (via EDR telemetry, an entry in the Windows Event Log, etc.) and assume that it completed successfully.
 Keeping that in mind, there are some less-than-obvious artifacts we can look to regarding indications of "program execution"; for example, consider the Microsoft-Windows-Shell-Core\Operational.evtx Event Log file.
 Some notable event IDs of interest (all with event source "Shell-Core"):
 Event ID 9705/9706 - start/finish processing of Run/RunOnce keys
 Event ID 9707/9708 - indicates start and stop of process execution, with corresponding PID.
 Event ID 62408/62409 - start/finish processing of <process>
 Some additional, useful information - from Geoff Chappell's site , we can see that the event ID 9707 records pertain to the "ShellTraceId_Explorer_ExecutingFromRunKey_Info" symbol.
 While the events are restricted to what's processed via the Run and RunOnce keys, we may see the records provide us with full process command lines. For example, and entry for an audio processor included "WavesSvc64.exe -Jack", which appears to be the full command line for the process. This is a bit better information than we tend to see when Process Tracking is enabled in the Security Event Log, leading to event ID 4688 (Process Creation) records being generated; many organizations will enable this setting, but then not also enable the corresponding Registry modification that allows the full command line to be added to the record.
 "Program Execution" is not the only category we should look to go beyond the basic indicators that different resources provide analysts; for example, devices connected to a Windows system, particularly via USB connections, are another category that would benefit from some clarification and exploration.
 USB Devices
 Cory Altheide and I published a paper on tracking USB devices across Windows systems in 2005; at the time, Windows XP was the dominant platform. Obviously, that field has changed pretty dramatically in the ensuing 17 years. In 2015, Jason Shaver's NPS master's thesis on the same topic was published, giving us an updated view of the topic. As you'd expect, Jason's thesis is a pretty thorough treatment of the topic, but for anyone who's been involved in DFIR analysis of Windows systems is likely aware that there have been a number of build updates to Windows 10 that, over time, have changed how the operating system responds to or records various activities.
 In addition, Nicole Ibrahim has done considerable work regarding different types of USB devices and their impact on Windows systems, based on the protocols used. The take-away from Nicole's research is that not all devices are treated the same, just because they're connected via a USB cable. Different devices (i.e., USB thumb drives, smartphones, etc.), while presented via a common interface, often use different underlying protocols, and therefore require different examination routes.
 A note on connecting USB devices to Windows systems...Kevin Ripa recently published an article on that topic where he points out that how you connect the device can make a difference. The key here is to be very aware of the process Kevin went through, as well as his assumptions...both of which he was very clear about. For example, the first device he connected wasn't a USB thumb drive; rather, it was a hard drive connected via a USB cable. Then, he connected the same hard drive via a T-300D Super Speed Toaster , and stated, Now I don’t know about you, but I would have expected a device such as this to simply be a pass-through device . I'd have to say, given that the device is described as a stand-alone disk duplicator, I would not have assumed that it would "simply be a pass-through device" at all. As such, I don't agree that the tools are telling lies, but I do agree that if you're connecting a device to a Windows 10 system, you need to be clear about the device, as well as how it's connected.
 Back to Nicole's research, the take-away is that not all "USB" devices are the same. For example, a USB thumb drive is not the same as a smartphone, even though both are connected via a USB cable. For USB thumb drives and external hard drives, I've found a good bit of evidence in the Microsoft-Windows-StorageSpaces-Driver\Operational.evtx Event Log, specifically in event ID 207, which contains drive model and serial number information.
 For my iPhone, the Microsoft-Windows-WPD-MTPClassDriver\Operational.evtx Event Log contains event ID 1005 records contain messages such as "Customizing for <device>", which points to the smartphone. I've connected my iPhone to my computer upon occasion, to copy pictures over or to use iTunes to update my music playlists on the device. These event ID 1005 records correspond to when I've connected that phone to the system, and will likely show other iPhones I've connected, as well - I don't do this too often, so the log should not be too terribly populated.
 For other useful Windows Event Logs that can provide additional information that may be of use to your investigation, consider checking out Nasreddinne or Adam 's pages, both of which allude to "obscure" or less-commonly-referred-to event logs.
 Take-Away
 What we have to remember from all this is that the artifact categories provide an indication of that category, a sign post that tells the analyst that further examination is necessary. As such, the indicator is the beginning of the examination, not the end. Because a file name appeared in the ShimCache or AmCache, we should not simply jump ahead and assume that the file was executed; rather, we should look for other indications of program execution (Prefetch file, impact of the file execution on the system, etc.), other artifacts in the constellation, before establishing the finding that the program was, in fact, executed.
 No comments:
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

 ▼ 2022 (51) ► December (3)
 ► November (4)
 ► October (6)
 ► September (5)
 ► August (5)
 ► July (9)
 ► May (5)
 ► April (5)
 ▼ March (4) Scheduled Tasks and Batteries
 Windows Event Log Evasion Review
 The (Mis)Use of Artifact Categories, pt II
 DFIR Reporting

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
