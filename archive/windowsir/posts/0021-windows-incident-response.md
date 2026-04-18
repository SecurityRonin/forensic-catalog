# Windows Incident Response

- URL: https://windowsir.blogspot.com/2007/04/from-lab-mapping-usb-devices-via-lnk.html
- Published: unknown
- Updated: unknown
- Labels: none

Windows Incident Response
 The Windows Incident Response Blog is dedicated to the myriad information surrounding and inherent to the topics of IR and digital analysis of Windows systems. This blog provides information in support of my books; "Windows Forensic Analysis" (1st thru 4th editions), "Windows Registry Forensics",
as well as the book I co-authored with Cory Altheide, "Digital Forensics with Open Source Tools".
 Monday, April 09, 2007
 From the Lab: Mapping USB devices via LNK files

 21 comments:
 Harlan,
 I wonder if the information is actually left over in the ntuser.dat of the user that formatted the volume?
 Interesting...any thoughts on where to look?
 I was going to go ahead and experiment with it a little bit today. No immediate thoughts on where, just a thought that it might exist there.
 I'm sure you'll find evidence in the UserAssist key that the MMC associated with the DiskManager was run, but beyond that, what information is usually left over when someone formats a disk or volume?

 From my perspective, I don't know of any. I'd like to better understand your thought processes with regards to what you're thinking is in the NTUSER.DAT.
 Harlan!

 Looking forward to getting a copy of the book. Nice to know you are still out there kicking it up.

 Point me to the BEST resources for learning PERL when you get a chance, or do I need to wait to read the book?

 Hope it sells millions!
 Heath,

 Nice to know you are still out there kicking it up.

 Thanks. Have we met?

 Point me to the BEST resources for learning PERL when you get a chance, or do I need to wait to read the book?

 It depends on what you want to do. The *best* resource, IMHO, is the OReilly books, and if you want to go specifically Windows, Dave Roth 's books are THE BEST !!

 I started with "Learning Perl on Win32 Systems", and moved on to "Advanced Perl Programming". However, I really have to say that most of my really exceptional leaps in knowledge have been from interacting with others, and looking at the code written by others.

 Hope that helps!
 Harlan,

 I have seen situations when using EnCase to review the USBSTOR entries I have found that a thumb drive's serial number was reported. When connected to my Tableau USB write blocker, I checked the information reported regarding frimware version, serial number, etc., and sure enough, it matched what was reported by the USBSTOR entry within EnCase.

 Of course, this is not always the case, and I find that the more well-known a device is, the better such a correlation exists. For example, my own experience is that for SanDisk thumb drives, more often than not, i have found a match between what the Tableau device reports as a serial number and what is listed within the info displayed within the USBSTOR.

 So I guess my point is that is is possible that the serial number is reported by the USBSTOR information, but as there is yet no hard-fast rule regarding this, each vendor is free to do whatever they want, so perhaps it is still at best a case-by-case basis.

 Or perhaps you might argue that this "serial number" is vague in that it might be the volume serial number or a hardware/device serial number. This brings up another query:

 As long as it matches the evidence I have from within the setupi.log and/or USBSTOR entries, does it matter? a match strongly supports the connection, so does it matter as long as it matches?

 This is the heart of the matter, isn't it?

 -srobtjones
 srobtjones,

 "I have found that a thumb drive's serial number was reported"

 Yes, I'm sure. I've written about this on numberous occaisions, as well as blogged about it.

 "...i have found a match between what the Tableau device reports as a serial number and what is listed within the info displayed within the USBSTOR"

 I'm sure...because if a serial number exists for the device, it is in the device descriptor, which means that what is in the USBStor key and what is being reported by the Tableau device is being pulled from the same location.

 Reading through the rest of your comment, it's clear that there's some confusion here. What you are seeing in the USBStor key is the device's serial number. This is not a volume serial number. The VSN is the serial number assigned to a volume when the volume is formatted, as referenced by the documentation I've linked to.

 Thanks for your comments.
 Hi Keydet89,

 Interestingly, I'm doing something as to what you've blogged. However, I noticed that if the device connected is an external USB drive, the DosDevice entry will not store the string "\??\STORAGE Removeablemedia", instead i see non-ascii values.

 This is killing my brain cells as my program will fail to determine if the drive is a removable storage.

 You have any ideas or workarounds?

 - stardust
 stardust,

 I'd have to see an example of what you're referring to...
 keydet89,

 Here's a screenshot of the binary value in DosDeviceG when i plugged in my external USB hdd into my host machine:

 http://www.flickr.com/photos/21152357@N04/2055535783/

 It's not the usual "\??\Storage\..." that we see when we plugged in a thumbdrive.

 Cheers,
 stardust
 I had assumed that all USB storage devices connected to the host would contains the "\??\Storage\.." value but somehow, it wasn't the case?

 stardust
 It's not the usual "\??\Storage\..." that we see when we plugged in a thumbdrive.

 From the graphic you posted on flickr, it appears that the system has recognized this device more as an external hard drive, than an a removable storage device.

 I have a WD PassPort USB-connected 120GB drive, and it get's recognized in the same manner as you've presented above. I've got some information about this in my book, Windows Forensic Analysis .
 I see. Let me check out your book first for more information. Thanks! :)

 stardust
 Its now September 2011, and I am looking at the same issue still, Harlan. I have a situation where I have a number of device images, one being a laptop. I'd like to link the images of the USB flash drives with their volume serial numbers to laptop registry and other file evidence including: (1) specific USB VID/PID/SN values and matching entries in the SetupAPI device logs; then (2) their drive letters; along with (3) LNK shortcuts to files that perhaps were on the removable drive but have been deleted & wiped; and similarly (4) ShellBags information on full paths to files and the removable devices they were once on.

 As you summarize at the very end of your post, this may involve a significant timeline analysis and some good fortune, but may not yield an ironclad link between the image and the physical device info.

 Has anything popped up on your horizon in the passing years on this matter?
 George,

 Has anything popped up on your horizon in the passing years on this matter?

 I'm not sure what you're trying to get at, as I think that enough information has been developed since I wrote this post to address this issue.

 Can you contact me offline at keydet89 at yahoo dot com and let me know a little bit more about what you're looking for? I understand what you're trying to do, but what is it I can do to assist?

 Thanks.
 I obtained USB device artifacts from a forensic digital report, for one of the USB Drives there is no drive letter assigned to it, however the lnk files with Linked path of D: point to that specific USB?
 This post is 16 yrs old...what version of Windows was examined? Did you go back and ask whomever wrote the report about this?
 It was Windows 10 Professional (2009). Haven't consulted with the expert yet. Due to some budget constraints cannot reach out to the expert again. But I see two storage devices and the drive letter D: is only assigned to one of them.
 I can't speak to what the expert did, but it could be that the drive letter was assigned to both devices, at different times.

 You should be able to tell via the Windows Event Log.
 http://windowsir.blogspot.com/2022/05/usb-device-redux-with-timelines.html
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
 ▼ April (7) Something New To Look For
 WFA Sample Chapter
 Drive Encryption
 From the Lab: Mapping USB devices via LNK files
 Great news for IR and live response!
 Interesting Tool - SecInspect
 Using Perl in Forensics

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
