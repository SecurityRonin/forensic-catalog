# Windows Incident Response

- URL: https://windowsir.blogspot.com/2021/01/extracting-toolmarks-from-open-source.html
- Published: unknown
- Updated: unknown
- Labels: none

Windows Incident Response
 The Windows Incident Response Blog is dedicated to the myriad information surrounding and inherent to the topics of IR and digital analysis of Windows systems. This blog provides information in support of my books; "Windows Forensic Analysis" (1st thru 4th editions), "Windows Registry Forensics",
as well as the book I co-authored with Cory Altheide, "Digital Forensics with Open Source Tools".
 Thursday, January 28, 2021
 Extracting Toolmarks from Open Source Intel
 I've talked about toolmarks before...what they are, why (I believe) they're important, that sort of thing.  I've also described how I've implemented them, and about toolmarks specific to different artifacts, such as LNK files . The primary source for toolmarks should be the investigations you're performing; when you do data collection pursuant to an investigation, those toolmarks that you develop should be baked back into your analysis process.  For #DFIR consulting businesses, this is a truly powerful use of the petabytes of data flowing through your organization on a monthly basis, driving toward increasingly efficient analysis and reducing the engagement/SOW lifecycle.
 While your own investigations should be the primary source of toolmarks, you can also take advantage of open source reporting to extend this capability. In some cases, open source reports are full of unrealized toolmarks, which any organization can leverage to extend their detection and threat hunting (including #DFIR threat hunting) capabilities.

 I know what you're thinking...how would you go about doing that?  How do you turn open source reporting into something actionable, leveraging toolmarks to extend your organization's capabilities?  Well, let's take a look...
 Recently, Microsoft published a security blog regarding the 2nd stage activation from SunBurst , and based on the information they provided in the article, I thought that this would be a good opportunity to illustrate how to extract or realize toolmarks from open source reporting. The Microsoft article is a great example, because it is chock full of intrusion intel that leads directly to toolmarks.
 For example, consider fig. 3 in the article; step 3 shows an "Image File Execution Options" Debugger value being set for the dllhost.exe executable.  The toolmark here is obvious; a new subkey is created, and a new value is added to that subkey. In step 6 we see that the Debugger value is deleted; at this point, the question is, is the dllhost.exe subkey left in place?  If so, the LastWrite time of the key would correspond to when the Debugger value was deleted; if not, and the dllhost.exe subkey is also deleted, then the residual toolmark becomes the LastWrite time of the "Image File Execution Options" key.  As a result, if the time stamp toolmark in question falls within the window of other interesting activity, then you likely have an actionable toolmark associated with this activity.
 Make sense?
 A couple of paragraphs below the figure, we see the following statement:

 Finally, the VBScript removes the previously created IFEO value to clean up any traces of execution (step #6) and also deletes the following registry keys related to HTTP proxy:
 Nomenclature alert...the two subsequent paths listed are actually to Registry values, not keys. As a result, in this case, the LastWrite time of the "Internet Settings" key would correspond closely to the above toolmark (i.e., IFEO key LastWrite time).  These two time stamps are very specific set of toolmarks related to this specific activity.
 Now, let's mosy on down to the section of the article that mentions "anti-forensic behavior" because, well, this is the fun stuff.  The second bullet in that section includes the statement, "... WMI persistence filters were created ..."; basically what this tells us is, be sure you're parsing the OBJECTS.DATA file!  So, different data source (i.e., not the Registry in this case), but a definite toolmark.
 The third bullet in the "anti-forensic" section states, "... the attackers took care of disabling event logging using AUDITPOL and re-enabling it afterward. "  Oh, now THIS is cool! We see in the table following this section that the command used is:
 auditpol /set /category:”Detailed Tracking” /success:disable /failure:disable
 This command modifies the "Default" value beneath the Policy\PolAdEvt key in the Security hive, which in turn causes the LastWrite time of the key to be updated.  The article then states that when the threat actor completes their activity, they set "success" and "failure" to "enable"; at this point, the toolmark is the LastWrite time of the key, and the value settings themselves.
 Next, when the threat actor modifies the Windows firewall by adding a rule via netsh.exe , the action results in a modification to the Windows Registry, as does the use of sc.exe and reg.exe to disable Windows services remotely and locally, respectively, and the use of net.exe to map a OneDrive share. All of these actions result in a modification to the system that is evident tithin the Windows Registry, and all you need to do is pull them out in a timeline to see how close they are, temporally.
 Another example of extracting toolmarks from open source reporting can be found in this article that describes how to use the finger.exe client to transfer files between systems. The article describes the use of netsh.exe to set up a port proxy in order to get port 79 TCP traffic out of the network.  The command used is similar to the one described in t his Mandiant article on RDP tunneling , which results in a modification to the HKLM\System\CurrentControlSet\services\PortProxy\v4tov4\tcp key, adding a value. When the port proxy configuration is removed, the action can be indicated by the LastWrite time of the tcp key being updated.
 An important aspect to keep in mind is detection and response time with respect to the toolmarks being created.  What I mean by that is that if response occurs relatively close to the action occurring (i.e., a Registry key and/or value being added), then the modification may exist in the transaction log, and may yet to be written to the hive.  This is also true with respect to the deletion.  Also, if the key LastWrite times correspond to the window of suspicious activity, but the keys/values do not exist, be sure to parse unallocated space within the hive file to determine if the deleted nodes can still be found and extracted.
 While your own investigations should continue to be the primary source of actionable toolmarks that you apply back to or bake back into your analysis process, incorporating toolmarks developed and leveraged from open source reporting can significantly extend your reach and capabilities.
 No comments:
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
 ► August (2)
 ► June (4)
 ► April (4)
 ► March (1)
 ▼ January (1) Extracting Toolmarks from Open Source Intel

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
