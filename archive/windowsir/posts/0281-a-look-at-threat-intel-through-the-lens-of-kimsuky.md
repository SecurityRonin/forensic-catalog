# A Look At Threat Intel Through The Lens Of Kimsuky

- URL: https://windowsir.blogspot.com/2024/03/a-look-at-threat-intel-through-lens-of.html
- Published: 2024-03-21T19:28:00.001-05:00
- Updated: 2024-03-21T19:28:33.795-05:00
- Labels: none

Rapid7 recently shared a fascinating post regarding the Kimsuky threat actor group making changes in their playbooks, specifically in their apparent shift to the use of .chm/"compiled HTML Help" files. In the post, the team does a great job of sharing not only likely reasons why there might be a shift to this file format, but also what organizations have been previously targeted by the threat actor group, and why they believe that this is shift in TTPs, rather than a separate group all together.
 Specifically with respect to this threat actor group, if you fall into one of the previously targeted organizations, you'd definitely want to be concerned about the group itself, as well as it's change in tactics.
 Even if you're not in one of the targeted organizations, there's still value in a blog post such as this; for example, are you able to detect .chm files being sent via email, even if they're embedded in archives? Is this something you even want to do?
 How can you protect yourself? Well, the first thing to look at is your attack surface...is there any legitimate business reason for you or your employees to access .chm files? If not, change the default file association from hh.exe to something else, like Notepad. If you want to take it step further, create a text document with a message along the lines of "...you're tried to open a .chm file, please contact an administrator...", and change the default file association to have Notepad open that file. Heck, you can even create a PowerShell script that grabs the name of .chm file, as well as other information (file path, system name, user name, time stamp), and emails it to an administrator, and have that script run instead of actually opening the .chm file. Something like this not only prevents the attack all together, but also provides insight into the prevalence of this type of attack. This may be important to other organizations not targeted by this specific group, as this group is not the only one to rely on .chm files ( see here , also). In fact, the folks from TrustWave shared their findings regarding .chm files from over 6 yrs ago.
 This is not terribly different from similar measures laid out by Huntress not long ago , in that you can use native Windows functionality (which is free) to enable protective measures that make sense for your organization.
 One thing to be aware of, though, is from the section of the blog post that addresses persistence, illustrated in the below image:

 The Run key in the HKCU path does not ensure that the program runs at startup, but instead, as stated in the following sentence, when the user logs in.
 What I would do in an investigation is correlate the Run key LastWrite time with the contents of the Microsoft-Windows-Shell-Core%4Operational Event Log , allowing me to validate when the value was actually written to the key. I would then use this information to then pivot back into the investigative timeline in order to determine how the value ended up being created in the first place.
 Reading through the Rapid7 post, as well as other posts regarding a similar use of .chm files indications that we could have other information available to serve as pivot points and to validate attack timing, through Windows services or scheduled tasks.
 File Metadata
 Something else the Rapid7 post does a good job of presenting/discussing is the .chm file format, and tools you can use to access it without launching any code and infecting yourself. There's information in the blog post regarding not just tools, but also the binary structure of the file format itself. This can all be used to enhance DFIR information regarding an attack, which should then feed threat intel, and provide additional insight to detect and respond to such attacks.
 Also, given what can be embedded in a .chm file, there are other possibilities for metadata and time stamps, as well.
 On the topic of file metadata, the Rapid7 blog post makes reference to the threat actor group's prior use of LNK files as a delivery mechanism, describing several scenarios during which the use of LNK files was observed. I think it would be fascinating to view the LNK metadata across their use; after all, others have done so to great effect.
 Conclusion
 There's a lot of great information in the Rapid7 blog post, and I applaud and greatly appreciate the efforts by the authors, not only performing the research, but also in publishing their findings. However, in the end, this a good deal of threat information, and it's up to the individual reader to determine how to apply it to their environment.

 For me, this is what I like about things like this, and why I appreciate them. Put all the cards...or almost all...on the table, and let me determine who best to utilize or exploit that information within my own infrastructure or processes. A lot of times this is what's best, and we shouldn't consider it to be "threat intelligence".
 Additional Info
 For those interested, here's some additional information about the .chm file format that may be useful in writing tools to parse the binary structure of the file format.
