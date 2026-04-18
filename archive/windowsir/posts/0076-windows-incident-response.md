# Windows Incident Response

- URL: https://windowsir.blogspot.com/2011/09/jump-list-analysis-pt-iii.html
- Published: unknown
- Updated: unknown
- Labels: none

Windows Incident Response
 The Windows Incident Response Blog is dedicated to the myriad information surrounding and inherent to the topics of IR and digital analysis of Windows systems. This blog provides information in support of my books; "Windows Forensic Analysis" (1st thru 4th editions), "Windows Registry Forensics",
as well as the book I co-authored with Cory Altheide, "Digital Forensics with Open Source Tools".
 Thursday, September 08, 2011
 Jump List Analysis, Pt III

 10 comments:
 Not taking anything away from your post but this capability of determining execution of a program using data other than access times has been known through similar means for some time.
 Thanks for the mention, Harlan.

 Jamie's suggestion is one that shouldn't be overlooked. I saw her response as well and thought to myself, "well, it definitely would open a lot of doors to find that out." But like you said, the calculation might be similar to a one-way hash, rendering reverse calculation an exercise in futility.

 However, as Jamie said, it's probably more important to confirm that the hash is or is not reversible than to compile lists of AppIDs that may not be maintained forever. I suppose the AppIDs would be most suited for cases in which the suspect is not particularly tech-savvy and installs apps to their default locations.

 In any case, I'm glad more people are looking into Jump Lists. Great post.

 -Dan
 Dan,

 I agree that Jamie's suggestion is indeed important, and I wanted to provide a solution that could be used immediately, while (hopefully) efforts are made to pursue identifying the algorithm and developing a solution that way.

 Thanks.
 Anonymous,

 Not taking anything away from your post but this capability of determining execution of a program using data other than access times has been known through similar means for some time.

 I'm sure...but the point of the post isn't specifically about determining the execution of a program. It's about identifying which application was run when either the Jump List AppID is unknown, or when the application was deleted.

 Besides, is there really anything wrong with saying it again, or pointing out another artifact (Jump Lists) that can be used?
 As far as figuring out how AppIDs are calculated: even if an AppID is a one way hash like a prefetch hash this is useful. This is because if you know how to calculate the hash, you can validate the AppID of an unknown application in a jumplist on the fly instead of having to do so manually. If you have several unknown applications this saves a lot of time. Since these AppIDs seem to be stable across systems and they are probably a one-way hash calculated using application name, version and path, it just seems like it would be worth figuring out how the hash is calculated... I also agree that creating these lists is a good start.
 Sorry forgot to add that once you know the algorithm you can programically try variations on the application name, versions, path etc until you create a hash that matches the AppID. Most likely you'd probably know the program name, location and maybe even version if it were still on the acquired disk... That way it doesn't really matter that you can't get the name from the hash itself, you create the hashes and see if one matches... does that make sense?
 Nope, nothing wrong with saying it again. Just decided to add an equally insulting comment about your research as you made about Registry Decoder on your email list. It was a parody of your earlier comment to Andrew Case:

 "Not taking anything away from Andrew's efforts (much applause to him) but this capability has been available through other means for some time. For example, installing the Parse::Win32Registry module adds the regdiff.pl script to your installation."
 Anonymous,

 Just decided to add an equally insulting comment...

 How as my comment insulting? I have great respect for the work that Andrew did. I guess you can make anything sound the way you like.

 How about providing your name?
 @Jamie:

 ...you can programically try variations on the application name, versions, path etc until you create a hash that matches the AppID.

 This is very true and certainly makes sense. I wrote a little about how one could manually compare the AppIDs, but it's a very roundabout way of finding a match. Once the algorithm is discovered, automation of this process would be a viable option -- subsequently easing the pain of doing a manual, one-by-one check.

 I totally agree with you that finding out how these AppIDs are calculated would be a discovery of great value. Thanks for the insight on this; it's great to see this issue from many different angles.

 -Dan
 If you haven't looked at @Hexacorn's blog post on AppID and Jumplist filename calculation, be sure to look at it. It sheds light on the way AppIDs are calculated; the exact issue we were discussing here a while back.

 http://www.hexacorn.com/blog/2013/04/30/jumplists-file-names-and-appid-calculator/
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
 ► October (10)
 ▼ September (15) NoVA Forensics Meetup Reminder
 Friday Stuff
 Links and Updates
 Links...and whatnot
 NoVA Forensics Meetup Group
 HowTo: Mount and Access VSCs
 HowTo: File Extension Analysis
 HowTo: Creating Mini-Timelines
 Growing the NoVA Forensics Meetup
 Updates and Links
 Jump List Analysis, Pt III
 Registry Stuff
 Getting Started
 Stuff...and whatnot
 Friday Updates

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
