# Windows Incident Response

- URL: https://windowsir.blogspot.com/2005/12/mystery-of-muicachesolved.html
- Published: unknown
- Updated: unknown
- Labels: none

Windows Incident Response
 The Windows Incident Response Blog is dedicated to the myriad information surrounding and inherent to the topics of IR and digital analysis of Windows systems. This blog provides information in support of my books; "Windows Forensic Analysis" (1st thru 4th editions), "Windows Registry Forensics",
as well as the book I co-authored with Cory Altheide, "Digital Forensics with Open Source Tools".
 Tuesday, December 27, 2005
 The Mystery of MUICache...solved??

 19 comments:
 About "when the application was run" could you not dump the registry with "reg export" and then view the dump with http://www.mitec.cz/wra.htm
 to see the timestamp of the entry?
 Sure, you could, but isn't that an awful lot of work for something like that? After all, I've written Perl scripts that retrieve that information directly from the Registry. Go to the Windows-ir.com web site, click on "Tools", and there's a link to an archive for several tools that includes "keytime". This comes as a standalone EXE as well as a Perl script.

 Thanks,

 Harlan
 RSS Feed. Off topic, but wanted to request if you could turn on your site feed so I can add it to my google homepage.
 Hi Harlan
 So "we don't know when the application was run" is not entirely true or? does this "keytime" tell us when?
 RSS Feed is on: http://windowsir.blogspot.com/atom.xml

 Let me know if it works for you, and what site you're using.

 As far as the keytime goes, take a look at the MUICache key...there's not MRUList, so you can't tell when the last entry was made, or even which one was the last entry. You'd have to correlate the information in the key with info from the filesystem, but there isn't enough info using just the key.
 I tried that link last night using google's homepage, but it seems to work now. Thanks!
 What site will the feed be going to?

 What's your site?
 A possible answer to the Trivia Question:

 cmd.exe /k cd c:\ && color fc && title ***** HERE IS WHAT YOU WANT TO CHANGE *****
 I was really hoping to get some comments on the Registry key...
 Hiya. This is a really, really old post, and I have no idea if anyone will read it -- HOWEVER, I can tell you this: I just wrote a little program in VB.Net,and I wanted to get it to show up on the OpenWith dialog's list of Recommended Programs. I used Inno Setup to create an installer package for the program, and (using Inno Setup) made some Registry entries that would add keys and values to get the program on the List, and delete same on uninstall. However, in order to get the program's NAME (or any other info I wanted) to show up on the list, I had to create an entry in the MUICache.

 Hope this helps.
 The entries under this key are also used as the application title displayed in the task bar when the icons have been grouped together. This happens on XP and 2003 when "Group Similar Taskbar Buttons" has been enabled. Also, this entire subkey is refreshed by explorer.exe often, so don't count on keys being retained.
 According to your entry, the values under the key are FileDescription(s) for the executable(s).

 Moving forward, I changed the name of the executable and ran it via the shell, hoping to verify that indeed it was the FileDescription that was reflected in the registry. Somehow, the value reflected the changed name of the executable instead.
 Like "Anonymous", I also created a little picture-viewing program that I wanted to appear in "OpenWith..." and I also used InnoSetup. The problem I'm having is getting the description to STAY in the MUICache. The program remains, and it stays on the OpenWith list, but the description periodically disappears. Don't know Why. Have to go back in and edit the key's Value to get it back. Weird. Any thoughts what could be doing this? (also unable to get the icon correct.)

 So the MUICache is can be, in my opinion, more innocuous than some think; but it also appears to be more complex.
 Lynn,

 I'm not sure how this is a forensics-related post...it may be best answered on a programmer's forum.

 Thanks.
 Hi. I do forensics, too and stumbled across your site. Looks great!

 I am cleaning a system now (XPH) and it had a ton of malware, including backdoor.genlot.aet, and sembako-chzjlog.exe, among others.
 I had tried BitDefender on it (just to try BD) and it cleaned out 3260 infected items.

 After BD cleaned it and the sembako- file was deleted, the reboot showed an error saying that that file could not be found. I did find it in the registry under the MUICache.

 I thought, being a cache, that this was only a list of recently run files.

 Under Winlogon is a key Shell that has the value Explorer.exe "C:\Windows\sembako-chzjlog.exe"

 As far as max entries, this one has 112 entries.
 Boethos.

 I am facing similar problem with another Sembako-cjzjlri.exe ...

 How do I remove this error beep? I am not a tech guy... however, can follow the instruction up to opening of registry ....

 please mail me at rajiv.bishnoi@gmail.com

 Thanks

 Rajiv
 Hi everyone

 So, I understood most of the post, though (5 years later) I do not know the context, so I have some questions please?

 Are these .exe files created dangerous? Why is the malware causing the Explorer.exe shell to create them?

 Are they responsible for opening up random websites through firefox? Why do they cause error pop-ups if they are malware - surely if they were harmful or collecting/sending data it would be smarter to be "invisible"?

 How can I STOP them? Or stop the error pop-ups? I have recently been attacked by Antimalware Doctor (AD), and though I have cleared my registery and run a scan and cleaned out what was left, I still have residual issues. Are these pop-ups originally supposed to LOOK like AD's fake scan pop-ups that would try to convince you to buy the product? And now because the malware's files are mostly removed all that's left is an executable file with no camouflage, that without the right tools fails to run?

 Does this mean that I may DELETE these executable files please? And what about the Explorer shell - can I locate the point where the malware is being activated to stop creating more files? It seems to put a lot of pressure on explorer.exe, or messing with the process, because explorer.exe also has error pop-ups and at shutdown brings up "this programme is taking too long to respond. Wait or shut down immediately"

 I look forward to hearing from you, and seeing if I understand anything at all!
 It is not clear to me what executables are referenced in the MUICache and which are not. I understand what element of the PE resource is read and added to the registry key, but it doesn't seem to be the case that if an .exe has a string added to the FileDescription field of its VERSION_INFO resource section that it will automatically be added to the MUICache when launched.

 if two .exe's both have a string added to their FileDescription field...why will one be added to the MUICache and the other not?

 Thanks,
 Stephen
 It is not clear to me what executables are referenced in the MUICache and which are not.

 I don't think that's clear to anyone.
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

 ▼ 2005 (163) ▼ December (5) The Mystery of MUICache...solved??
 Registry Reference
 The age of "Nintendo forensics"...
 Visibility?
 Registry Reference

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
