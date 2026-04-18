# Investigative Scenario, 2024-03-12

- URL: https://windowsir.blogspot.com/2024/03/investigative-scenario-2024-03-12.html
- Published: 2024-03-14T06:48:00.000-05:00
- Updated: 2024-03-14T06:48:33.294-05:00
- Labels: none

Goals
 I try to make sure I have the investigative goals written out where I can see them and quickly refer back to them.
 Per the scenario, our goals are to determine:
 1. How the threat actor accessed the system?
 2. What were their actions on objectives, prior to wiping the system?
 Investigation
 The first thing I'd do is create a timeline from the Software and System hive files, in order to establish a pivot point. Per the scenario, the Registry was backed up "just before the attacker wiped the system". Therefore, by creating a timeline, we can assume that the last entry in the timeline was from just prior to the system being wiped. This would give us a starting point to work backward from, and provide an "aiming stake" for our investigation.
 The next thing I'd do is examine the NTUSER.DAT files for any indication of "proof of life" up to that point. What I'm looking for here is to determine the how of the access; specifically, was the laptop accessed via a means that provided shell- or GUI-based access?

 If I did find "proof of life", I'd definitely check the SAM hive to see if the account is local (not a domain account), and if so, try to see if I could get last login time info, as well as any indication that the account password was changed, etc. However, keep in mind that the SAM hive is limited to local accounts only, and does not provide information about domain accounts.
 Depending upon the version/build of Windows (that info was not available in the scenario), I might check the contents of the BAM subkeys, for some indication of process execution or "proof of life" during the time frame of interest.
 If there are indications of "proof of life" from a user profile, and it's corroborated with the contents of the BAM subkeys, I'd definitely take a look at profile, and create a timeline of activity.
 What we're looking for at this point is:
 1. Shell-, GUI-based access, via RDP, or an RMM?
 2. Network-, CLI-based access, such as via ssh, Meterpreter, user creds/PSExec/some variant, or a RAT
 Shell-based access tends to provide us with a slew of artifacts to examine, such as RecentApps, RecentDocs, UserAssist, shellbags, WordWheelQuery, etc., all of which we can use to develop insight into a threat actor actor, via not just their activity, but the timing thereof, as well.
 If there are indications of shell-based access, we check the Registry to determine if RDP was enabled, or if there were RMM tools installed, but without Windows Event Logs and other other logs, we won't know definitively which means was used to access the laptop. Contrary to what some analysts seem to believe, the TSClients subkeys within the NTUSER.DAT hive do not show systems that have connected to the endpoint, but rather which systems were connected to from the endpoint.

 Something else to consider is if the threat actor had shell-based access, and chose to perform their actions via a command prompt, or via Powershell, rather than navigating the system via the Explorer shell and double-clicking files and applications. As we have only the backed up Registry, we wouldn't be able to examine user's console history, nor the Powershell Event Logs.
 However, if there are no indications of shell-based access, and since we only have the Registry and no access to any other log files from the endpoint, it's going to likely be impossible to determine the exact means of access. Further, if all of the threat actor's activity was via network-based/type 3 logins to the laptop, such as via Meterpreter, or PSExec,
 It doesn't do any good to parse the Security hive for the Security Event Log audit policy, because we don't have access to the Windows Event Logs. We could attempt to recover them via record parsing of the image, if we had a copy of the image.
 I would not put a priority on persistence; after all, if a threat actor is going to wipe a system, any persistence they create is not going to survive, unless the persistence they added was included in a system-wide or incremental backup, from which the system is restored. While this is possible, it's not something I'd prioritize at this point. I would definitely check autostart locations within the Registry for any indication of something that might look suspicious; for example, something that may be a RAT, etc. However, without more information, we wouldn't be able to definitively determine if (a) if the entry was malicious, and (b) if it was used by the threat actor to access the endpoint. For example, without logs, we have no way of knowing if an item in an autostart location started successfully, or generated an error and crashed each time it was launched. Even with logs, we would have no way of knowing if the threat actor accessed the laptop via an installed RAT.
 Something else I would look for would be indications of third-party applications added to the laptop. For example, LANDesk used to have a Software Monitoring module, and it would record information about programs executed on the system, along with how many times it was launched, the last time it was launched, and the user name associated with the last launch.
 Findings
 So, where do we stand with our goals? I'd say that at the moment, we're at "inclusive" because we simply do not have enough information to go on. There is no memory dump, no other files collected, no logs, etc., just the backed up Registry. While we won't know definitively how the threat actor was able to access the endpoint, we do know that if access was achieved via some means that allowed for shell-based access, we might have a chance at determining what actions the threat actor took while they were on the system. Of course, the extent to which we'd be able to do that also depends upon other factors, including the version of Windows, the software "load" (i.e., installed applications), actions taken by the threat actor (navigating/running apps via the Explorer shell vs. command prompt/Powershell). It's entirely possible that the threat actor accessed the endpoint via the network, through a means such as Meterpreter, or there was a RAT installed that they used to access the system.
