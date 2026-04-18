# Shell Items

- URL: https://windowsir.blogspot.com/2024/10/shell-items.html
- Published: 2024-10-08T20:19:00.005-05:00
- Updated: 2024-10-08T20:19:48.649-05:00
- Labels: none

I ran across a Cyber5W article recently titled, Windows Shell Item Analysis . I'm always very interested in not only understanding parsing of various data sources from Windows systems, but also learning a little something about how others view the topic.
 Unfortunately, there was very little actual "analysis" in the article, an excerpt of which is shown in figure 1.

 I'm not sure I can agree with that statement; tools, be they open source or commercial, tend to be very good at extracting, parsing, and presenting/displaying data, but analyzing that data really depends on the investigative goals, something to which tools are generally not privy.
 But we do see that quite often in the industry, don't we? We'll see something written up, and it will say, "...<tool name> does analysis of...", and this is entirely incorrect. Tools are generally very good at what they do; that is, parsing and displaying information, that an analyst then analyzes, in the context of their investigative goals, as well as other data sources and artifacts.
 The rest of the article doesn't really dig into either the metadata embedded within shell items, nor the analysis of the various artifacts themselves. In fact, there's no apparent mention of the fact that there are different types of shell items, all of which contain different information/metadata.
 I've written quite a bit regarding Windows shell items embedded within various data sources. In fact, looking at the results of a search across this blog, there are more than a few posts. Yes, several of them are from 2013, but that's just the thing...the information still applies, when it comes to shell item metadata. Just because it was written a decade or more ago doesn't mean that it's "out of date" or that it's no longer applicable.
 While it is important to understand the nature and value of various data sources and artifacts, we must also keep in mind that tools do not do analysis, it's analysts and examiners who collect, correlate and analyze data based on their investigative goals.
