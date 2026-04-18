# Windows Incident Response

- URL: https://windowsir.blogspot.com/2022/10/testing-registry-modification-scenarios.html
- Published: unknown
- Updated: unknown
- Labels: none

Windows Incident Response
 The Windows Incident Response Blog is dedicated to the myriad information surrounding and inherent to the topics of IR and digital analysis of Windows systems. This blog provides information in support of my books; "Windows Forensic Analysis" (1st thru 4th editions), "Windows Registry Forensics",
as well as the book I co-authored with Cory Altheide, "Digital Forensics with Open Source Tools".
 Monday, October 31, 2022
 Testing Registry Modification Scenarios
 After reading some of the various open reports regarding how malware or threat actors were "using" the Registry, manipulating it to meet their needs, I wanted to take a look and see what the effects or impacts of these actions might "look like" from a dead-box, DFIR perspective, looking solely at the Registry.  I wanted to start with an approach similar to what I've experienced during my time in IR, particularly the early days, before EDR, before things like Sysmon or enabling Process Tracking in the Security Event Log. I thought that would be appropriate, given what appears to be the shear number of organizations with limited visibility into their infrastructures. For those orgs that have deployed Sysmon, the current version (v14.1) has three event IDs (12, 13, and 14) that pertain to the Registry.
 The first scenario I looked at was from this Avast write-up on Raspberry Robins's Roshtyak component ; in the section titled "Indirect registry writes", the article describes the persistence mechanism of renaming the RunOnce key, adding a value, then re-renaming the key back to "RunOnce", apparently in an effort to avoid rules/filters that look specifically for values being added to the RunOnce key. As most analysts are likely aware, the purpose of the RunOnce key is exactly that...to launch executables once. When the RunOnce key is enumerated, the value is read, deleted, and the executable it pointed to is launched. In the past, I've read about malware executables that are launched from the RunOnce key, and the malware itself, once executed, will re-write a value to that key, essentially allowing the RunOnce key and the malware together to act as if the malware were launched from the Run key.
 I wanted to perform this testing from a purely dead-box perspective. Using EDR tools, or relying on the Windows Event Logs. Depending upon your configuration, you could perhaps look to the Sysmon Event Log, or if the system had been rebooted, you could also look to the Microsoft-Windows-Shell-Core%4Operational.evtx Event Log and Events Ripper to percolate unusual executables.
 For reference, information on the Registry file format specification can be found here .
 Methodology
 The first thing I did was use "reg save" to create a backup of the Software hive. I then renamed the RunOnce key, and added a value (i.e., "Calc"), and renamed the key back to "RunOnce", all via RegEdit. I then closed RegEdit and used "reg save" to create a second copy of the Software hive. I then opened RegEdit, deleted the value, and saved a third copy of the Software hive.
 During this process, I did not reboot the system; rather, I 'simulated' a reboot of the system by simply deleting the added value from the RunOnce key. Had the system been rebooted, there would likely be an interesting event record (or two) in the Microsoft-Windows-Shell-Core%4Operational.evtx Event Log.
 Finally, I created a specific RegRipper plugin to extract explicit information about the key from the hive file.
 First Copy - Software
 So, again, the first thing I wanted to do was create a baseline; in this case, based on the structure for the key node itself.

 Using the API available from the Perl Parse::Win32Registry module, I wrote a RegRipper plugin to assist me in this testing. I wanted to get the offset of the key node; that is, the location within the hive file for the node itself. I also wanted to get both the parsed and raw information for the key node. This way, I could not only see the parsed data from within the structure of the key node itself, but I could also see the raw, binary structure, as well.
 Second Copy - Software2
 After renaming the RunOnce key, adding a value, and re-renaming the key back to "RunOnce", I saved a second copy of the Software hive, and ran the runonce_test.pl plugin to retrieve the information illustrated in figure 2.

 We can see between figures 1 and 2 that there are no changes to the offset, the location of the key within the hive file itself. In fact, the only changes we do see are the LastWrite time (which is to be expected), and the number of values, which is now set to 1.
 Third Copy - Software3
 The third copy of the Software hive is where I had deleted the value that had been added. Again, this was intended to simulate rebooting the system, and did not account for the malware adding a reference to itself back to the RunOnce key once it was launched.
 Figure 3 illustrates the output of the plugin run against the third copy of the Software hive.

 Again, the offset/location of the key node itself hasn't changed, which is to be expected. Deleting the value changes the number of values to "0", and adjusts the key LastWrite time (which is to be expected).
 I then ran the del.pl plugin (to get deleted keys and values from unallocated space within the hive file) against the third copy of the Software hive, opened the output in Notepad++, searched for "calc", and found the output shown in figure 4 below. I could have used regslack, from Jolanta Thomassen (go here to see Jolanta's thesis from 2008), but simply chose the RegRipper plugin because I was already using RegRipper.

 Unfortunately, value nodes contain neither time stamps, nor a reference back to the original key node (parent key offset) to which they were a member, as described in sections 4.1.1 and 4.1.2 of the Registry file format specification for key nodes; value node structures are described in sections 4.4.1 and 4.4.2.
 Conclusion
 As we can see from this testing, there's not much that we can see just from the Registry hive file that would lead us to believe that anything unusual had happened. While we might have an opportunity to see something of this activity via the transaction logs, that would depend a great deal upon how long after the activity that the incident was discovered, the amount of usage on the system, etc. It appears that the way this specific activity would be discerned would be through a combination of malware RE, EDR, Windows Event Log records, etc.
 Next, I'll take a look at at least one of the scenarios presented in this Microsoft blog post .

 Addendum, 1 Nov : Maxim Suhanov reached to me about running "yarp-print --deleted" to get a different view of deleted data within the hive, and I found some anomalous results that I simply cannot explain. As a result, I'm going to completely re-run the tests, fully documenting each step, and providing the results again.
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
