# File Formats

- URL: https://windowsir.blogspot.com/2022/04/file-formats.html
- Published: 2022-04-25T19:54:00.000-05:00
- Updated: 2022-04-25T19:54:05.375-05:00
- Labels: none

Having an understanding of file formats is an important factor in DFIR work. In particular, analysts should understand what a proper file using a particular format should look like, so that they can see when something is amiss, or when the file itself has been manipulated in some manner.
 Understanding file formats  goes well beyond understanding PE file formats and malware RE. Very often, various Microsoft file formats include data, or metadata (defined as "data about data") that can be mined/parsed, and then leveraged to tremendous effect, furthering overall analysis and intelligence development, often across multiple cases and campaigns.
 LNK
 Windows shortcut, or LNK files, have been covered extensively in this blog , as well as other blogs, in addition to having been well documented by MS . Suffice to say, LNK files can be leveraged by both good guys and bad guys, and if bad guys leverage them, so should the good guys...after all, the bad guys sending you an LNK file created in their environment is essentially just "free money", particularly if you're in CTI.
 For example, the GOLDBACKDOOR report shows us a threat actor that sends an LNK file to their target, in a zip archive. So, the threat actor develops the LNK file in their environment, and sends that LNK file with all of it's metadata to the target. Now, as a DFIR analyst, you may have a copy of a file created within the threat actor's environment, one that contains information about their system(s). Why not take advantage of that metadata to develop a more meaningful threat intel picture?
 Analysis of LNK files is similar to being an EOD tech (I would imagine)...you're looking at the construction of a "device", noting production mechanisms (based on tooling) as well as unique items that allow you to tie or "attribute" the LNK file in some manner. You can then leverage sites such as VirusTotal (via a retro-hunt) and populate your own MISP instance to build out a larger, more contextual threat intelligence picture. For example, consider the LNK file delivered as part of the Quantum ransomware campaign discussed in this TheDFIRReport article; the article provides an image of the metadata extracted from the LNK file. The machine ID, MAC address, and volume serial number (not shown) could be put into a Yara rule and submitted as a VirusTotal retro-hunt, providing insight into other instances or campaigns were LNK files with the same values were employed. You can also tighten or loosen the aperture of your retro-hunt by adding or removing values within the Yara rule.
 Yet another example of how LNK metadata can be used...and likely the best example of this sort of data exploitation available thus far...can be seen in this Mandiant blog post from 2018 . The post addresses differences between APT29 campaigns from 2016 and 2018, with references to differences in LNK files shows in figures 5 and 6. Reading through the article, you can see where the Mandiant team leveraged the data they had available to develop insights about the actor's campaigns.
 OLE
 OLE, or "object linking and embedding" (aka, "structured storage") is a file format most often associated with older versions of MS Office. As such, when MS transitioned the world to the "new" MSOffice file format, many likely thought, "okay, I'll never see that file format again." Oh, how wrong we were! The OLE format is used within a number of other files, including:
 Files
 Automatic JumpLists - all but one embedded stream consists of an LNK "file"
 MSI files
 Sticky Notes
 Other files , some of which are application-dependent
 We can look to a variety of tools to meet our needs in parsing these files:
 OLE Parsing Tools
 olefile
 MiTeC SSV
 ripOLE

 For specifically parsing/working with MSI files, I'm told that folks use tools such as InstEdit , and orca.exe from MS .
 Metadata that may be present in the document structure can be leveraged or exploited in a manner similar to LNK files, or better yet, really leveraged by combining it with what's found in LNK files. For example, the LNK file in the GOLDBACKDOOR report reportedly downloads a decoy .doc file, meaning that the metadata from the LNK file has a direct link to the metadata found in the .doc file.
 Registry
 At this point, the Windows Registry file format is well-understood, and documented ( here, by Maxim Suhanov ). As a result, we know how to parse it, but much like other file structures, we (as a community and industry) regularly fall short in truly exploiting the file format to our advantage.
 Registry keys have embedded time stamps, which as Lina L . astutely and articulately described in her blog post , can be manipulated. Time stamps are also visible in value data (albeit NOT in the structure of values) as strings, as binary data, or embedded within binary data streams. All of these can be used to further analysis, including possibly even to identify key LastWrite time manipulation (very much depending upon how it's done).
 For example, a recent TheDFIRReport write-up on the Quantum ransomware indicates that when the user double-clicks the ISO file, artifacts of the ISO file being mounted on the system appear in the Windows Event Log. Okay, great...but does the drive letter assignment also appear in the MountedDevices key in the System hive? When the user double-clicks the LNK file embedded in the ISO file, is that action reflected in the user's UserAssist key?
 Aside from the "normal" Registry hive files we're all familiar with...Software, System, SAM, Security, NTUSER.DAT, USRCLASS.DAT...other files on the system follow the same file format. This means that all of the tools we use to parse the 'usual suspects' can also be leveraged against these other files, which include BBI, DEFAULT, and ELAM in the system32\config folder, the AmCache.hve file, and the settings.dat files associated with MS Store apps (i.e., in the %user%\AppData\Local\Packages\windows.immersivecontrolpanel_cw5n1h2txyewy\Settings folder, etc.)
 Tools
 In 2008, Jolanta Thomassen created the tool regslack.pl , to parse deleted data from hive files as part of her thesis work. Since then additional tools have been created for parsing this information, not the least of which includes the del.pl and slack.pl RegRipper plugins.
