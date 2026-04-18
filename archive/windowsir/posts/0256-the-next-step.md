# The Next Step

- URL: https://windowsir.blogspot.com/2023/07/the-next-step.html
- Published: 2023-07-23T14:56:00.003-05:00
- Updated: 2023-07-23T14:56:21.643-05:00
- Labels: none

A lot of times, we'll run across something or read something really very profound and valuable, something that opens our eyes and makes us go, "oh, wow", and impacts us enough that it changes the way we do things. I can say that of a number of blogs posts and articles, by various authors, going back quite a few years. And then later, being able to add additional information to the original findings, information that may not have been recognized or available at the time, can really aid in our investigations.
 A while back, Krz shared this blog post , detailing how default settings for Scheduled Tasks include the "stop if the computer switches to battery power" setting by default, and the impact that setting can have on a forensic investigation. For example, PCI forensic investigations require the analyst to specifically address the "window of compromise", and malware that persists via a Scheduled Task will be impacted by whether or not the system in question was running on battery power or not. Krz's previous blog post addressed using the SRUM database to determine the battery charge level, and in that post, Krz linked to a tool he'd written to extract and display that data.
 I ran across something recently that I wanted to use to build on Krz 's excellent work; from the Microsoft-Windows-UniversalTelemetryClient%4Operational.evtx Event Log, we see a UniversalTelemetryClient/60 event record that lets us know if the system was on battery power or not, as illustrated in figure 1:
