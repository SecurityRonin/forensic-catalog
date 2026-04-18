# Digging Into Open Reporting

- URL: https://windowsir.blogspot.com/2022/04/digging-into-open-reporting.html
- Published: 2022-04-13T20:14:00.002-05:00
- Updated: 2022-04-14T07:09:58.797-05:00
- Labels: none

As many readers of this blog are aware, I often find great value in open reporting, but that I also see the value in taking that open reporting a step (or three) further beyond where it exists now. In more than a few instances, something extra can be pulled from open reporting, something not presented or discussed in the article that can be of significant value to readers in domains such as DFIR, detection engineering, MSS/SOC monitoring, etc. As a result, I've spent a great deal of time during my career looking for alternate means for detecting activity (user, threat actor, malware) presented in open reporting, largely due to gaps in that reporting.
 For example, there's a great deal of open reporting that is based solely on RE and analysis of malware that is part of the final stage of the attack (ransomware, etc.), without addressing anything that occurred prior to the final malware deployment, beginning with initial access. Now, I understand that not everyone has access to DFIR data to support this level of open reporting, but some do , and tremendous value can be derived if that information is shared. I understand that in some instances, authors feel that they can't share the information but very often it may simply be that they don't see the value in doing so.
 MS DART and MSTIC recently published some excellent open reporting on Tarrask that I found quite fascinating, based initially on the title alone (which, oddly enough, is what also catches my eye with respect to beers and books). I mean, the title refers to using Scheduled Tasks for defense evasion...when I read that, I immediately wanted to know more, and I have to say, the DART and MSTIC teams did not disappoint.
 The article itself is full of "stuff" that can be unpacked. For example, there were some interesting statements in the article, such as:
 Further investigation reveals forensic artifacts of the usage of Impacket tooling for lateral movement and execution...

 So, what does that look like? I searched on the web and found that someone shared some forensic artifacts associated with the use of Impacket tooling about 2 yrs ago. While this is a great resource, unfortunately, many of the Windows Event Log records described are not available via the default logging configuration and require additional steps to set up. For example, while generating Security-Auditing records with event ID 4688 is just a matter of enabling Process Tracking, including the full command line for the process in the event record requires an additional Registry modification . Throughout my time in DFIR, there have been very few instances where the audit configuration of compromised systems was beyond default, and more than a few instances where threat actors took steps to disable the logging that was available.
 Note that the article has a similar comment to that effect:

 Neither of these are audited by default and must be explicitly turned on by an administrator.
 Ah, there you go! There are these great Windows Event Log artifacts that are available if the default audit configuration is modified prior to an attack taking place!

 Note : Even after an attack starts, there are ways to manipulate Windows Event Logs as a means of defense evasion. This is NOT part of the MS DART/MSTIC blog post, and is mentioned here to bring attention to issues with default audit configuration and monitoring systems for modifications to the audit configuration.
 From the guidance section of the article:
 Enumerate your Windows environment registry hives looking in the HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Schedule\TaskCache\Tree registry hive and identify any scheduled tasks without SD (security descriptor) Value within the Task Key...
