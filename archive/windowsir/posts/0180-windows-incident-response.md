# Windows Incident Response

- URL: https://windowsir.blogspot.com/2022/11/challenge-7-write-up.html
- Published: unknown
- Updated: unknown
- Labels: none

Windows Incident Response
 The Windows Incident Response Blog is dedicated to the myriad information surrounding and inherent to the topics of IR and digital analysis of Windows systems. This blog provides information in support of my books; "Windows Forensic Analysis" (1st thru 4th editions), "Windows Registry Forensics",
as well as the book I co-authored with Cory Altheide, "Digital Forensics with Open Source Tools".
 Sunday, November 27, 2022
 Challenge 7 Write-up
 Dr. Ali Hadi recently posted another challenge image , this one (#7) being a lot closer to a real-world challenge than a lot of the CTFs I've seen over the years. What I mean by that is that in the 22+ years I've done DFIR work, I've never had a customer pose more than 3 to 5 questions that they wanted answered, certainly not 51. And, I've never had a customer ask me for the volume serial number in the image. Never. So, getting a challenge that had a fairly simple and straight forward "ask" (i.e., something bad may have happened, what was it and when??) was pretty close to real-world.
 I will say that there have been more than a few times where, following the answers to those questions, customers would ask additional questions...but again, not 37 questions, not 51 questions (like we see in some CTFs). And for the most part, the questions were the same regardless of the customer; once whatever it was was identified, questions of risk and reporting would come up, was any data taken, and if so, what data?
 I worked the case from my perspective, and as promised, posted my findings , including my case notes and timeline excerpts. I also added a timeline overlay , as well as MITRE ATT&CK mappings (with observables) for the "case".
 Jiri Vinopal posted his findings in this tweet thread ; I saw the first tweet with the spoiler warning, and purposely did not pursue the rest of the thread until I'd completed my analysis and posted my findings. Once I posted my findings and went back to the thread , I saw this comment:
 "...but it could be Windows server etc..so prefetching could be disabled..."

 True, the image could be of a Windows server, but that's pretty trivial to check, as illustrated in figure 1.

 Checking to see if Prefetching is enabled is pretty straightforward, as well, as illustrated in figure 2.

 If prefetching were disabled, one would think that the *.pf files would simply not be created, rather than having several of them deleted following the installation of the malicious Windows service. The Windows Registry is a hierarchal database that includes, in part, configuration information for the Windows OS and applications, replacing the myriad configuration and ini files from previous versions of the OS. A lot of what's in the Registry controls various aspects of the Windows eco-system, including Prefetching.
 In addition to Jiri's write-up/tweet thread of analysis, Ali Alwashali posted a write-up of analysis, as well . If you've given the challenge a shot, or think you might be interested in pursuing a career in DFIR work, be sure to take a look at the different approaches, give them some thought, and make comments or ask questions.
 Remediations and Detections
 Jiri shared some remediation steps , as well as some IOCs , which I thought were a great addition to the write-up. These are always good to share from a case; I included the SysInternals.exe hash extracted from the AmCache.hve file, along with a link to the VT page, in my case notes .
 What are some detections or threat hunting pivot points we can create from these findings? For many orgs, looking for new Windows service installations via detections or hunting will simply be too noisy, but monitoring for modifications to the /etc/hosts file might be something valuable, not just as a detection, but for hunting and for DFIR work.
 Has anyone considered writing Yara rules for the malware found during their investigation of this case? Are there any other detections you can think of, for either EDR or a SIEM?
 Lessons Learned
 One of the things I really liked about this particular challenge is that, while the incident occurred within a "compressed" timeframe, it did provide several data sources that allowed us to illustrate where various artifacts fit within a "program execution" constellation. If you look at the various artifacts...UserAssist, BAM key, and even ShimCache and AmCache artifacts...they're all separated in time, but come together to build out an overall picture of what happened on the system. By looking at the artifacts together, in a constellation or in a timeline, we can see the development and progression of the incident, and then by adding in malware RE, the additional context and detail will build out an even more complete picture.
 Conclusions
 A couple of thoughts...
 DFIR work is a team effort. Unfortunately, over the years, the "culture" of DFIR has been one that has developed into a bit of a "lone wolf" mentality. We all have different skill sets, to different degrees, as well as different perspectives, and bringing those to bear is the key to truly successful work. The best (and I mean, THE BEST ) DFIR work I've done during my time in the industry has been when I've worked as part of team that's come together, leveraging specific skill sets to truly deliver high-quality analysis.
 Thanks
 Thanks to Dr. Hadi for providing this challenge, and thanks to Jiri for stepping up and sharing his analysis!
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
 ▼ November (4) Post Compilation
 Challenge 7 Write-up
 Thoughts on Teaching Digital Forensics
 RegRipper Value Proposition

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
