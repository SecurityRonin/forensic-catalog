# Windows Incident Response

- URL: https://windowsir.blogspot.com/2022/10/post-compilation.html
- Published: unknown
- Updated: unknown
- Labels: none

Windows Incident Response
 The Windows Incident Response Blog is dedicated to the myriad information surrounding and inherent to the topics of IR and digital analysis of Windows systems. This blog provides information in support of my books; "Windows Forensic Analysis" (1st thru 4th editions), "Windows Registry Forensics",
as well as the book I co-authored with Cory Altheide, "Digital Forensics with Open Source Tools".
 Monday, October 10, 2022
 Post Compilation
 For this post, I'll throw out a bunch of little snippets, or "post-lets", covering a variety of DFIR topics rather than one big post that covers one topic.
 What's Old Is New Again
 During Feb, 2016, Mari published a fascinating blog post regarding the VBAWarnings value . That was a bit more than 6 1/2 yrs ago, which in "Internet time" is several lifetimes.

 Just this past September, Avast shared a write-up of the Roshtyak component of Raspberry Robin, where they described some of the techniques used by this malware, including checking the VBAWarnings value as a means of "detecting" virtual or testing environments.
 Getting PCAPs
 When I've been asked on-site (or remotely), it's most often been after an incident has happened. However, that doesn't mean that I shouldn't have a means available for myself, or to share with IT admins, to collect pcaps. Having something like this readily available can be very beneficial, when you need it.
 It seems that Windows 10 and above comes with a native tool for collecting network traffic data called pktmon .
 Prefer Powershell? Doug Metz over at BakerStreetForensics has a solution for you.
 I've used bulk_extractor to get pcaps from memory dumps; because this uses a different means for identifying network connections than Volatility, running them both is a really, REALLY good idea! So good, as a matter of fact, that I included an example of this in Investigating Windows Systems , which just shows that regardless of the version of Windows you're dealing with, the process still holds up.
 Memory Analysis
 Or, if you're looking for a bit more, consider bulk_extractor with record carving .
 Also, if you're doing memory analysis, you might consider tools such as MemProcFS and MemProcFS-Analyzer . While I'm not a fan of a lot of the available GUI tools that folks (generally) use for analysis, this tweet from "evild3ad79" makes visualizing processes so much easier!
 MOTW
 MOTW, or "mark-of-the-web" is a pretty hot topic, as it should be. "MOTW" is the NTFS alternate data stream, or "ADS", attached to files downloaded from the Internet, and something we've seen expand over time. At first these were simply "zone identifier" ADSs, and contained just that...the "zone" for the downloaded file. We first saw these associated with files downloaded via IE and Outlook, and then later saw MOTW attached to files downloaded via other browsers.
 MOTW picked up steam a bit after MS announced that they were going to change the default behavior of systems running macros in Office documents downloaded from the Internet . We then saw some actors move to using archives rather than "weaponized" Office documents, and our attention shifted to archive utilities and MOTW propagation .
 For a bit of a different perspective on MOTW, Outflank published this article discussing MOTW from a red team perspective .
 And, to top it all off, MS has shared information regarding how to disable the functionality (of attaching MOTW). What this does provide is an excellent opportunity for detections, both in the SOC (adding or modifying the SaveZoneInformation value) and for DFIR (checking the value).
 Web Shells
 Many, many moons ago (circa 2007, 2008), Chris Pogue and I were addressing investigating SQL Injection and web shells for the IBM ISS X-Force ERS team, codifying (or trying to) some basic processes for locating these attacks in a reactive, DFIR mode. We had a lot of different approaches, all of which could be addressed programmatically...things such as the first instance of a page being requested (across the history of the web server logs that you have available), the number of times a page was requested, the length of the request sent to the page, User Agents, etc. Now, all of these depended upon which fields were actually being logged, so we started with the default IIS logging fields and attempted to modify and address things from there. This way, encountering IIS logs with the fields having been modified (hopefully, added to...) or non-IIS web servers were considered "one-offs", and we found that the approach worked well.
 I learned recently that Aaron Shelmire authored a blog on this topic for Anomali ; this was a great finding, not just because it lists some of the things we'd looked for, but also because Aaron and I worked together at one point. It's great to see contributions like this within the community.
 Events Ripper
 Not long ago, I released Events Ripper , a proof-of-concept tool based on RegRipper , in that it relies on plugins to extract and present data. The idea behind Events Ripper is to leverage what analysts are already doing to provide situational awareness and pivot points for analysis. So, when analysts are performing timeline creation (and moving to timeline analysis), they can leverage the events file they've already created to obtain insight into the system.
 At this point, all of the Events Ripper plugins are based on data in an events file, from parsed Windows Event Logs (via wevtx.bat ). For example, I wrote two plugins recently, mount.pl and ntfs.pl , that analysts can use to verify initial access used by Raspberry Robin ; mount.pl runs through the events file looking for Microsoft-Windows-VHDMP/1 events indicating that a disk was "surfaced", and outputs a list of the VHD[X] and/or ISO files. Ntfs.pl looks for Microsoft-Windows-Ntfs/145 events to locate volumes, and map them to the drives or devices. Using these two plugins, you can get some quick insight as to how Raspberry Robin (or other malware) may have originally made it on to the system...via a USB thumb drive or ISO file delivered as an email attachment.
 Interestingly enough, when I was developing and testing the mount.pl plugin, the Microsoft-Windows-VHDMP/Operational.evtx log file from my test system contained three ISO files. Checking the RecentDocs/.iso values in the Registry, I found those same three files listed, as well.
 Per a request from my esteemed co-worker Dray , all of the plugins display the system name, or names, as the case may be. It's not unusual for systems to start out as a gold image and be renamed, so you may have event records that still contain the original system name.
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
 ▼ October (6) Testing Registry Modification Scenarios
 Data Collection
 Events Ripper
 We Need Cybersecurity Mentors
 Post Compilation
 Speaking Engagements

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
