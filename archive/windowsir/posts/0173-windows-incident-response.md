# Windows Incident Response

- URL: https://windowsir.blogspot.com/2021/08/tips-for-dfir-analysts.html
- Published: unknown
- Updated: unknown
- Labels: none

Windows Incident Response
 The Windows Incident Response Blog is dedicated to the myriad information surrounding and inherent to the topics of IR and digital analysis of Windows systems. This blog provides information in support of my books; "Windows Forensic Analysis" (1st thru 4th editions), "Windows Registry Forensics",
as well as the book I co-authored with Cory Altheide, "Digital Forensics with Open Source Tools".
 Thursday, August 26, 2021
 Tips for DFIR Analysts
 Over the years as a DFIR analyst...first doing digital forensics analysis, and then incorporating that analysis as a component of IR activity...there have been some stunningly simple truths that I've learned, truths that I thought I'd share. Many of these "tips" are truisms that I've seen time and time again, and recognized that they made much more sense and had more value when they were "named".
 Tips, Thought, and Stuff to Think About

 When performing DF analysis, the goal is to be as comprehensive and thorough as possible. A great way to achieve this is through automation. For example, I developed RegRipper because I found that I was doing the same things over and over again, and I wanted a way to make my job easier. The RegRipper framework allowed me to add checks and queries without having to write (or rewrite) entirely new tools every time, as well as provided a framework for easy sharing between analysts.
 TCP networking is a three-stage handshake, UDP is "fire and forget". This one tip helped me a great deal during my early days of DFIR consulting, particularly when having discussions with admins regarding things like firewalls and switches.
 Guessing is lazy. Recognize when you're doing it before someone else does. If there is a gap in data or logs, say so. At some point, someone is going to see your notes or report, and see beyond the veil of emphatic statements, and realize that there are gaping holes in analysis that were spackled over with a thin layer of assumption and guesswork. As such, if you don't have a data source...if firewall logs were not available, or Windows Event Logs were disabled, say so.
 The corollary to the tip on "guessing" is that nothing works better than a demonstration. Years ago, I was doing an assessment of a law enforcement headquarters office, and I was getting ready to collect password hashes from the domain server using l0phtcrack. The admin said that the systems were locked down and there was no way I was going to get the password hashes. I pressed the Enter key down, and had the hashes almost before the Enter key returned to its original position. The point is, rather than saying that a threat actor could have done something, a demonstration can drive the point home much quicker.
 Never guess at the intentions of a threat actor. Someone raised in the American public school system, with or without military or law enforcement experience, is never going to be able determine the mindset of someone who grew up in the cities of Russia, China, etc. That is, not without considerable training and experience, which many of us simply do not have. It's easy to recognize when someone's guessing the threat actor's intention, because they'll start off a statement with, "...if I were the threat actor...".
 If no one is watching, there is no need for stealth. The lack of stealth does not bely sophistication. I was in a room with other analysts discussing the breach with the customer when one analyst described what we'd determined through forensic analysis as, "...not terribly sophisticated...", in part because the activity wasn't very well hidden, nor did the attacker cover their tracks. I had to later remind the analyst that we had been called in a full 8 months after the threat actor's most recent activity.
 The adversary has their own version of David Bianco's "Pyramid of Pain" , and they're much better at using it. David's pyramid provides a framework for understanding what we (the good guys) can do to impact and "bring pain" to the threat actor. It's clear from engaging in hundreds of breaches, either directly or indirectly, that the bad guys have a similar pyramid of their own, and that they're much better at using theirs.
 We're not always right, or correct. It's just a simple fact. This is also true of "big names", ones we imagine are backed by significant resources (spell checkers, copy editors, etc.), and as such, we assume are correct and accurate. As such, we shouldn't blindly accept what others say in open reporting, not without checking and critical thinking.
 There are a lot of assumptions in this industry. I'm sure it's the same in other industries, but I can't speak to those industries. I've seen more than a few assumptions regarding royalties for published books; new authors stating out big publishers may start out at 8%, or less. And that's just for paper copies (not electronic), and only for English language editions. I had a discussion once with a big name in the DFIR community who assumed that because I worked for a big name company, of course I had access to commercial forensic suites; they'd assumed that my commenting on not having access to such suites was a load of crap. When I asked what made them think that I would have access to these expensive tool sets, they ultimately said that yes, they'd assumed that I would.
 If you're new to DFIR, or if you've been around for a while, you've probably found interviewing for a job to be nerve-racking, anxiety-producing affairs. One thing to keep in mind is that most of the folks you're interviewing with aren't terribly good at it, and are probably just as nervous as you. Think about it...how many times have you seen courses offered in how to conduct a job interview, from the perspective of the interviewer?
 2 comments:
 Thanks so much as always for your thoughts and sharing.

 I think the rating that an attack was "sophisticated" says more about the person than about the actual attack, because there is no common interpretation what sophistication means or is, but more of a relative term compared to the speaker's knowledge and experience. Companies declaring an attack as sophisticated try to improve their light on themselves. Getting familiar with attack frameworks, detection frameworks, DFIR reports, Harlan's writeups, ... will reduce the use of the word sophistication I guess :)

 Guessing or overseeing gaps in analysis is difficult to spot, because we get blind during a case. Therefore, it's important that multiple analysts work on cases to identify those guessing and speculating.

 Regarding catalog of e.g "...we don't do this here.." : These catalogs varies inside large infrastructures between different platforms or organisational units. Clustering and applying detections to those clusters help but is a huge effort to maintain and build. Often we apply detections to all of the endpoints, servers, ... and aren't able to apply more specific needs to special environments. Do you have found such clustering too?

 "Never guess at the intentions of a threat actor. " : Unfortunately, it is exactly asked over and over when discussing cases. In the same sense, articles on Red Team Journal were also talking about this in regards to red teaming. Red teams faces a similar issue when building and executing cases, e.g. The Need for Genuine Empathy in Modern Adversarial Red Teaming . Only some posts are still available. It's difficult to play the role of the Gegenspieler when having totally different physical environments and time constraints, framed views, ... Instead of guessing the intention, we can enumarate different worst-case outcomes from the point where an attacker was without speculating and prooving these "hypothesis". Distinguishing if an attack is a ransomware attack vs a stealthy APT with spionage intentions based on a detection is difficult, nevertheless important, because the playbook differs... but to get that intention the attack must already be executed.
 Andreas,

 Thanks for leaving a comment!

 > ...more of a relative term compared to the speaker's knowledge and experience...

 Agreed, to some extent. In my experience, it's more often a statement to hide behind. I say this as the one who has gone to the CIO or CTO and had to say, "...this Windows 7 system was running an unauthorized RDP server, installed against policy, and the admin password was, in fact, "password"...".

 > ...catalogs varies inside large infrastructures...

 Yes, of course...but this not a reason for not having them. The example I gave was based on policy, as well as an operational business decision. It's up to organizations to make their own similar decisions.

 > Unfortunately, it is exactly asked over and over when discussing cases.

 Oh, yes...agreed. But that doesn't mean that it has to be answered, particularly not in the way that it is often answered, which is very often when gaps are not acknowledged.

 Consider this post:
 https://www.secureworks.com/blog/ransomware-as-a-distraction

 In the above case, my team was "second chair" to the primary response team, and were given this task in an almost, "well, crap, we hired you so we might as well give you something to do..." manner. I fully acknowledged that we did not have full visibility into the overall engagement, so we were only going to comment on what we could see.

 Again, thanks for reading, and thanks for commenting...
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

 ▼ 2021 (26) ► December (3)
 ► November (3)
 ► October (3)
 ► September (5)
 ▼ August (2) Building a Career in CyberSecurity
 Tips for DFIR Analysts

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
