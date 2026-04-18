# Windows Incident Response

- URL: https://windowsir.blogspot.com/2011/06/defining-forensic-value.html
- Published: unknown
- Updated: unknown
- Labels: none

Windows Incident Response
 The Windows Incident Response Blog is dedicated to the myriad information surrounding and inherent to the topics of IR and digital analysis of Windows systems. This blog provides information in support of my books; "Windows Forensic Analysis" (1st thru 4th editions), "Windows Registry Forensics",
as well as the book I co-authored with Cory Altheide, "Digital Forensics with Open Source Tools".
 Wednesday, June 22, 2011
 Defining "Forensic Value"

 14 comments:
 > Who defines "forensic value" when it comes to data?

 Great post and very thought provoking. Tools can add different functionality or people can present of different tools/artifacts but I think practitioners finally realize the forensic value of data once it has an impact on their case or they can see how it applies their work. Take timelines for example. There were a lot of people saying how valuable the timeline analysis technique was but I didn’t fully realize the power of timelines until I started using them to answer questions. Now I found myself saying how valuable timelines are and some examiners don’t see the value of them.

 There are different learning styles and maybe for seeing the forensic value of data falls more in the category of learning by doing instead of listening and seeing.
 Very interesting and thoughtful, Harlan. I tend to think that value is in the eye of the beholder (or hands of the practitioner, as it were). It's very subjective. If you are working on something and think it has value, then it does. Just as with Corey's example of timelines, not everyone may agree; over time, overall opinion may change regarding community-wide value.

 At the same time I can see where it could be frustrating to spend time and energy researching, testing, and presenting information - only to be met by blank stares. Not saying you were, but I know that sometimes positive feedback is needed to fuel continued work in an area.

 If someone's having to sell the value of what they're doing in order to justify their existence, I still think value is subjective. The difficulty then becomes the necessity to have particular people drink the koolaid. ;)
 Great post Harlan. First time caller, long time listener. Sorry in advance if I digress a little...

 As a member of LE in Canada doing tech crime work, I know Value to the Case is something that we reinforce with junior examiners. The baseline for us starts with looking at the charges, and what the elements of the offence are, thus what type of records of activity need to be uncovered. The assumption being that the investigator has seized the computer with grounds making that evidence potentially relevant - but we'll make that leap.

 Working in that direction, an understanding of the case and elements to be proven is very important. That combined with a thorough knowledge of the technology and where relevant artifacts lie, will allow the examiner to more directly identify those artifacts with a higher forensic value. I believe "Forensic Value" can best be determined when you have a strong understanding of the high level elements you are trying to prove.

 On your forensic scanner idea: Something like this is needed badly for a couple of reasons.

 1) It would be nice if there was a way (we are exploring this in-house) that initial artifacts could be pulled off the digital evidence in a forensically sound manner even at the scene (field triage) to hand off to the investigator for interview purposes immediately. A tool that can accomplish this needs to be a closed loop tool (so they don't hang themselves) usable by field investigators with minimal but requisite training. I would classify Helix and all the many other boot discs as open-ended tools.

 2) Being able to peel off evidence via initial triage prior to the main examination will also serve to move the forensic examiner's starting point that much farther ahead. So much of what we do as forensic examiners in the initial steps of a forensic exam could be automated.

 Bottom line, if low hanging fruit are there to be had, why not create a process to auto-gather that based on the overall goals of the case? I think that is what your intent is with the scanner. (I stand to be corrected) I liken this to discussions people must have had way back about whether it's "worth it" to put the effort into adding power steering to a car.

 Anyway, just my 2c.

 @Clintonian on Twitter...
 Yes, the DestList is the thing that arranges the listed items in either an MRU or MFU format. That is why I told you and others on the win4n6 list to study the DestList. (I will check my slide deck, published through CTIN, and make sure that is clear--if not I will revise them.) While I have worked out the DestList internals, I used internal sources and have no public sources I can point you to. Unfortunately, I therefore cannot legally disclose the DestList portion of my Jump List slides.

 I will say that if you force changes in the listing order by clicking on different items in a Jump List, and then compare before and after versions of the DestList, some of the internals should become clear. You could also use process monitor to monitor Jump list activity, particularly file I/O at offset locations in the Jump List. I know this isn't pretty, but it is basically how I start.

 In addition to figuring out the DestList, it would be good for us forensics folk to begin aggregating applicationIDs, which identify the application that a particular Jump List pertains to. The applicationIDs are based on name, path and command arguments, so like prefetch hashes, the same application, in the same path, with the same arguments, will have the same applicationID on different systems. If we can develop a good reference of applicationIDs, investigators can quickly select the Jump Lists that are most interesting to their cases.

 Re forensic value:

 In law school, we were taught to think through legal issues in terms of elements--as in the elements of a contract are . . . In some sense, this form of breaking things down can work for forensics. It requires explicit definition of terms, and then thinking through the elements of that consitute sufficient evidence or facts to support an inference. In some instances, the work is partially done in that the elements can be defined in terms of corporate policy or law: e.g., the crime of possession of CP requires 1) CP, 2) computer user 3) who downloaded the CP, and so on. It may appear to be harder to do this for "intrusion" but think it through. I have, but I am too tired and old to remember right now.

 Timelines are one of the best tools for forensics examinations and can be quite useful in determining the forensic value of facts. I was trained to use them back in the bad old days of lawyering, and continued to use them in forensics. Lawyers--aka our clients--understand their value. So, in addition to being a good thinking tool, they are also a good way to give an attorney confidence in your work.
 Thanks, everyone, for your comments thus far...this has been somewhat illuminating. I feel another blog post coming on... ;-)

 I think that what your responses are illustrating to me is that value is both subjective , and relative . In my experience, I would think that both are predicated, in part, by the knowledge, experience, and training of the analyst.

 Timelines seem to be a very good illustrative point, so I'll continue using it. I've been creating timelines from multiple data sources for a number of years now, and found considerable value in doing so as part of an exam, when appropriate. I've spoken to others (and continue to do so) about creating timelines, including describing how they can be used to increase relative confidence in data , as well as provide increased context to the data. I've even demonstrated how I've used timelines to discern an issue when no other technique worked.

 However, there are still those who have yet to "drink the koolaid", as it were. Why is that? Is the value not recognized? Or is the bar for entry set too high?

 Okay, enough of that...back to the value of data . When it comes to timelines, IMHO, there are two basic camps...the "kitchen sink" camp, and the minimalist camp. I'm in the minimalist camp...my thought on timelines is to build them from specific data sources, applying overlays one layer at a time. I've built valuable timelines from just selected web server logs and file system metadata (SQLi attack). I've also developed "micro-timelines", using only selected data sources (or subsets thereof) in order to answer a particular question. For example, I once parsed all of the available logon events from the Security Event Log and produced a timeline from just those entries.

 The "kitchen sink" approach is to put everything available into the timeline, and allow the analyst to sift (no pun intended) that for the value . The reasoning for this approach has been that the value of specific data or data sources may not be known until that data is viewed in the context of other data (i.e., "I don't know what I need, so I want everything so I can decide...").

 My personal opinion (and this is just my opinion, I'm not trying to push this on anyone) is that this approach is somewhat cumbersome and requires a considerable amount of time and effort to sift through and determine the actual data of value .

 I hope this illustrates the point I'm trying to make, which is that the value of data , at this point, appears to be subjective. Given a CP case, for example, I think (and based on my experience) that once the requirements of the federal statute have been met, my focus would be determining who and when images were placed on the system (and how), when they were viewed, etc. And yet, I continue to meet LE analysts who appear to be caught off guard by the "Trojan Defense"...
 Regarding JumpLists, I haven't come across many Windows 7 systems yet as the corporate world is slow in upgrading. However, I've been experimenting with ProDiscover and noticed that it has the option to analyze JumpLists. Have you talked to Chris Brown about this?
 Phil,

 I'm a user of ProDiscover, and I have been since version 3.0. I have seen that recent updates to PD include the ability to parse jump lists...and as I'm sure you've seen, they do not parse the DestList stream.

 Thanks.
 Hi Harlan.

 The audience at OSDFC 2011 was definitely quiet but I wouldn't misinterpret that as them not seeing the value in what you presented. Your presentation was quite good and flowed with the standard high energy and passion that we expect and appreciate from you!

 I think that most people were listening, absorbing, and for me, waiting to head back to my computer to try some of this stuff out or write down some ideas. Some people just don't get it but for myself, I find trying it out, and seeing the practical application of the tool(s), to be of most value. I think your plugin system to extend the Forensic Scanner is great and whether others see the value in it or not, who cares? You have to scratch your own itch. If you see value in it then IMO it doesn't matter if others find it useful as well. If they do, great, if not, no big deal.

 In regards to the CCN track2 data that `bulk_extractor' extracts, we worked on this with Simson and it's relatively sound (as of bulk_extractor-1.0.0) in regards to the data it extracts. The track2 format is fairly specific which reduces the number of false positives. ie. credit_card_number==MMYY\d{3}security_data . For anyone doing economic crimes files, skimmers, and other credit card fraud, it has proven to be fairly valuable information. The track2 regex definitely needs more testing as I'm sure there is some fine tuning that can be done there to make it even better!

 Also in regards to `bulk_extractor' and how to find the actual filenames that an offset refers to. Simson includes an `identify_filenames.py' Python script in the 'python' directory of the latest `bulk_extractor'. This script can be ran against the `bulk_extractor' output, in combination with a `fiwalk' XML output file, and it will identify which file on the filesystem contains that offset. We are going to write a simple script which will identify the filenames without requiring the `fiwalk' XML output as this is a bit of a barrier right now (IMO) as many people don't have the `fiwalk' output. Once this is done I'll submit it to Simson and hopefully he'll roll it into his tarball.

 Keep up the good work!
 Phil,

 I try to send Chris Brown any new discoveries or research that I want to see incorporated into tools. He has always been very good about building capability to handle new Windows artifacts. Check out what he has done with shadow copies, for example.

 More on forensic value:

 Philosophically and practically, the foundation of forensic value starts with the concepts embedded in our use of "known good" file hashes.

 Stepping up, I have two big buckets for thinking about cases--they are usually more or less content focused (e.g., eDiscovery) or more or less activity related (determining who did what or what has happened).

 I would posit that forensics value is not so much subjective, but rather context driven. The questions that need to be answered determine the facts that are relevant.

 Jump Lists, for example, might not be important in investigating something like stuxnet. They could be very relevant in an IP theft case or a CP case. Moreover, the DestList would be relevant for determining the frequency or recency of file usage.
 I would posit that forensics value is not so much subjective, but rather context driven. The questions that need to be answered determine the facts that are relevant.

 I would further suggest that the value of data is then subject to the training, knowledge and experience of the examiner, and is again, subjective.

 Given two arbitrary analysts, with the same case, data, and goals, you're going to get two different results.
 bulk_extractor comes with a script called identify_filenames.py which provides the file name for every discovered feature.
 bulk_extractor comes with a script called identify_filenames.py which tells you the files that the features were found in.
 another blogger with spelling problems,

 Any thoughts on the content?

 Thanks.
 Another thought on data value being subjective...

 Consider an exam involving possible malware (I'm using this as an example as I get a lot of these...)...when performing these exams I have a checklist of things I run through. I've talked to others in the past about NTFS alternate data streams (ADSs)...when I've presented on this topic, I get a lot of blank stares, but on a recent exam, I did find an ADS that, of the 60 ADSs on the system, this one wasn't part of normal user activity. It turned up in the timeline during the same time as artifacts for a possible keylogger installation.

 Another thing I check for during these exams, particularly the ones where the alleged malware isn't known, is MBR infectors. I have a script I run against the image file and quickly lets me see enough information to determine the likelihood of an MBR infector being installed. I've talked to other analysts about this, and their candid answers have been, no, they don't check for this.

 My point is that the value of data is subjective...not just in the context of the case or exam, but also in the context of the analyst or examiner. I often find that the absence of an artifact where one is expected is in itself an artifact , that NOT finding something where I would expect it is as or more valuable than finding it.
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
 ► September (15)
 ► August (11)
 ► July (8)
 ▼ June (10) Meetup, Tools and other stuff
 Links and Updates
 Defining "Forensic Value"
 Awards
 Links and Updates
 Thoughts on IR
 OSDFC Follow-up
 Updates, Links, Etc.
 Updates
 Updates

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
