# Windows Incident Response

- URL: https://windowsir.blogspot.com/2019/08/program-executionor-not.html
- Published: unknown
- Updated: unknown
- Labels: none

Windows Incident Response
 The Windows Incident Response Blog is dedicated to the myriad information surrounding and inherent to the topics of IR and digital analysis of Windows systems. This blog provides information in support of my books; "Windows Forensic Analysis" (1st thru 4th editions), "Windows Registry Forensics",
as well as the book I co-authored with Cory Altheide, "Digital Forensics with Open Source Tools".
 Friday, August 16, 2019
 Program Execution...Or Not

 19 comments:
 Interesting thoughts Harlan. Just because a program was executed, doesn't mean it was used, is something that gets overlooked a lot. I think, from my experience I would try and find additional artifacts from the program being used such as files created or other files modified. Hopefully these would show up in a timeline analysis of the system and possibly application specific event logs. Just another reason why when an analyst makes a statement in a report, they need at least 2 pieces of evidence to back it up.
 Chris,

 Great comment, and I fully agree.

 Beyond assumption (i.e., there's an artifact of program execution, so it *must* have been run...), there's also the issue of data/artifact misinterpretation, with AppCompatCache time stamp being perhaps the most misunderstood artifact.
 For the FTP question: the first thing that comes to mind is using the [System.Net.FtpWebRequest]::Create function in Powershell to create the connection.
 In my enviroment, I'd use our EDR to look at powershell commands that are run, and proxy logs to see the network connection.
 James,

 Great answer!

 There's still another way, or perhaps more accurately, likely other ways...but that's a great answer. This would leave or modify a Prefetch file for PowerShell, but not for ftp.exe.
 Go to FTP://REMOTE_IP in your browser, that should leave iexplorer.exe in the Prefetch. Check firewall/proxy logs to see if FTP was used?
 Jassim,

 That's a great solution, and it's getting closer to what I had in mind.

 Also, there would be a value added to the TypedURLs key, and depending upon the version of Windows, a corresponding value in the TypedURLsTime key.

 Again, great solution, and you're getting closer!
 To add ---- In Windows, event ID for process execution (4688) does not mean it was executed. You need to refer to the process exit (4688) related to that PID its status code in order to see if it was really executed.. This thing is always missed by SOC (they always reported its executed without referring to the process exit event)
 One way is the use of bitsadmin.exe to transfer a file. For example:
 Upload: bitsadmin /transfer /upload
 Download: bitsadmin /transfer /download

 Some ways to detect:
 - Monitor execution of bitsadmin for specific command-line arguments (Endpoint)
 - Parse Windows Event Log - Microsoft-Windows-Bits-Client/Operational (Endpoint)
 - Look for User Agent String: Microsoft BITS/x.x in web proxy logs (Network)

 Just thought of using putty to ftp to a remote server. Prefetch will log putty.exe
 Taking a quick guess..:-p If using a solid state drive the enableprefetcher default setting will not create prefetch. The extra write operation would affect performance.
 Great comments!

 Anonymous, I don't believe that bitsadmin uses the FTP protocol, but yes, it can be used to download (and upload) files.

 Jassim, great answer!

 Greywolf570, great answer!

 There's still another way, though! ;-)
 Hey Harlan, nice post! Thanks for sharing as always. On the FTP question, one example that comes to mind is using Windows Explorer to access an FTP site. In that case, you could take a look at the shellbags to see references to the interaction with the FTP server.
 yawnz... ftp works via Windows Explorer
 Jason Hale was the first to provide the answer I was looking for, and anonymous/yawnz followed up with the same answer!

 However, neither one provided the "how would you determine/verify it?" part of the answer...any thoughts??
 yawnz... HKCU\Software\Microsoft\FTP\Accounts + possibly MRU + possibly NTFS artifacts (if file was copied/downloaded) + memdump analysis could show file transfers
 1C76188233DAF93F4DE192708AFC83AACF671365C5D363477F09EB2F8CEA8691
 yawnz...good guesses, but that's what they are...guesses. ;-)

 I connected to an anonymous FTP server using Windows (NOT Internet) Explorer, and there were no entries beneath HKCU\Software\Microsoft\FTP.

 There was, however, this:

 Explorer\ftp://ftp.cs.brown.edu [Sat Aug 17 20:22:10 2019] [Desktop\12\0\]

 The answer is 'shellbags'. ;-)
 yawnz... they are not guesses, but actual artifacts that can be found - you just need to do more tests :-)
 To claim they are guesses is not forensically sound.

 I just confirmed HKCU\Software\Microsoft\FTP with a FileZilla Server installed on a localhost and with no network connections. If you google this Registry key you will find plenty of references online.

 Interestingly, I have not seen the shellback artifact you mentioned either, so here we are talking about two non-reproducible scenarios... :)
 To further obfuscate, you can map FTP shares as drives in Windows - shellbags would still track, but would require knowledge of the mount point's underlying destination (HKCU\SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\MountPoints2\ for those playing along at home).
 Great stuff, Mike!!
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

 ▼ 2019 (43) ► December (5)
 ► November (2)
 ► October (2)
 ► September (3)
 ▼ August (4) DFIR Open Mic Night
 Program Execution...Or Not
 Chasing the DFIR Cure, pt II
 Chasing the DFIR Cure

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
