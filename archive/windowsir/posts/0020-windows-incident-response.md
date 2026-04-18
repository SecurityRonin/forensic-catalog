# Windows Incident Response

- URL: https://windowsir.blogspot.com/2007/03/mounting-dd-image.html
- Published: unknown
- Updated: unknown
- Labels: none

Windows Incident Response
 The Windows Incident Response Blog is dedicated to the myriad information surrounding and inherent to the topics of IR and digital analysis of Windows systems. This blog provides information in support of my books; "Windows Forensic Analysis" (1st thru 4th editions), "Windows Registry Forensics",
as well as the book I co-authored with Cory Altheide, "Digital Forensics with Open Source Tools".
 Wednesday, March 28, 2007
 Mounting a DD image

 10 comments:
 Harlan,
 I'm glad to see this has come together in to something that looks very very useful.
 Waow, now windows user can do the same as Linux investigators but in a more complicated manner and having a lot of software to install.

 Hey, wake up, Linux can do that for years, for free and with everything bundled in the OS !

 Even better : with no OS crash !
 Thanks for the input, but sometimes "use Linux" just isn't the answer.
 BTW - In order to use Live View, one needs to have VMWare Workstation, not just the player.
 It actually works just fine with the free vmware server.
 BTW - In order to use Live View, one needs to have VMWare Workstation, not just the player.

 I've seen others do this with just the VMPlayer.

 I have an opportunity to try this very soon, so I'll let you know how it works.
 Hallo,
 I am the author of the everyday increasingly outdated small pseudo-GUI for VDK.EXE you can find here:
 http://home.graffiti.net/jaclaz:graffiti.net/Projects/VDM/vdm.html

 I just want to let you know that a handy way to mount "dd-like" images is to use VMWare 2.00 .pln descriptor files, see this:
 http://www.msfn.org/board/index.php?showtopic=80281&st=1
 This way the "dd-like" image can be mounted with the "correct" geometry, as VDK defaults normally to a 64/32 one.

 To automatically create a .pln file descriptor for the image you have, you are free to "borrow" code from this other small batch of mines (MBRbatch/Mkimg):
 http://www.boot-land.net/forums/MBRBatch-001-ALPHA-t3191.html

 Moreover, there is a new Filesystem driver, IMDISK:
 http://www.ltr-data.se/opencode.html
 http://www.boot-land.net/forums/ImDisk-f59.html
 (you will need to specify a "hidden sectors" offset to mount "full" HD images)

 Here is a thread where I try to collect all links I can find to Ramdisk/Filedisk drivers:
 http://www.boot-land.net/forums/index.php?showtopic=1507

 Finally, there are a number of absolutely FREEWARE "dd-like" tools for Windows, I use a lot dsfo/dsfi from the DSFOK toolkit:
 http://members.ozemail.com.au/~nulifetv/freezip/freeware/

 But in this thread there are a few other options:
 http://www.911cd.net/forums//index.php?showtopic=16534

 jaclaz
 jaclaz,

 Thanks for the comment!

 Do you have a solid, end-to-end, complete description of how to go from a dd image to a live file system?

 Thanks!
 @Keydet89

 I am not sure I understand correctly what you mean by "live" filesystem.
 If you mean a filesystem mounted and accessible as a drive letter under 2K/Xp, all you have to do is to mount the image with either VDK or IMDISK.
 Unless you need to perform "filesystem level" chores (such as formatting the image) the given geometry won't affect operation in any way (2K and XP use LBA to access sectors).
 Or you mean something else?

 jaclaz
 jaclaz,

 If you mean a filesystem mounted and accessible as a drive letter under 2K/Xp, all you have to do is to mount the image with either VDK or IMDISK.

 That's exactly what I was asking...thanks!
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

 ▼ 2007 (83) ► December (6)
 ► November (7)
 ► October (1)
 ► September (3)
 ► August (4)
 ► July (8)
 ► June (10)
 ► May (12)
 ► April (7)
 ▼ March (11) Book updates - NukeOnDelete and SysProt AntiRootkit
 "Windows Forensic Analysis" is now available!
 Teaching IR/CF
 Change Analysis Diagnostic Tool for Windows XP
 Why current IR models don't work, part deux
 Mounting a DD image
 Why current IR models don't work
 Thoughts on Incident Response/Management
 Forensic Challenges
 Getting service information during IR
 Book & Attending BlackHat DC

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
