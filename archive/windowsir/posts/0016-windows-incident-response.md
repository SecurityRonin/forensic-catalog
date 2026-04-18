# Windows Incident Response

- URL: https://windowsir.blogspot.com/2006/11/case-study-from-securiteam.html
- Published: unknown
- Updated: unknown
- Labels: none

Windows Incident Response
 The Windows Incident Response Blog is dedicated to the myriad information surrounding and inherent to the topics of IR and digital analysis of Windows systems. This blog provides information in support of my books; "Windows Forensic Analysis" (1st thru 4th editions), "Windows Registry Forensics",
as well as the book I co-authored with Cory Altheide, "Digital Forensics with Open Source Tools".
 Thursday, November 16, 2006
 Case Study from SecuriTeam

 7 comments:
 Great comment...what're your thoughts on that?
 "fight between the attackers...and the incident response personnel."
 Check out Steps 10 and 11.

 SIMBAR:
 http://vil.nai.com/vil/content/v_131206.htm
 http://www.google.com/search?hl=en&lr=&q=SIMBAR&btnG=Search
 http://www.google.com/search?hl=en&lr=&q=%22SIMBAR+Enabled%22+spyware+&btnG=Search

 The truth is that it doesn't really matter what added the "SIMBAR Enabled" to the user-agent. The idea is that the attacker user-agent contained a string that helped to pinpoint his traffic in the logs. Finding SIMBAR in the second incident may have been coincidence, but once again it helped us to mark the intruder's activity.

 Further more it was mentioned that there were a few computers involved in both incidence. The "Simbar" computer was involved in the initial attack and uploaded some tools in both of the incidence.

 If every thing was known, there wasn't any place of assumptions and there wasn't any thing to investigate.
 “There are things known and there are things unknown, and in between are the doors of perception” (Aldous Huxley)

 (:
 kfir d...

 Hey, thanks for dropping by and commenting...

 Check out Steps 10 and 11.

 I did. In step 10, the authors state that they found a user logged in and attempting to use the exploit. However, that's it...there's no further discussion of the attackers/users battling the responders. In fact, step 10 doesn't say anything about the responders actively battling the bad guys...it just says that a user was logged in and says nothing at all about what the responders did with regards to that fact. Did the bad guy taunt the responders with messages? Did the bad guy try to delete things?

 The truth is that it doesn't really matter what added the "SIMBAR Enabled" to the user-agent.

 Excellent point. If that's the case, though, why was it so prominently mentioned in both reports? You could have simply stated that the string assisted in log analysis.

 BTW...I did the same searches as you did, and found the same things. In fact, the second search string was the first one I used.

 IMHO, I think it would've had greater impact in the report had the authors really went with the fact that the "SIMBAR" string assisted with log analysis, and perhaps stayed away from speculative things like the stating that the remote system was infected with spy- or adware.

 If every thing was known, there wasn't any place of assumptions and there wasn't any thing to investigate.

 I tend to agree with your statement in general, but I'm not clear on how it applies to this incident. It seems very clear to me that the authors had a lot that they didn't know, and a lot that they investigated. It also seems that some of the speculation was unnecessary...you even pointed that out yourself.

 Again, I do applaud their efforts overall, and I thank them for taking the time and effort to write up what they did.
 I suppose the fact that they had management approval to use the tool left by the attacker and get the SQL password mitigates any liability on their part. I don't know, just doesn't feel right to me.

 They did do a great job, but as you note, not a professional job.
 Well, from a forensic perspective, that may not have been the best thing to do, but from an operational perspective, it may have been the ONLY thing to do!
 That brings up another question - is incident response turning into "get the machine up ASAP" at the expense of collecting evidence?
 No, it's always been that way.
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

 ▼ 2006 (118) ► December (1)
 ▼ November (16) Artifact classes
 MS AntiMalware Team paper
 Some sites of note
 A little programming fun, etc.
 How do you do that voodoo that you do?
 Case Study from SecuriTeam
 Van Hensing Rides...uh, Blogs Again!
 Trends in Digital Forensics, and news
 Evidence Dynamics
 OMG! So says Chicken Little...
 Parsing Raw Registry Files
 ComputerWorld: Undisclosed Flaws Undermine IT Defe...
 DoD CyberCrime 2007
 Forensics Blogs/Podcasts
 Updates - Code and such
 Documenting Disk Information

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
