# Windows Incident Response

- URL: https://windowsir.blogspot.com/2022/02/the-misuse-of-artifact-categories.html
- Published: unknown
- Updated: unknown
- Labels: none

Windows Incident Response
 The Windows Incident Response Blog is dedicated to the myriad information surrounding and inherent to the topics of IR and digital analysis of Windows systems. This blog provides information in support of my books; "Windows Forensic Analysis" (1st thru 4th editions), "Windows Registry Forensics",
as well as the book I co-authored with Cory Altheide, "Digital Forensics with Open Source Tools".
 Monday, February 28, 2022
 The (Mis)Use of Artifact Categories
 Very often in DFIR, we categorize artifacts in an easy-to-understand and easy-to-digest manner, as using or relying on these categories often helps us navigate our investigations. There are also times when we reduce those artifacts to a level where they're easier to understand, and in doing so, the categorization of the artifact isn't quite accurate. As such, it's necessary now and again to go back and take a look at that categorization to determine if still holds, or if it truly served the community in the manner intended.
 SPOILER ALERT - TL:DR
 Within the DFIR community, we should not be hanging investigation findings on single artifacts in isolation. If there are gaps in data , they need to be recognized, understood and communicated. Do not spackle those gaps over with guesswork and assumption; instead, ensure that you're validating your findings through artifact constellations.
 Program Execution
 One such artifact category is Program Execution. As with other categories, this one often lists single artifacts, in isolation, that provide an indication to the examiner that a program may have been executed on the endpoint. This issue of the category's misuse sprouts from when analysts do not use it as an indication for further investigation, but instead look to those single artifacts on which to build their findings, without further validation or support.
 To start, the MITRE ATT&CK framework includes an "Execution" tactic, and the description of the Execution tactic includes the following:
 The adversary is trying to run malicious code.
 Execution consists of techniques that result in adversary-controlled code running on a local or remote system. Techniques that run malicious code are often paired with techniques from all other tactics to achieve broader goals, like exploring a network or stealing data.
 Note that the description specifies "...trying to run code"; this does not say that it succeeded. This is an apt and necessary distinction, as there are times (many, many times) when code is attempted to be run but is not successful for some reason. As stated in the description, the tactic provides a number of techniques that describe how an actor might attempt to initiate execution of a program or process, but that does not guarantee that the code runs successfully.
 The SANS DFIR poster includes a "Program Execution" section, which includes individual artifacts such as ShimCache, AmCache, etc. all of which are meant to provide indications of...yes..."program execution". However, some of the artifacts mentioned do not specifically indicate that a program executed (only that the file existed on the system) in and of themselves, while others provide simply an indication that an actor (the user or a threat actor) may have attempted to initiate a program. The important thing to remember here is that these indicators are just that...indicators...and that further analysis is required.
 As it turns out, we (those of us in the DFIR community) are misusing the term "program execution", to the point where a more apt description might be that we're abusing it. What I mean is that we may look to a single artifact...EDR telemetry, a ShimCache entry, UserAssist entry, or another Registry entry of some kind...and assume that it means that a program was executed, that it was launched or started, and at some point it successfully completed execution. As such, the "program execution" category can sometimes be specious; attempts may have been made to create a process, but for some reason, those attempts failed. As such, there was no actual "program execution", in the sense that a process was created and completed it's intended function. Relying on a single artifact to support a finding or theory is insufficient analysis, and can get us into trouble because we will be incorrectly reporting what occurred on the system.
 Instances of "program execution" misuse result from insufficient investigations, and often start out as a SOC alert based on NDR or EDR telemetry, or as an initial finding during a DFIR investigation. If we suspect that code has been executed, or we're attempting to establish if code has been executed, we need to ensure that we're not basing our findings on single "program execution" artifacts in isolation, but instead pursuing additional artifacts within the constellation to support our finding.
 What we should expect to see (artifact constellation/toolmarks):
 - Prefetch file, depending upon the Windows version
 - UserAssist entry, if launched by the user double-clicking via Explorer
 - Impact of the execution on the system (Registry keys/values, Windows Event Log records, etc.)
 Sometimes we might not find these additional artifacts from the constellation; if this is the case, we need to determine why.
 What can impact "program execution":
 - AV, NGAV, EDR (in block mode)
 - Privilege management software (BeyondTrust, OneIdentity, etc.)
 - System configurations
 - Coding errors
 - System region or "locale" settings
 The point is that in some cases, we may "see" what we think is an attempt to launch a program or script, but we need to look beyond that single artifact, and examine the system more broadly to validate the finding. Did Windows Error Reporting (WER) kick off as the launch attempt occurred? Was an application popup error message or a crash dump generated? Was a log entry generated as a result of UAC or some other protection mechanism blocking the execution?
 We need to ensure that when we see a single artifact in isolation, we have to ensure that we're validating our findings through the use of artifact constellations and toolmarks.
 A Closer Look At Some Artifacts
 What do some of the artifacts that we look to as indicators of "program execution" really tell us? What do they really mean?

 With respect to the ShimCache artifacts, there is a Mandiant blog post from 2015 which states (emphasis added):

 It is important to understand there may be entries in the Shimcache that were not actually executed .

 Further, in this article from CountUpon Security , we see (emphasis added):

 In addition the ShimCache tracks executables that have not been executed but were browsed for example through explorer.exe. This makes a valuable source of evidence for example to track executables that were on the system but weren’t executed ...

 I should note that many of these articles are older, covering Windows XP and Windows 7/2008, and there have been some modifications to what is available in Windows 10. For example, the ShimCache entries on Windows 10 still maintain the time stamp and file path, but the "execution flag" seen on earlier versions of Windows does not appear to be available. However, the time stamp within the entry is still the file system last modification time, extracted from the $STANDARD_INFORMATION attribute within the MFT. As such, if the threat actor were to create the file on the system (copy it over, extract it from an archive, etc.), and then time stomp the file, when the file is added to the ShimCache, it will have the "new", albeit incorrect time data. I've seen this happen on a number of incidents, specifically with respect to PCI forensic investigations, where an accurate understanding of the "window of compromise" is mandated by the PCI Council.

 As for the AmCache artifacts that may be found, this paper provides a much more detailed examination of the forensic uses of the artifact, based on library versions, and within the paper, those libraries are associated with specific Windows 10 builds. A thorough read of the paper leaves us with the clear understanding that simply because there is an entry in the AmCache for a file, this does NOT specifically indicate that it was executed. For example, section 6.2 states (emphasis added):

 Secondly, the last write time of the subkey coincides with either the first time of execution of the PE or the time of installation of the program .
 Given this, while the AmCache is listed as an "indicator of program execution", we cannot simply state that there was, in fact, program execution based on an entry in the file. Where the entry is located (beneath which key) within the AmCache.hve file is an important factor, as is the fact that the existence of the entry does not specifically correlate to program execution.
 Consider this tweet from Maxim Suhanov (circa Nov, 2018):
 Amcache entries are created for executables that were never executed. Executables that were launched and then deleted aren't recorded. Also, Amcache entries aren't created for executables in non-standard locations (e.g., "C:\1\2\") _unless_ they were actually executed.
 As such, it should be clear that, in isolation, AmCache entries do not explicitly state that a program was run, but rather provides an indication that the file was on the endpoint, and may have been run.
 Something else Maxim has also pointed out is that the hashes in the AmCache file are generated for the first 30Mb of the file. As such, if the file is greater than 30Mb , the hash you generate for the file itself will not match what's listed in the AmCache.hve file, and the hash from the AmCache.hve file should not be used for open source searches.
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
 ► March (4)
 ▼ February (2) The (Mis)Use of Artifact Categories
 LNK Files, Again

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
