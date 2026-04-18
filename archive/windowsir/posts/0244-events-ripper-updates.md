# Events Ripper Updates

- URL: https://windowsir.blogspot.com/2023/05/events-ripper-updates.html
- Published: 2023-05-05T05:59:00.000-05:00
- Updated: 2023-05-05T05:59:54.707-05:00
- Labels: none

One of the ways I do this is by creating simple plugins for Events Ripper , a proof-of-concept tool for "mining" Windows Event Log data for pivot points that can be applied to analysis, and in particular timeline analysis. Events Ripper uses the events file, the intermediate step between normalizing Windows Event Log events into a timeline, extracting pivot points and allowing me to build the picture of what happened, and when, a great deal faster than doing so manually.
 The recently created or updated plugins include:
 sec4797.pl
 Check for "Microsoft-Windows-Security-Auditing/4797" events, indicating that a user account was checked for a blank password. I'd never seen these events before, but they popped up during a recent investigation, and helped to identify the threat actor's activity, as well as validate the compromised account they were using.
 filter.pl
 "Microsoft-Windows-Security-Auditing/5156", and /5158 events; this plugin output is similar to what we see with ShimCache parsers, in that it lists the applications for which the Windows Filtering Platform allows connections, or allows to bind to a local port, respectively. Similar to "Service Control Manager" events illustrating a new service being installed, this plugin may show quite a few legitimate applications, but it's much easier to go through that list and see a few suspicious or malicious applications than it is to manually scroll through the timeline. Searching the timeline for those applications can really help focus the investigation on specific timeframes of activity.
 defender.pl
 Windows Defender event IDs 1116, 1117, 2051, and 5007, all in a single plugin, allowing us to look for detections and modifications to Windows Defender. Some modifications to Windows Defender may be legitimate, but in recent investigations, exclusions added to Windows Defender have provided insight into the compromised user account, as well as the folders the threat actor used for staging their tools.
 msi.pl
 Source "MsiInstaller", with event IDs 11707 (successful product installation), 11724, and 1034 (both successful product removal).
 scm.pl
 Combined several event IDs (7000, 7009, 7024, 7040, and 7045) events, all with "Service Control Manager" as the source, into a single plugin. This plugin is not so much the result of recent investigations, as it is the desire to optimize validation; a service being created or installed doesn't mean that it successfully runs each time the system is restarted.
 appissue.pl
 Combined "Application Hang/1002", "Application Error/1000", and "Windows Error Reporting/1001" events into a single plugin, very often allowing us to see the threat actor's malware failing to function.
 Each of the new or updated plugins is the result of something observed or learned during recent investigations, and allow me to find unusual or malicious events to use as pivot points in my analysis.
 We can do the same things with RegRipper plugins, Yara or Sigma rules, etc. It simply depends upon your framework and medium.
