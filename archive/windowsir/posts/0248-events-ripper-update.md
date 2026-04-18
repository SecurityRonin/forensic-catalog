# Events Ripper Update

- URL: https://windowsir.blogspot.com/2023/06/events-ripper-update.html
- Published: 2023-06-01T18:15:00.000-05:00
- Updated: 2023-06-01T18:15:06.624-05:00
- Labels: none

As I was using some of the indicators we already had (file and process names) to pivot into the timeline, I saw that I had Security Event Log records from 2020...now, that is weird! After all, it's not often that I see Security Event Log records going back a week or month, let alone 3 years!
 Another indicator was the sessions.pl output from Events Ripper; I had logins lasting 26856 hours (1119 days), and others lasting -16931 hours (over 705 days). Given how the session span is calculated, I knew some was "off" in the Security (and very likely, other) Event Logs, particular the records associated with logon and logoff events.
 I knew something was up, but I also knew that finding the "what was up" was also based largely on my experience, and might not be something a new or more junior analyst would be familiar with. After all, if an analyst was to create a timeline (and I'm seeing everyday that's a pretty big "if"), and if they were pivoting off of known indicators to build context, then how likely would it be that they had the experience to know that something was amiss?
 So, naturally, I wrote an Events Ripper plugin ( timechange.pl ) to pull Security-Auditing/4616 event records from the Security Event Log and display the available information in a nice table. The plugin collects all of these events, with the exception of sub-second time changes (which can be fairly common), and displays them in a table showing the user, the time changed from, the time changed to, and via which process. I wrote the plugin, and it produced an entry on the next investigation...not one that had much impact on what was going on, as the system clock was updated by a few minutes, but this simply shows me how the use of plugins like this can be very valuable for elevating interesting and important artifacts to the analyst for review without requiring that analyst to have extensive experience.
