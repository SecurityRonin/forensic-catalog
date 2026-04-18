# New Events Ripper Plugins

- URL: https://windowsir.blogspot.com/2023/04/new-events-ripper-plugins.html
- Published: 2023-04-24T20:07:00.001-05:00
- Updated: 2023-04-24T20:07:32.091-05:00
- Labels: none

The mssql.pl plugin primarily looks for MS SQL failed login events in the Application Event Log. I'd engaged in a response where we were able to validate the failed login attempts first in the MS SQL error logs, but then I learned that the events are also listed in the Windows Event Log, specifically the Application Event Log, and I wanted to provide that insight to the analyst.
 The plugin lists the usernames attempted and the frequency of each, as well as the source IP address of the login attempts and their frequency. In one instance, we saw almost 35000 failed login attempts, from 4 public IP addresses, three of which were all from the same class C subnet. This not only tells a great deal about the endpoint itself, but also provides significant information that the analyst can use immediately, as well as leverage as pivot points into the timeline. The plugin does not yet list successful MS SQL logins because, by default, that data isn't recorded, and I haven't actually seen such a record.
 The plugin also looks for event records indicating settings changes, and lists the settings that changed. Of specific interest is the use of the xp_cmdshell stored procedure.
 So, why does this matter? Not long ago, AhnLab published an article stating that they'd observed attacks against MS SQL servers resulting in the deployment of Trigona ransomware.

 The scm7000.pl plugin locates "Service Control Manager/7000" event records, indicating that a Windows service failed to start. This is extremely important when it comes to validation of findings ; just because something (i.e., something malicious) is listed as a Windows service does not mean that it launches and runs every time the endpoint is restarted. This is just as important to understand, alongside Windows Error Reporting events, AV events, application crash events, etc. This is why we cannot treat individual events or artifacts in isolation; events are in reality composite objects, and provide (and benefit from) context from "nearby" events.
 The scm7024.pl plugin looks for " Service Control Manager/7024 " records in the System Event Log, which indicate that a service terminated.
 The apppopup26.pl plugin looks for "Application Popup/26" event records in the Application Event Log, and lists the affected applications, providing quick access to pivot points for analysis. If an application of interest to your investigation is listed, the simplest thing to do is pivot into the timeline to see what other events occurred "near" the event in question. Similar to other plugins, this one can provide indications of applications that may have been on the system at one point, and may have been removed.
 Events Ripper has so far proven to be an extremely powerful and valuable tool, at least to me. I "see" something, document it, add context, analysis tips, reference, etc., and it becomes part of an automated process. Sharing these plugins means that other analysts can benefit from my experiences, without having to have ever seen these events before.
 The tool is described here , with usage information available here , as well as via the command line.
