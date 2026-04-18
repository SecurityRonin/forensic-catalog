# RegRipper

- URL: https://windowsir.blogspot.com/2025/07/regripper.html
- Published: 2025-07-03T14:05:00.000-05:00
- Updated: 2025-07-03T14:05:25.952-05:00
- Labels: none

The awesome folks over at Cyber Triage recently published their 2025 Guide to Registry Forensic Tools , and being somewhat interested in the Windows Registry, I was very interested to take a look. The article is very well-written, and provides an excellent basis for folks who are new to DF/IR work, and new to the Windows Registry.

 Now, why did I want handling the transaction logs to be a purposeful, intentional decision? If you've ever processed the transaction logs, you'll notice that when you apply the transaction logs to a Registry hive, the hive file itself remains the same size; keys and values are updated or added, but the file itself remains the same size, even through the hash changes. This means that for the resulting hive file, unallocated space within the hive file is overwritten...deleted keys and values, and possibly even slack space, are overwritten.

 So, to be clear, if you're at all interested in data deleted from the Registry, and you understand that Registry hive files themselves contain unallocated space, and that values can contain slack space, you might not want to just automatically apply transaction logs. Depending upon the timing of the incident and your investigative goals, you may want to first fully parse the hive file, before applying the transaction logs and applying the same parsing process a second time. Sort of a "before" and "after" snapshot of the hive.
 Neither RegRipper v3.0 nor RegRipper v4.0 processes the transaction logs; however, both are open source, and you can write your own plugins, or modify current plugins in any way you choose, such as changing the output format. For example, both versions include multiple plugins that output in 5-field TLN format (for inclusion directly into a timeline events file), and v4.0 has several plugins that output in JSON format . I get it, though...the TLN output is meaningless if you're not creating timelines.
 Also, with RegRipper v4.0, I got Yara working within RegRipper , meaning that you can run Yara rules against Registry values, right from RegRipper.
 Finally, both versions include plugins to do various parsing, such as parsing unallocated space, parsing Registry value sizes, locating EXE/PE files in Registry values , etc.
