# Windows Incident Response

- URL: https://windowsir.blogspot.com/2006/09/metadata-and-ediscovery.html
- Published: unknown
- Updated: unknown
- Labels: none

Windows Incident Response
 The Windows Incident Response Blog is dedicated to the myriad information surrounding and inherent to the topics of IR and digital analysis of Windows systems. This blog provides information in support of my books; "Windows Forensic Analysis" (1st thru 4th editions), "Windows Registry Forensics",
as well as the book I co-authored with Cory Altheide, "Digital Forensics with Open Source Tools".
 Monday, September 25, 2006
 MetaData and eDiscovery

 9 comments:
 I posted a HOWTO for anyone who wants to run your tool on Linux and does not know how to prepare the Perl side of things. You can find it at:

 http://chicago-ediscovery.com/computer-forensic-howtos/howto-extract-metadata-microsoft-word-linux.html

 Andrew Hoog
 Chicago Electronic Discovery
 http://chicago-ediscovery.com
 hi, its impressive. If i want to use your perl file as is and use it in my own metadata extraction tool written in PHP, will you mind? What might be the restrictions of it? Here is the usage scenario:
 1. I have a user who will upload a file in .doc extension
 2. I will use PHP to call your perl file to extract the metadata for me
 3. I will receive it and use it for subsequent part of my system.

 Can you please be generous on this issue? I want to thank you in advance.
 Please, feel free...thanks!
 Hi Harlan,

 I cannot use wmd.pl in case of docx files. Its giving me the following error:

 "C:\Perl\site\lib>wmd.pl f:\hi.docx
 Use of assignment to $[ is deprecated at C:/Perl/site/lib/OLE/Storage.pm line 61
 .
 Use of assignment to $[ is deprecated at C:/Perl/site/lib/OLE/PropertySet.pm lin
 e 409.
 --------------------
 Statistics
 --------------------
 File = f:\hi.docx
 Size = 50501 bytes
 Magic = 0x0 ()
 Version = 0
 LangID = Unknown

 Document was created on Windows.

 Magic Created :
 Magic Revised :

 Can't call method "directory" without a package or object reference at C:\Perl\s
 ite\lib\wmd.pl line 59."

 Please help
 Sunny,

 wmd.pl isn't intended for .docx files...it was written for .doc files. The Office 2007+ file formats are different and as such require the use of different tools.
 Hello Mr. Carvey. My computer forensics class (MSIA program, 600-level course, will remain anonymous for now) is using one of your textbooks along with some of your Perl scripts. Thank you very much for the easy-to-follow, chock-full-of-information resources! Anyways, I have a lingering question that I thought I would bring to the man himself. In fact, I am going to paste the exactly question I posted in our class forum:

 Prof./Class,

 Does anyone know exactly what the Trash Bin metadata is in a Word Document? I searched for about an hour on the topic just now. I found articles on Word metadata, articles on the severity of security leaks caused by such metadata, etc. I then looked up the Perl modules that were used to pull the trash information. However, the closest I came to finding what the trash bins really are is the following article: http://windowsir.blogspot.com/2006/09/metadata-and-ediscovery.html

 The article is from Carvey's blog. He notes:

 "The module extracts the information, it just needs to be prettied up a bit. Another benefit of the module is that it extracts additional information from the OLE contents of the file. First off, it extracts information about the OLE "trash bins", where useful data could be hidden:

 Trash Bin Size
 BigBlocks 0
 SystemSpace 940
 SmallBlocks 0
 FileEndSpace 1450"

 OK.... so what does that mean? What exactly are the four items noted and how does one go about trying to extract data stored in those "trash bins?" I also found http://www.cpan.org/authors/id/H/HC/HCARVEY/File-MSWord-0.1.readme, where Carvey discusses trash bins. He notes:

 "%hash = $word->readTrash()

 Reads the trash bins in an OLE/compound/structured storage document.
 Returns a hash of hashes with the names of the trash bins as keys, and
 the size and contents of the bins as subkeys."

 Again, this does not really tell me what I would like to know about the trash bins. I would really like to know what those buggers are, so any feedback is appreciated. Thanks prof/all!

 Can you please shed more light on the trash bins for me?
 Hello Mr. Carvey. My computer forensics class (MSIA program, 600-level course, will remain anonymous for now) is using one of your textbooks along with some of your Perl scripts. Thank you very much for the easy-to-follow, chock-full-of-information resources! Anyways, I have a lingering question that I thought I would bring to the man himself. In fact, I am going to paste the exactly question I posted in our class forum:

 Prof./Class,

 Does anyone know exactly what the Trash Bin metadata is in a Word Document? I searched for about an hour on the topic just now. I found articles on Word metadata, articles on the severity of security leaks caused by such metadata, etc. I then looked up the Perl modules that were used to pull the trash information. However, the closest I came to finding what the trash bins really are is the following article: http://windowsir.blogspot.com/2006/09/metadata-and-ediscovery.html

 The article is from Carvey's blog. He notes:

 "The module extracts the information, it just needs to be prettied up a bit. Another benefit of the module is that it extracts additional information from the OLE contents of the file. First off, it extracts information about the OLE "trash bins", where useful data could be hidden:

 Trash Bin Size
 BigBlocks 0
 SystemSpace 940
 SmallBlocks 0
 FileEndSpace 1450"

 OK.... so what does that mean? What exactly are the four items noted and how does one go about trying to extract data stored in those "trash bins?" I also found http://www.cpan.org/authors/id/H/HC/HCARVEY/File-MSWord-0.1.readme, where Carvey discusses trash bins. He notes:

 "%hash = $word->readTrash()

 Reads the trash bins in an OLE/compound/structured storage document.
 Returns a hash of hashes with the names of the trash bins as keys, and
 the size and contents of the bins as subkeys."

 Again, this does not really tell me what I would like to know about the trash bins. I would really like to know what those buggers are, so any feedback is appreciated. Thanks prof/all!

 Can you please shed more light on the trash bins for me?
 can we modify the tool to handle pptx and xlsx files as well?

 Regards
 Nope, they're completely different formats.

 Try EXIFTool.
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
 ► November (16)
 ► October (18)
 ▼ September (15) New issue of the IJDE
 Something old, something new...with USB
 Perl Programming on Win32
 MetaData and eDiscovery
 FIRST Conf '07 CfP
 Vista RC1 Install
 ProDiscover 4.8 is out!
 OS Detection, Explained
 OS Detection from a RAM Dump, part deux
 OS Detection from a RAM Dump
 Gromozon Rootkit
 Extracting and authenticating files from RAM dumps
 Win2003SP1 and RAM dump parsing
 What's new
 Using Perl in the Enterprise

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
