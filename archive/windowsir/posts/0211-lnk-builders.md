# LNK Builders

- URL: https://windowsir.blogspot.com/2022/09/lnk-builders.html
- Published: 2022-09-03T06:37:00.002-05:00
- Updated: 2022-09-19T09:50:32.221-05:00
- Labels: builder, generator, LNK, toolmarks

I've blogged a bit...okay, a LOT...over the years on the topic of parsing LNK files, but a subject I really haven't touched on is LNK builders or generators. This is actually an interesting topic because it ties into the cybercrime economy quite nicely. What that means is that there are "initial access brokers", or "IABs", who gain and sell access to systems, and there are "RaaS" or "ransomware-as-a-service" operators who will provide ransomware EXEs and infrastructure, for a price. There are a number of other for-pay services, one of which is LNK builders.
 In March, 2020, the Checkpoint Research team published an article regarding the mLNK builder , which at the time was version 2.2. Reading through the article, you can see that the building includes a great deal of functionality, there's even a pricing table. Late in July, 2022, Acronis shared a YouTube video describing how version 4.2 of the mLNK builder available.
 In March, 2022, the Google TAG published an article regarding the "Exotic Lily" IAB , describing (among other things) their use of LNK files, and including some toolmarks (drive serial number, machine ID) extracted from LNK metadata. Searching Twitter for "#exoticlily" returns a number of references that may lead to LNK samples embedded in archives or ISO files.
 In June, 2022, Cyble published an article regarding the Quantum LNK builder , which also includes features and pricing scheme for the builder. The article indicates a possible connection between the Lazarus group and the Quantum LNK builder; similarities in Powershell scripts may indicate this connection.
 In August, 2022, SentinelLabs published an article that mentioned both the mLNK and Quantum builders. This is not to suggest that these are the only LNK builders or generators available, but it does speak to the prevalence of this "*-as-a-service" offering, particularly as some threat actors move away from the use of "weaponized" (via macros) Office documents, and toward the use of archives, ISO/IMG files, and embedded LNK files.
 Freeware Options
 In addition to creating shortcuts through the traditional means (i.e., right-clicking in a folder, etc.), there are a number of freely available tools that allow you to create malicious LNK files. However, from looking at them, there's little available to indicate that they provide the same breadth of capabilities as the for-pay options listed earlier in this article. Here's some of the options I found:
 lnk-generator (circa 2017)
 Booby-trapped shortcut (circa 2017) - includes script
 LNKUp (circa 2017) - LNK data exfil payload generator
 lnk-kisser (circa 2019) - payload generator
 pylnk3 (circa 2020) - read/write LNK files in Python
 SharpLNKGen-UI (circa 2021) - expert mode includes use of ADSs ( Github )
 Haxx generator (circa 2022) - free download
 lnkbomb - Python source, EXE provided
 lnk2pwn (circa 2018) - EXE provided
 embed_exe_lnk - embed EXE in LNK, sample provided
 Next Steps
 So, what's missing in all this is toolmarks ; with all these options, what does the metadata from malicious LNK files created using the builders/generators look like? Is it possible that given a sample or enough samples, we can find toolmarks that allow us to understand which builder was used?

 Consider this file , for example, which shows the parsed metadata from several samples (most recent sample begins on line 119). The first two samples, from Mandiant's Cozy Bear article , are very similar; in fact, they have the same volume serial number and machine ID. The third sample, beginning on line 91, has a lot of the information we'd look to use for comparison removed from the LNK file metadata; perhaps the description field could be used instead, along with specific offsets and values from the header (given that the time stamps are zero'd out). In fact, besides zero'd out time stamps, there's the SID embedded in the LNK file, which can be used to narrow down a search.
 The final sample is interesting, in that the PropertyStoreDataBlock appears to be well-populated (unlike the previous samples in the file), and contains information that sheds light on the threat actor's development environment.
 Perhaps, as time permits, I'll be able to use a common executable (the calculator, Solitaire, etc.), and create LNK files with some of the freeware tools, noting the similarities and differences in metadata/toolmarks. The idea behind this would be to demonstrate the value in exploring file metadata, regardless of the actual file, as a means of understanding the breadth of such things in threat actor campaigns.
