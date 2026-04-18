# Analysis Process

- URL: https://windowsir.blogspot.com/2024/10/analysis-process.html
- Published: 2024-10-15T13:08:00.000-05:00
- Updated: 2024-10-15T13:08:49.584-05:00
- Labels: none

My analysis process, laid out in books like " Investigating Windows Systems ", is, essentially:

 1. Document investigative goals. These become the basis for everything you do in the investigation, including the report.

 Always start with the goals, and always start documentation by having those goals right there at the top of your case notes file. When I was active in DFIR consulting, I'd copy the investigative goals into the Executive Summary of the report, and provide 1-for-1 answers. So, three goals, three answers. After all, the Executive Summary is a summary for executives, meant to stand on it's own.

 2. Collect data sources.

 This one is pretty self-explanatory, and very often based on your response process (i.e., full images vs "triage" data collections). Very often, collection processes will include the least amount of data extracted from a system for the biggest impact, based upon the predominance of business needs, leaving other specific sources for later/follow-on collection, if needed.

 3. Parse, normalize, decorate, enrich those data sources.

 Basically, create a timeline, from as many data sources as I can or makes sense, based on my investigative goals. Easy-peasy.

 Timelines are not something left to the end of the investigation, to be assembled manually into a spreadsheet. Rather, creating a timeline as a means of initiating an investigation provides for much needed context.

 4. Identify relevant pivot points.

 RegRipper and Events Ripper are great tools for this step. Why is that? Well, within the Registry, often items of interest are encoded in some manner, such as binary, hex, ROT-13, or some folder or other resource represented by a GUID; many of the RegRipper plugins extract and display that info in human-readable/-searchable format. So, running RegRipper TLN plugins to incorporate the data into a timeline, and then run "regular output" plugins to develop pivot points.

Events Ripper is great for extracting items of interest from events files with (hundreds of) thousands of lines.

 5. Identify gaps, if any, and loop back to #2 .

 Based on the investigative goals, what's missing? What else do you need to look for, or at? You may already have the data source, such as if you need to look for deleted content in Registry hives,

 6. Complete when goals are met, which includes being validated.

 An issue we face within the industry, and not just in DFIR, is validation. If a SOC analyst sees a "net user /add" command in EDR telemetry, do they report that a "user account was created" without (a) checking the audit configuration of Security Event Log, and (b) looking for Security-Auditing event records that demonstrate that a user account was created? If it was a local account, is the SAM checked?

 Or, if msiexec.exe is seen (via EDR telemetry) running against an HTTP/HTTPS resource, is the Application Event Log checked for MsiInstaller events?

 My point is, are we just saying that something happened, or are we validating via the available data sources that it actually happened?

 7. Anything "new" gets baked back in

 The great thing about timelines and other tools is that very often, you'll find something new, something you hadn't seen before, and was relevant (or could be) to your investigation. This is where most of the Events Ripper plugins have originated; I'll see something "new", often based on an update to Windows, or some installed application, and I'll "bake it back into" the process by creating a plugin.

 Yes, documenting it is a good first step, but adding it back into your automation is taking action. Also, this way, I don't have to remember to look for it...it's already there.

 For example, several years ago, another analyst mentioned seeing something "new" during a response; looking into it, this new thing was a Microsoft-Windows-TaskScheduler/706 event record, so once I got a little more info about it, and dug into the investigation myself just a bit, I added it to eventmap.txt . After that, I never had to remember to look for it, and I had the necessary references to support the finding already documented.
