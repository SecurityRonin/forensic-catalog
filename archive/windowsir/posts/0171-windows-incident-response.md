# Windows Incident Response

- URL: https://windowsir.blogspot.com/2020/10/settings-that-impact-windows-os.html
- Published: unknown
- Updated: unknown
- Labels: none

Windows Incident Response
 The Windows Incident Response Blog is dedicated to the myriad information surrounding and inherent to the topics of IR and digital analysis of Windows systems. This blog provides information in support of my books; "Windows Forensic Analysis" (1st thru 4th editions), "Windows Registry Forensics",
as well as the book I co-authored with Cory Altheide, "Digital Forensics with Open Source Tools".
 Sunday, October 11, 2020
 Settings That Impact The Windows OS
 There are a number of settings within Windows systems that can and do significantly impact the functionality of Windows, and as a result, can also impact what is available to a #DFIR analyst.  These settings very often manifest as modifications to Registry keys or values. These settings also make excellent targets for threat hunting, as well.
 Application Prefetching
 Most DFIR analysts are aware of application prefetching and what it means.  In short, when application prefetching is enabled, files with *.pf extensions are created in the C:\Windows\Prefetch folder.  These files are intended to provide quicker loading of frequently-used applications, by placing needed information in a specific, known location.
 Most analysts are also aware that application prefetching is not enabled by default on server versions of Windows.  As such, Prefetch files are expected on Windows 7 and 10 systems, but not on Windows Server 2016 and 2019 systems.
 As Dr. Ali Hadi pointed out , not all application prefetch files appear directly beneath the Prefetch folder.
 Plaintext Passwords
 We've seen the UseLogonCredential value being used during credential access for some time now, as creating this value and setting it to "1" tells the operating system to maintain credentials in memory in plain text.  As a result, threat actors have been observed creating this value (via reg.exe ), setting it to "1", and then returning to the system 6 - 14 days later (depending upon the time frame, campaign, threat actor group, etc.) and using freely available tools such as mimikatz to dump credentials.  In a number of cases, lateral movement has followed very shortly thereafter, as the credentials are available.
 If you perform a Google search for the value name, you'll find more than a few articles that mention setting the value to "0" in order to disable the functionality; however, this will have little effect if the threat actor is able to access systems with the appropriate level of privileges to set the value to "1".  However, this does provide an excellent resource for proactive, reactive, and DFIR threat hunting.  This is easy to set up and automate, and you'll only need to react when the value is found, and when it's set to "1".  That is, however, if the value doesn't exist within your infrastructure at all; if it does, and you find it set to "0", you then have good reason to investigate further.
 Disabling Security Event Logging
 About a year ago, this Sec-Labs blog post described a Registry key that, when added to a system, disabled Security Event Logging.  More recently, this tweet reiterated the same thing, referring to the blog post; I tested this on a Windows Server 2016 VM and found that it worked exactly as described in the Sec-Labs blog post; the EventViewer wasn't functioning properly and after extracting Windows Event Logs from the VM image file, I found the Security Event Log was not being populated.  After adding the value to the Registry, I had rebooted the system several times, which should have caused logon events to be written to the log file; however, upon examination of the Security.evtx file, this was not the case.
 This is markedly different from clearing the Security Event Log.  If a Windows Event Log is cleared, some, if not all, of the records may be recovered (I say "may" because it depends upon how soon you're able to respond).  However, adding the "MiniNt" key to the Registry causes events to not be written to the Security Event Log, and as a result, there's nothing to "recover".  Nothing is written, neither to the log nor to unallocated space.
 I know...I was thinking the same thing when I read the original blog post on the topic, and thought it again when I saw that it worked.
 Conclusion
 There are other Registry keys and values that can significantly impact the performance and operation of the Windows operating system; the three listed here are by far not the only ones.  Rather, they are just examples, and serve to demonstrate what I meant by "significantly impact". These keys and values can also be added to a proactive, reactive, or DFIR threat hunting process.
 2 comments:
 From Lorie Hermesdorf, by way of LinkedIn...

 https://www.quppa.net/blog/2016/04/14/beware-of-the-minint-registry-key/
 Here's another one, to load a DLL via Office:

 https://twitter.com/william_knows/status/897616872673292288
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

 ▼ 2020 (26) ► November (2)
 ▼ October (3) Name Resolution
 Settings That Impact The Windows OS
 #OSDFCON

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
