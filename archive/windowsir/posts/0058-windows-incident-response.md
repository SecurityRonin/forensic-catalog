# Windows Incident Response

- URL: https://windowsir.blogspot.com/2009/11/working-with-volume-shadow-copies.html
- Published: unknown
- Updated: unknown
- Labels: none

Windows Incident Response
 The Windows Incident Response Blog is dedicated to the myriad information surrounding and inherent to the topics of IR and digital analysis of Windows systems. This blog provides information in support of my books; "Windows Forensic Analysis" (1st thru 4th editions), "Windows Registry Forensics",
as well as the book I co-authored with Cory Altheide, "Digital Forensics with Open Source Tools".
 Wednesday, November 18, 2009
 Working with Volume Shadow Copies

 17 comments:
 I tried this a few weeks ago using a Windows 7 Pro dd image, Windows 7 host and guest and VMWare 2.0 Server, and it worked as well.
 As you know, VSS stores Volume ID with a snapshot to registry.
 HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\SPP\Clients
 vssadmin can recognize a snapshot after Volume ID was registered with registry.
 I confirmed it in VMware environment of Windows 7 & Vista.
 However, I am still testing it ;-)
 Looking at your summary, I'd shorten the process by just booting an image of the target system. Then, I'd run vssadmin in the VM. Next, I'd use Dan Mares' VSS to mount the shadow volume(s) of choice, and image them with FTK. I'd run FTK from a USB drive attached to the VM and image the shadows to that drive. It's slow, as Troy advised.

 I haven't tried your outlined procedure, but I'm sure that it works if you've done it that way. In the 1-2 cases where I tried to use mklink, I was unable to access the linked image. I think that it's a permission problem, and others have found the same thing. I was unable to overcome the issue, but simply gave up when VSS worked so easily. If you find yourself in that boat, I'll be interested to see whether you can resolve the access problem.
 I've found that booting the Vista disk image is a bit hit-and-miss. For cases where this doesn't work well I put the disk image in the VM and boot the VM with Windows FE. This allows me access to all the Shadow Volumes so I just DD the Shadow Volume out, as you've described Harlan.

 As for Dan Mares' VSS. It looks like this tool has been discontinued and his site is shutting down. Does anyone know where I can get hold of this tool now?
 Jimmy,

 In the 1-2 cases where I tried to use mklink, I was unable to access the linked image.

 What do you mean? Did you receive an error message? If so, what was it?
 Lee,

 Dan just posted to the CFID list this morning...you might try reaching out to Dan, rather than posting someplace where he may not be looking...

 HTH
 Did you receive an error message?

 It wasn't an error, per se, but something like the message you get when you try to access a system-only folder.
 Jimmy,

 Did you ensure that the trailing "\" was added to the GlobalRoot path?
 Mounting the suspect drive via a write blocker to a vista system is the way to go on this. I have done it many times with great sucess
 Did you ensure that the trailing "\" was added to the GlobalRoot path?
 Yes. It wasn't an issue with creating the link, but accessing the link after it was mounted. Although the volume was "there," I could not access any object that it contained, nor could I image the volume.
 For live images, I have used F-Response to connect my Vista target drive's VSC data to my Win7 analysis machine.

 Troy has mentioned this too, and I tested that it works, you can also mount the VSS using cmds/tools like :

 a) net share shadowcopy1=\\.HarddiskVolumeShadowCopy1\

 or

 b) Dosdev y: \\?\GLOBALROOT\Device\HArddiskVolumeShadowCopy1
 Also of note re: booting VM images: if you stay virtually booted too long, you may run the risk of the VSS background process creating new VSC entries and thus deleting old ones. I had been playing with a Vista vm image 6+ monthes ago. I came back to it a two monthes ago to review something and noticed another user had "booted" my vm image for their testing of something. That period of activity (uptime) created new VSC entries and deleted my old ones.
 Hi,

 I'm not sure if you're still checking the comments on this posting, as it is a pretty old article...

 But I recently followed all of your steps (which was VERY helpful for me) as part of a homework assignment. However, right now, my MD5 hashes of the images I have created from the Volume Shadow Copies are not matching the assignment's possible answers...

 So I'm curious...in transforming the vmdk to a vhd...would that alter in any way the hash of the image that is created? Or is the --cryptsum function of FAU's dd.exe accurate? I'm stuck as to why the hashes of my VSC images would be off.

 Thanks! I'm a big fan of all your posts!
 I receive an email whenever a new comment is added to any blog post...

 ...in transforming the vmdk to a vhd...

 Perhaps. I know that using vhdtool.exe to convert a raw/dd image file to a VHD adds a 512 byte footer to the end of the file, which will change the hash. So it's very likely that converting your VMDK to a VHD file altered it enough to change the hash...after all, if all you did was flip a single bit, *that* would alter the hash...
 Thank you for the quick reply.

 If I wrap my head around whats actually happening, it does make sense that the hashes would be different. (Although I used vmdk2vhd as opposed to vhdtool, I imagine they work in similar ways.)

 Thanks!
 Thank you for your great book and for this post. I was reading the book and did a test using Windows 7 64b running under Virtualbox, and when I didn't specify the "\\?" before the first "\" I couldn't get access to the VSC using the symbolic link.

 Not sure if not using it on other systems might work or not. On mine I had to use the "\\?" and thought of checking with you and ask if you left it out from the book too.

 Thanks again for your wonderful work.

 Regards,
 I am in need of step by step instructions to get a shadow copy of a Windows System Image restored to a usable file or directory. I can see the shadow copy as follows:

 C:\Users\ndavidson.ILSCO>vssadmin list shadows /for=E:
 vssadmin 1.1 - Volume Shadow Copy Service administrative command-line tool
 (C) Copyright 2001-2005 Microsoft Corp.

 Contents of shadow copy set ID: {9d5c0a4c-a1fd-41a8-ae45-80eea05f98ac}
 Contained 1 shadow copies at creation time: 8/13/2015 7:23:33 PM
 Shadow Copy ID: {9e0a646e-d7fc-47d8-b000-016ec0306f8a}
 Original Volume: (E:)\\?\Volume{f0fe8d8f-7a50-11e1-9f9d-b870f4c5bf8d}\
 Shadow Copy Volume: \\?\GLOBALROOT\Device\HarddiskVolumeShadowCopy4
 Originating Machine: Norms_Asus
 Service Machine: Norms_Asus
 Provider: 'Microsoft Software Shadow Copy provider 1.0'
 Type: DataVolumeRollback
 Attributes: Persistent, No auto release, No writers, Differential

 How do I make this readable to do a recover or restore on my home PC? The shadow copy resides on my external hard drive. I need to use it to restore my Laptop to this copy.

 Thanks in advance for any direction you can give me.
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
 ▼ November (14) Incident Preparation
 More Timeline Creation Techniques
 Even More Linky Goodness...
 Working with Volume Shadow Copies
 It's about time...
 Some Analysis Coolness
 In The News
 Happy Birthday, VMI!
 Happy Birthday, Marines!
 p0wnage
 More Linky Goodness, plus
 Link-alicious
 The Future of RegRipper
 Into The Boxes

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
