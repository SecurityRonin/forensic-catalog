# Events Ripper

- URL: https://windowsir.blogspot.com/2022/10/events-ripper.html
- Published: 2022-10-15T06:59:00.000-05:00
- Updated: 2022-10-15T06:59:22.631-05:00
- Labels: auditpol, Events Ripper, ISO

Not long ago, I made a brief mention of Events Ripper , a proof-of-concept tool I wrote to quickly provide situational awareness and pivot points for analysts who were already on the road to developing a timeline. The idea behind the tool is that artifacts are compound objects, and have value based not just on their time stamps, their value can also be predicated on the analysis questions or goals, or just the nature of their path, or some other factor.
 The tool leverages the fact that analysts are already creating timelines, and uses the intermediate events file format to develop situational awareness and pivot points to facilitate analysis. Many times, we're looking through a timeline for some root cause or predicating event, but we're dealing with the fact that there was some normal system behavior (such as an update) that's caused a large number of events to be generated.
 At the moment, the available plugins target Windows Event Log data, in many cases producing output similar to what analysts are used to seeing in ShimCache or AmCache parser output. So, of course, the output of the various plugins are going to depend upon the Windows Event Logs you've included in the timeline, as well as how long it's been since the activity in question occurred (i.e., logs roll over), and what specifically is being audited (although that pertains more to the Security Event Log). Further, they're doing to also depend upon what's being logged, something you can check via the auditpol.exe native utility (or the auditpol.pl RegRipper plugin). For example, I've once saw a Security Event Log with over 35,000 records, and they were all successful logins. Yep, that's it...just successful logins, and because of the nature of the system, most of them were type 3 logins...which is why I wrote a plugin to just get a count of logins by type, so that it's easy to see this information about your data quickly.
 That's one of the keys to this tool...to be able to quickly and easily distill and discern some important insight about the data that you have from a system. As such, the real value of this tool comes from analysts using it, exploring it, and asking questions, talking about how to view and manage the data they have.
 Tools like this are especially useful in diverse environments where you are likely to encounter data sources with disparate content, such as consulting environments. During my time in consulting, I never... never ...saw two identical environments. Ever organization is different. In fact, it wasn't unusual to find different application loads and audit configurations between departments, or sometimes even within the same department. So when you find something new, you create a plugin to parse it out and provide context, because you never know when you, or another analyst on your team, is going to see it, or something like it, again.

 Something else that a few analysts are familiar with is that Application Event Log records can often contain references to malicious software, in DCOM errors, Application Hang event records, and Windows Error Reporting event records. As such, I wrote plugins for each of these event records that lists the impacted applications in a format similar to what's seen in ShimCache or AmCache parser output.
 How To Use It
 Here's example output from the ntfs.pl plugin:
 D:\erip>erip -f g:\ntfs_events.txt -p ntfs
 Launching ntfs v.20221010
 Get NTFS volumes
 System name: enzo
 Mounted Volumes:
 C:\ -  WDC WD5000BEKT-75KA9T0
 D:\ -  WDC WD5000BEKT-75KA9T0
 F:\ - Msft     Virtual Disk
 F:\ - WD       My Passport 0741
 F:\ - WD       My Passport 25E2
 G:\ - SanDisk  Cruzer
 Analysis Tip: Microsoft-Windows-Ntfs/145 events provide a list of mounted volumes.
 From the above output, we can see that the C:\ and D:\ drives (the system named "enzo" has one hard drive split into two volumes), but we can also see other drive letters listed, along with their associated friendly names. We can likely find similar information in the Registry, and I'd definitely want to include that info, as well, but this is immediate, valuable insight from a limited data source, as I can quickly see drive letter mappings. However, I do need to keep in mind that this information may not be complete, but it's a good start.
 Here's example output from the mount.pl plugin:
 D:\erip>erip -f g:\vhd_events.txt -p mount
 Launching mount v.20221010
 Get VHD[X]/ISO files mounted
 System name: Stewie
 Files mounted (VHD[X], ISO):
 D:\test\iso\b91c8f5adb81a262b2b95e2bf85fd7170e100885600af1f9bde322f10ac0e800.iso
 D:\test\billid0574.iso
 D:\test\iso\test.iso
 Analysis Tip: Microsoft-Windows-VHDMP/1 events provide a list of files mounted or "surfaced".
 Let's say that you look at the above output and think, "I want to see a timeline of all instances where 'test.iso' was involved"; well, that's easy enough to do, in a few simple steps:
 type g:\vhd_events.txt | find "test.iso" > g:\iso_events.txt
 parse -f g:\iso_events.txt > g:\tln.txt
 Now, you have a timeline of all of the events that include "test.iso".
 Interestingly enough, the above output is from one of my own systems, and once I saw it, I checked the values within the RecentDocs/.iso Registry key and found all three of those ISO files listed.
 Using the two above plugins, I'm able to get a quick look at drive mappings for devices, as well as mounted ISO files, with minimal effort.
 So What?
 So, why does any of this matter? Red Canary recently shared some open reporting on Raspberry Robin , where they stated that this malware was spread via infected thumb drives. However, they also stated that there were "several intelligence gaps around this cluster", mentioning one of these gaps. Note that Cisco Talos also reports that Raspberry Robin spreads via "external drives"; however, Cybereason indicates that it could be "removable devices or ISO files". I'm not suggesting that this is a disparity in primary sources, but rather that it's pretty straightforward to gather insight and some solid answers based solely on one or two Windows Event Log files.
 Conclusion
 Tools like this provide for:
 - Creating situational awareness and "pivot points" from your incident data
 - Creating context and insights from your incident data
 - Corporate knowledge retention, particularly for diverse environments, such as you find with consulting
 - An alternate/additional means for analysts of all levels to contribute
 - Fully exploiting limited data sets
 However, tools like this (and timelines, as well) are limited by:
 - Which Windows Event Logs are included
 - The applications installed on the system
 - The audit policy of the system
 - How long it's been since the incident occurred
