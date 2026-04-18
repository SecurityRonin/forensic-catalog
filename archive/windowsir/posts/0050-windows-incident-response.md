# Windows Incident Response

- URL: https://windowsir.blogspot.com/2009/05/e-evidence-updates.html
- Published: unknown
- Updated: unknown
- Labels: none

Windows Incident Response
 The Windows Incident Response Blog is dedicated to the myriad information surrounding and inherent to the topics of IR and digital analysis of Windows systems. This blog provides information in support of my books; "Windows Forensic Analysis" (1st thru 4th editions), "Windows Registry Forensics",
as well as the book I co-authored with Cory Altheide, "Digital Forensics with Open Source Tools".
 Saturday, May 02, 2009
 e-Evidence updates

 4 comments:
 Great roundup post!

 I particularly enjoyed the references to virtulization. Use of "LiveCD" OS enviroments that boot (almost) entirely removed from the hardware and run in system memory are very common and becoming slightly more mainstream.

 And while a bit different than Moka5 and MojoPac, certainly would present challenges for an investigator.

 Microsoft's Virtual PC virtualization platform is also popular in offices and homes.

 I'm particularly interested in it as Windows 7 pushes forward. Windows 7 Ultimate and Enterprise builds will offer an "embedded" virtualization of XP: Secret No More: Revealing Windows XP Mode for Windows 7 . - Within Windows blog

 Localized usage (as opposed to server-based) of most of these virtual systems would require a virtual hard-drive file somewhere; either on the local drive or removable media. As long as you can identify and recover that file, I suppose you still can apply traditional forensics methods on that "hard-drive".

 What could also be worthy of investigation on these "virtual hard-drive" files are file-fragment records from the host system that get "trapped" in the free-space of the virtual drive.

 Why does it take so long to create a fixed size virtual hard disk? - Virtual PC Guy WebLog

 Quoting below:

 "Imagine the following situation:

 * You have a virtual machine with a bunch of confidential data running on a central server (e.g. your company payroll).
 * This virtual machine gets moved to a new physical server in response to increased work load.
 * You create a new virtual machine which is given to someone on from the in-house dev team - but the virtual hard disk data was not zeroed out.
 * Developer then runs data recovery tools on his new, blank virtual machine and is able to recover data from the old payroll server (yikes!)

 You see - data is never actually deleted from a disk when a file is moved or deleted (it is just dereferenced) so to avoid the above scenario - we must take the time to "do the right thing" and zero out the VHD contents."

 End quote.

 So non-technical users (or lazy technical ones) who quickly create virtual hard drive files might create security issues or leave breadcrumbs for a skilled forensic investigator in that virtual file environment.

 For suspects who do their homework and use non-footprint leaving methods of their activity with virtual systems, Diane Barrett's presentation hit it on the mark that investigators may have to take some methods from security incident responders by moving on to corporate/home router/IP log file records to recreate Web activity.

 On that, while many folks might already know about potential IP/Web activity logging in a corporate setting, it seems to me that forensic investigators might do well to examine any home routers used by suspects as well. Depending on the specific model and the suspect's technical knowledge, some SOHO/home routers have the ability to retain logs as well. Those might also contain evidence information.

 Finally, as Diane Barrett (and you) point out out, (most) of these virtualization applications/environments do leave some evidence of their own presence behind. So even if the virtual environment itself disappears, the "wrapper" that ran it might be left behind to some degree. Familiarity of these markers might lead the examiner to then look deeper or elsewhere (server file, USB stick-drive, portable hard-drive) for the actual "hard-drive" file and all the information it contains.

 Great find!

 --Claus V.
 Thanks for the shout out!
 Another great post!

 To add to the comment from Claus. From a security standpoint, has there been any discussion about XPM and patching the virtualized OS? We see plenty of users who don't bother to patch their systems. Would this be a double whammy by basically having 2 (possibly unpatched) OS's to exploit?

 This might be quick on the trigger as I have not looked much into XPM, but would be curious how keeping a user keeps it up to date.

 Thanks again for a great blog,
 Ed
 @ Ed - That's getting a bit off the forensics angle but that is an excellent point that hasn't been deeply discussed regarding the XPM environment.

 I've not had a chance to play with it yet but it seems to basically be just another virtualized XP build OS that is interacting with the Windows 7 host OS a bit more "optimized" for application display purposes for the end-user. It has some more tricks up the sleeve but I'm also concerned with security patching methods for the XPM enviroment.

 Also, Windows 7 appears to also still have the standard "Application Compatibility Mode" that XP and Vista have to run applications optimized on the host OS as XP/Win98/etc.

 This is different than XPM which fully virtualizes the XP OS for the application

 Application Compatibility Mode generally works well in Vista for most otherwise balky XP applications. So I'd initially assume that it would be the same under Windows 7.

 With this in mind and the hardware impact of XPM as a client verses running XP apps on the host OS directly in the still available "application compatibility mode" I'm starting to wonder what class of applications that XPM mode is really needed for.

 Plus all the administrative sysadmin overhead for deploying, configuring and maintaining XPM support on these client systems.

 What started out sounding like a slam-dunk for Microsoft with this feature is now bring with it lots of (yet) unanswered questions on practicallity.

 But then, this is a forensics blog and not a sysadmin blog. ;)

 --Claus V.
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

 ▼ 2009 (166) ► December (15)
 ► November (14)
 ► October (10)
 ► September (9)
 ► August (13)
 ► July (12)
 ► June (13)
 ▼ May (12) Stuff
 More giggity, news, links and stuff
 Giggity giggity
 SANS WebCast today at 1pm, EST
 Links and Stuff
 Excellent Sunday Linkage
 PaulDotCom and TalkForensics Interviews
 Definitions
 Is this stuff really useful??
 WFA 2/e...proposed cover art
 e-Evidence updates
 SearchSecurity: Matt Shannon

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
