# Windows Incident Response

- URL: https://windowsir.blogspot.com/2008/12/honor-thy-settings.html
- Published: unknown
- Updated: unknown
- Labels: none

Windows Incident Response
 The Windows Incident Response Blog is dedicated to the myriad information surrounding and inherent to the topics of IR and digital analysis of Windows systems. This blog provides information in support of my books; "Windows Forensic Analysis" (1st thru 4th editions), "Windows Registry Forensics",
as well as the book I co-authored with Cory Altheide, "Digital Forensics with Open Source Tools".
 Friday, December 05, 2008
 Honor Thy Settings

 6 comments:
 I've been dealing with incidents involving USB malware for over two months straight now. 10% of everything, if not more now is capable of this method of spreading. Autorun needs to be disabled in every system. I discussed this in a few posts for the responder, but it applies to the end user/sysadmin as well.
 Like a lot of other things, this appears to be something where MS has had the information posted, and then during some kind of investigation, someone found out that the settings were not working. At that point, most likely due to the visibility of the 'victim', MS was engaged, and now there's a fix to the fix, one where you need to (a) install an update, and then (b) create and set a Registry value.
 Harlan,

 From a sysadmin perspective I'm 100% on board.

 However, from a "consumer/dad" perspective I'm I bit more frustrated.

 Daughter unit needed a USB stick to take to school to save work from a computer-lab if her assignment work wasn't completed. Asked a few days in advance and promptly forgot. Got in the car a few days later and remembered and asked her about it. She said she had the forethought to grab one of our old/small USB sticks (32MB?) and had it with her.

 I had to confiscate it with regret.

 1) I didn't know what of our data was still on it and needed to "audit" it and remove anything of importance in case of loss/theft at the school.

 2) I needed to make sure it was "clean" of any thing that might get her into trouble at school for "posessing" (forbidden utilities perhaps such as pentesting tools and other PUPS, etc.).

 3) I have NO idea the condition of the lab-pc's she will be using at school. Don't know how their IT department maintains them, what AV/AM software is used, how often they are scanned/checked for rootkits and other baddies, etc. So cross-infection of our systems could be a real possiblity.

 4) Need to figure out a "reasonable" way for daughter-unit to use a USB drive between school/friends houses/systems and our own but that will minimize chance of infecting our own. Going to have to spend time looking at my new AV/AM software to check out automatic detection and scanning/access settings for removable (USB) devices.

 (Sigh)

 It's hard being an IT dude AND a dad these days. Oh to be blissfully unaware....

 Would it be appropriate and reasonable to visit the school one day to request information on their IT policy and audit/security procedures? Or would that just freak them out as some kind of pen-test attack?

 I'm curious how many families even think about these things as a threat risk. I know I certainly do....
 Claus,

 4) Need to figure out a "reasonable" way for daughter-unit to use a USB drive between school/friends houses/systems and our own but that will minimize chance of infecting our own.

 Read the blog post. No automagical execution of autorun.inf files once set...you can even set the specific drive types to which it applies. Set in the HKLM hive. AV is secondary solution.

 What I'd love to do is be able to set admin-defined actions to occur based on an event on the system, much like WFP, rather than as a scheduled task...
 There are ways to mitigate risk on a usb stick.

 1) Buy one with a write block switch. Kanguru sells these.

 2) Create a directory(yes a directory) named Autorun.inf. This is known to help mitigate the ability of the malware to write to the drive.

 3) Disable Autorun using group policy on your computers and force the following registry change:
 REGEDIT4
 [HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows NT\CurrentVersion\IniFileMapping\Autorun.inf]
 @="@SYS:DoesNotExist"

 These steps work.

 And it is completely reasonable for you to question the schools.
 Harlan - Yep. Already had planned on doing that (MS fix) on all our systems over the weekend. Thank you!

 Hogfy - Wonderful tips! I use a Kanguru device and use the switch often. I will have to look specifically for that feature when I pick up a USB drive for her and show her how/when to use it.

 Also like the suggestion on making the renamed folder. Clever!

 Thanks gentlemen! And great/timely post!

 --Cheers!
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

 ▼ 2008 (108) ▼ December (9) Flippin' Sweet!!
 Using RegRipper
 Some Good Reading...
 Perl Module Updated!
 RegRipper
 Less blogging, more writing
 Windows Hibernation files
 Issues with AV
 Honor Thy Settings

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
