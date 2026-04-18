# Artifact Tracking: Workstation Names

- URL: https://windowsir.blogspot.com/2024/10/artifact-tracking-workstation-names.html
- Published: 2024-10-26T11:05:00.004-05:00
- Updated: 2024-11-26T13:46:23.353-05:00
- Labels: none

One such indicator is the workstation name, so named based on the indicator as found within Microsoft-Windows-Security-Auditing/4624 event records, indicating a successful login, as well as within Microsoft-Windows-Security-Auditing/4625 and Microsoft-Windows-Security-Auditing/4776 events.
 The value of the workstation name can depend upon the type of incident you're responding to, examining, or attempting to detect earlier in the attack cycle (i.e., moving "left of bang"). For example, many organizations become aware that files have been encrypted and they've been ransomed after those two things have happened. However, for someone to access an infrastructure or network, often they first need to access or log into an endpoint. Depending upon how this is achieved, there may be indicators left in popular Windows Event Logs.
 Huntress analysts have observed an IAB or Akira ransomware affiliate during multiple incidents with initial activity (logins via RDP) originating from a workstation named "WIN-JGRMF8L11HO".
 While investigating a ReadText34 ransomware incident , Huntress analysts found that RDP logins originated from a workstation named "HOME-PC".
 Huntress analysts have also observed the recurrence of workstation names such as "kali" and "0DAY-PROJECT" across multiple incidents. In most (albeit not all) instances, the workstation names associated with the identified malicious activity have not aligned with the naming scheme used by the organization. In fact, in some cases, Huntress analysts have been able to filter through the authentication logs and associate user account names with workstation names and IP addresses to clearly identify the malicious activity; that is, when there is a radical change in the workstation name normally associated with a user account.
 While we can also extract workstation names from Splashtop Event Logs , we're not limited simply to Windows Event Log records. For example, Huntress analysts saw logins via legacy TeamViewer installations ahead of attempts to deploy LockBit3.0 ransomware, and in multiple observed incidents, logins originated from a workstation named WIN-8GPEJ3VGB8U.
 Correlation
 If you've followed my blog for any amount of time, you'll likely have noticed that I'm very interested in file metadata, particularly LNK file metadata . In many cases, LNK/Windows shortcut files will contain a "machine ID" or NetBIOS name of the endpoint on which it was created. This information can be correlated with workstation names, looking for links between usage, campaigns on which the endpoints appear, etc.

 Addendum, 14 Nov
 This blog post was published today, regarding SafePay ransomware attacks observed within customer infrastructures. The table of IOCs contains two workstation names.
 Addendum, 26 Nov
 Others have publicized workstation names as part of their blogs, as well. Last year, Intrinsec published a blog post containing Akira indicators ; the section on lateral movement contains six observed workstation names.
