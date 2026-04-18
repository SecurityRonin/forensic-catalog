# Windows Incident Response

- URL: https://windowsir.blogspot.com/2023/08/the-next-step-expanding-regripper.html
- Published: unknown
- Updated: unknown
- Labels: none

Windows Incident Response
 The Windows Incident Response Blog is dedicated to the myriad information surrounding and inherent to the topics of IR and digital analysis of Windows systems. This blog provides information in support of my books; "Windows Forensic Analysis" (1st thru 4th editions), "Windows Registry Forensics",
as well as the book I co-authored with Cory Altheide, "Digital Forensics with Open Source Tools".
 Saturday, August 12, 2023
 The Next Step: Expanding RegRipper
 I thought I'd continue The Next Step series of blog posts with something a little different. This "The Next Step" blog post is about taking a tool such as RegRipper to "the next step", which is something I started doing in August, 2020. At first, I added MITRE ATT&CK mapping and Analysis Tips, to provide information as to why the plugin was written, and what an analyst should look for in the plugin output. The Analysis Tips also served as a good way of displaying reference URLs, on which the plugin may have been based. While the reference URLs are very often included in the header of the plugin itself, it's often simply much easier to have them available in the output of the plugin, so that they follow along and are available with the data and the case itself.
 So, in the spirit of the blog series, here are a couple of "the next steps" for RegRipper...
 JSON
 Something I've looked at doing is creating plugins that provide JSON-formatted output. This was something a friend asked for, and more importantly, was willing to discuss. When he asked about the format, my concern was that I would not be able to develop a consistent output format across all plugins, but during the discussion, he made it clear that that wasn't necessary. I was concerned about a consistent, normalized format, and he said that as long as it was JSON format, he could run his searches across the data. I figured, "okay, then", and gave it a shot. I started with the appcompatcache.pl plugin, as it meant just a little bit of code that repeated the process over and over again...an easy win. From there, I modifying the run.pl plugin, as well.
 An excerpt of sample output from the appcompatcache_json.pl plugin, run against the System hive from the BSides Amman image appears as follows:
 {
 "value": "C:\Users\Joker\DCode.exe"
 "data": "2019-02-15 04:59:23"
 },
 {
 "value": "C:\Windows\SysWOW64\OneDriveSetup.exe"
 "data": "2018-04-11 23:34:02"
 },
 ]
 }
 So, pretty straightforward. Now, it's a process of expanding to other plugins, and having the ability with the tool itself to select those plugin output types the analyst is most interested in.
 Yara
 Something else I've looked at recently is adding the ability to incorporate Yara into RegRipper. While I was at Nuix, I worked with David Berry's developers to get some pretty cool extensions added to the product; one for RegRipper, and one for Yara . I then thought to myself, why not incorporate Yara into RegRipper in some manner? After all, doing things like detecting malware embedded in value data might be something folks wanted to do; I'm sure that there are a number of use cases.
 Rather than integrating Yara into RegRipper, I thought, why re-invent the wheel when I can just access Yara as an external application? I could take a similar approach as to the one used by the Nuix extensions, and run Yara rules against value data. And, it wouldn't have to be all value, as some types won't hold base64-encoded data. In other instances, I may only want to look at binary data, such as searching for payloads, executables, etc. Given that there are already plugins that recursively run through a hive file looking at values and separating the actions taken based on data type, it should be pretty easy to gin up a proof of concept.
 And, as it turns out, it was. I used the run.pl plugin as a basis, and instead of just displaying the data for each value, I ran some simple Yara rules against the contents. One of the rules in the rule file appears as follows:
 rule Test3
 {
 strings:
 $str1 = "onedrive" nocase
 $str2 = "vmware" nocase
 condition:
 $str1 or $str2
 }
 Again, very simple, very straightforward, and simply designed to produce some output, nothing more.
 The output from the plugin appears as follows:

 Now, I'll admit up front...this is just a proof of concept. However, it illustrates the viability of this technique. Now, using something like the sizes.pl plugin, I can remove the code that determines the number of values beneath a key, and focus on just scanning the value data...all of it. Or, I can have other plugins, such as clsid.pl , comb through a specific key path, looking for payloads, base64-encoded data, etc. Why re-write the code when there are Yara rules available that do such a great job, and the rules themselves may already be part of the analyst's kit.
 Techniques like this are pretty powerful, particularly when faced with threat actor TTPs, such as those described by Prevalion in their DarkWatchman write-up :
 Various parts of DarkWatchman, including configuration strings and the keylogger itself, are stored in the registry to avoid writing to disk.
 So, with things like configuration strings and an entire keylogger written to the Registry, there are surely various ways to go about detecting the presence of these items, including key LastWrite times, the size of value data, and now, the use of Yara to examine data contents.
 As with the JSON output plugins, now it's simply a matter of building out the capability, in a reasonable fashion.
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

 ▼ 2023 (50) ► December (3)
 ► November (2)
 ► October (1)
 ► September (2)
 ▼ August (7) Book Review: Effective Threat Investigation for SO...
 The Next Step: Integrating Yara with RegRipper, pt II
 Yet Another Glitch In The Matrix
 Integrating Yara with RegRipper
 The Next Step: Expanding RegRipper
 Ransomware Attack Timeline
 Events Ripper Updates

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
