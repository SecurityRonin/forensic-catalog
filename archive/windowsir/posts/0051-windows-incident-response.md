# Windows Incident Response

- URL: https://windowsir.blogspot.com/2009/10/free-tools.html
- Published: unknown
- Updated: unknown
- Labels: none

Windows Incident Response
 The Windows Incident Response Blog is dedicated to the myriad information surrounding and inherent to the topics of IR and digital analysis of Windows systems. This blog provides information in support of my books; "Windows Forensic Analysis" (1st thru 4th editions), "Windows Registry Forensics",
as well as the book I co-authored with Cory Altheide, "Digital Forensics with Open Source Tools".
 Friday, October 23, 2009
 Free Tools

 21 comments:
 Great list of tools. I am just starting out in Computer Forensics and this is a big help. Thanks for posting this Harlan.

 Joe
 Good list Harlan. If you are looking for a pre-built environment with these tools ready to go out of the box.

 Majority of these tools built into the SANS Investigative Forensic Toolkit: https://computer-forensics2.sans.org/community/downloads/

 # ssdeep & md5deep (Hashing Tools)
 # Foremost/Scalpel (File Carving)
 # WireShark (Network Forensics)
 # HexEditor
 # Vinetto (thumbs.db examination)
 # Pasco (IE Web History examination)
 # Rifiuti (Recycle Bin examination)
 # Volatility Framework (Memory Analysis)
 # DFLabs PTK (GUI Front-End for Sleuthkit)
 # Autopsy (GUI Front-End for Sleuthkit)
 # The Sleuth Kit (File system Analysis Tools)
 #cregripper.pl and plugins Registry Forensic Carver
 # regslack.pl Registry slack
 # deleted.pl Registry deleted key examination
 # regtime.pl Registry timelime creator – now with sleuthkit bodyfile output
 #nwindata.pl Windows Time
 # Mandiant Auditviewer AuditViewer to parse and examine memory via GUI
 Awesome list Harlan, thanks for posting it.

 I didn't even know that ProDiscover had a free version! I'm grabbing that one now to look at it.

 Tom
 Another wonderful timeline tool Log2Timeline can be found at http://log2timeline.net

 --Rob
 Some others I thought of:

 dcfldd - Imaging with hashing

 LiveView - Convert DD images to VMWare bootable images

 VMWare Player - Free but I think you need a VMWare account

 Forensic CaseNotes - For keeping your case notes - hashes entries as you enter them

 DCode - Nice little utility for converting Data to Date/Time values.

 MFT Ripper - Dumps MFT file to a CSV file. You have to email the author for this, but he has a free version

 IfranView - Free Image Viewer/Converter

 Ok that's enough for now...I'm sure I have more at home. :)

 Tom
 FoprensicBox for MSN and Windows Live.

 Advanced Prefetch Analyzer (an update is in the works). Also Didier's tool.

 SQLite Spy - handy for FireFox 3

 Evidence Mover from Microforensics.

 ExtractNow for extracting files from multiple archives at once.

 SMPlayer for troublesome videos.

 ClamWinPortable and SysClean for AV scanning from a thumb.

 VSS from Dan Mares for mounting shadow volumes.

 There are a lot of good tools that are not free, but not expensive, either. The list could grow if you put a <$100 or even <$50 limit on cost. BTW, IrfanView is not free for use in a commercial or government environment, but I think that a license was around $10.00.
 Jimmy,

 Any chance of getting links to any of those tools?

 Thanks!
 Don't forget the Princeton tools:

 http://citp.princeton.edu/memory/code/
 Thanks Harlan. This has always been one of my peeves about the lists as well.
 Raptor - bootable Linux CD that can be used for imaging (this will likely open up a whole flurry of similar emails, so let's just use this one as a placeholder for all bootable Linux CDs...)

 Raptor in not a forensically sound distro - it recovers file systems during boot sequence, activates swap partitions and does not provide a way to mount a file system in real read-only manner.
 Regarding Raptor:

 It does provide a way to mount a file system in read-only.

 But it doesn't include the HPA when imaging into raw (dcfldd)
 Regarding the Anonymous comment about Raptor. Please test or research a tool before making any comments.

 Raptor is forensically sound. At no time does it mount or touch any attached devices without the approval of the user.

 If anyone has any questions or concerns feel free to contact me directly - swhalen@forwarddiscovery.com.
 Don't forget PsTools
 http://technet.microsoft.com/en-us/sysinternals/bb896649.aspx

 joeware
 http://www.joeware.net/index2.htm
 good domain tools
 Here are links to most of the tols that I mentioned:

 Advanced Prefetch Analyzer by Allan Hay (an update is in the works).
 UserAssist by Didier Stevens (http://blog.didierstevens.com/programs/userassist/)
 SQLite Spy - handy for FireFox 3 (http://www.yunqa.de/delphi/doku.php/products/sqlitespy/index)
 Evidence Mover from Microforensics. (http://www.microforensics.com/pages/downloads.php) Site is down at the moment.
 ExtractNow for extracting files from multiple archives at once. (http://www.extractnow.com/)
 SMPlayer for troublesome videos.(http://smplayer.sourceforge.net/downloads.php?tr_lang=en)
 ClamWinPortable for AV scanning from a thumb.(http://portableapps.com/apps/utilities/clamwin_portable)
 SysClean for AV scanning from a thumb (http://www.trendmicro.com/download/sysclean.asp)
 VSS from Dan Mares for mounting shadow volumes. (http://www.dmares.com/index.htm)
 Great list of software ! Thanks for taking the time to post this valuable information !
 ReviveIt: A tool for carving data from NTFS-compressed images at:

 http://sourceforge.net/projects/revit/

 Caine (Computer Aided INvestigative Environment) at:

 http://www.caine-live.net/

 Not, strictly, a tool but a collection of images used to support eDiscovery training and research (Digital Corpora) at:

 http://digitalcorpora.org/
 Sean,

 Thanks. I'm not as familiar with Caine as you are...how are you using it in the forensic examination of Windows systems?

 Thanks!
 Hi Harlan:

 I use Caine pretty much as I used to use Helix before it became a commercial product. I like the fact that it is simple to build a bootable USB version which I will sometimes use for triage purposes especially when the discovery order precludes me from making complete forensic images.

 The problem is often the judge's limited understanding of the digital forensic process and an increased emphasis on privacy precluding what are perceived as "fishing expeditions".

 With Caine I can often gather enough evidence to warrant a more complete examination, or eliminate the need to do so.

 There are other Linux distros offering a similar set of tools, and I'm not crazy about Ubuntu, which is not my favorite Linux distribution, but for ease of use, Caine is one of the best.

 Sean
 *Very* useful list; thanks for putting it together.

 Minor error: the link for dcfldd links back to this blog post; target should be http://dcfldd.sourceforge.net/
 Here's my list of tools:

 Chaosreader: http://chaosreader.sourceforge.net/
 Eindeutig: http://sourceforge.net/projects/fast/files/Eindeutig/
 FileAlyzer: http://www.safer-networking.org/en/filealyzer/index.html
 Galleta: http://sourceforge.net/projects/fast/files/Galleta/
 OfficeMalScanner: http://www.reconstructer.org/code/OfficeMalScanner.zip
 RunAlyzer: http://www.safer-networking.org/en/runalyzer/index.html
 Stegdetect: http://www.outguess.org/download.php
 TrID: http://mark0.net/soft-trid-e.html
 Windows Forensic Toolchest (WFT): http://www.foolmoon.net/security/wft/
 chkrootkit: http://www.chkrootkit.org/
 dumphive: (no URL found)
 pdftk: http://www.accesspdf.com/pdftk/

 And here's a list of very useful misc sites:

 Wepawet: http://wepawet.cs.ucsb.edu/
 jsunpack: http://jsunpack.jeek.org/dec/go
 Anubis: http://anubis.iseclab.org/

 Cheers, Stefan.
 Great list of tools from everyone!

 Baremetal Software http://baremetalsoft.com/

 Produces:
 baretail and baregrep

 Both great tools

 baretail is a great version of tail for windows systems

 Dave
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
 ▼ October (10) Linkware
 File Extensions and Programs
 Free Tools
 Windows 7 and the Future of Forensic Analysis
 Timeline Creation Tools
 Challenges
 Book news and Registry research
 DCC2009 Takeaways
 Links
 Hakin9 articles

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
