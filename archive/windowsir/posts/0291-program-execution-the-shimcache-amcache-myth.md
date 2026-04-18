# Program Execution: The ShimCache/AmCache Myth

- URL: https://windowsir.blogspot.com/2024/11/program-execution-shimcacheamcache-myth.html
- Published: 2024-11-26T12:53:00.002-05:00
- Updated: 2024-11-26T12:53:53.162-05:00
- Labels: none

I recently saw another LinkedIn post from someone supporting and sending readers to a site that was reportedly started using the SANS DFIR poster as a reference. As illustrated in figure 1, this site references the ShimCache artifact as providing evidence of program execution, and does the same for the AmCache artifact, as well.

 Now, while yes, it is true that these artifacts can provide evidence of program execution, that is not always the case, and this needs to be understood throughout the community.
 What I'm going to do with this blog post is provide the resources to show why these artifacts do not solely provide evidence of "program execution", so that others in the community can reference these resources.
 ShimCache
 Mandiant's article regarding ShimCache includes the statement illustrated in figure 2.

 Notice the highlighted section in figure 2, which states, "... that were not actually executed .", and then goes on to say that entries can be added if a user browses to a folder. The ShimCache value is written to the System hive, and does not provide a reference to the user who may have browsed to a folder. As such, this is something that would need to be resolved through user profile analysis, via artifacts such as JumpLists, recently accessed files, shellbags, etc.
 However, the important thing to understand here is that this reductionist approach of saying that ShimCache is evidence solely of program execution is incorrect.
 We also need to remember that the ShimCache is written to the Registry value at shutdown; this is trivial to demonstrate via a timeline.
 AmCache
 Blanche Lagney's Analysis of AmCache v2 paper is the definitive reference for all things AmCache . The research is thorough, and presented in a manner that, while it does require some reading to address specific questions, should remove all doubt as to the value of the artifact on specific Windows builds.
 If you're looking at just the AmCache.hve file as an artifact to determine program execution, you're going to need to find the closest match to the Windows build and libraries, based on the keys you're looking at, to better understand the nature of the artifacts you're seeing.
 Analysis Process
 The key here is not to try to memorize the "value" of individual artifacts in isolation, but to have an analysis process where multiple data sources and artifacts are viewed together, so that through this process you can 'see' the context of the events in question. For example, when it comes to program execution, we might look to JumpLists, Security (if configured) and/or Sysmon (if installed) Event Logs, UserAssist entries, and on workstation platforms, Prefetch files. On Windows 11, we might also look to data within the PCA folder . To validate that a program executed, we might look to impacts in the Registry and/or file system, or to the Application Event Log to determine if the program generated an error or crashed.
 For teaching/instructional purposes, it would be extremely valuable to start by describing one data source, such as the file system, and then show how that data source can be viewed via a timeline. Then, add another data source, such as the Windows Event Log or the Registry, and add that data source to the timeline. When discussing the Registry (as well as the ShimCache and AmCache artifacts), it will be important for analysts to understand the value of time-based metadata (key LastWrite times), as well as time-based data embedded within individual values, all of which can help better address analysis categories such as "program execution".
 Conclusion
 While it is valuable to have an understanding of various artifacts, the most important takeaway from this article is that analysts should not consider artifacts in isolation during an investigation, but should instead look to multiple data sources and artifacts, viewed together, to determine the nature and context of events in question.
