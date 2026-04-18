# Windows Incident Response

- URL: https://windowsir.blogspot.com/2011/12/jump-list-analysis.html
- Published: unknown
- Updated: unknown
- Labels: none

Windows Incident Response
 The Windows Incident Response Blog is dedicated to the myriad information surrounding and inherent to the topics of IR and digital analysis of Windows systems. This blog provides information in support of my books; "Windows Forensic Analysis" (1st thru 4th editions), "Windows Registry Forensics",
as well as the book I co-authored with Cory Altheide, "Digital Forensics with Open Source Tools".
 Wednesday, December 28, 2011
 Jump List Analysis

 19 comments:
 Harlan, you said "I wrote some code to parse these files. This code consists of two Perl modules, one for parsing the basic structure of the *.automaticDestinations-ms Jump List files, and the other to parse LNK streams."

 Trying to compare the tools (Jumplister, WFA, and SSV) to see if the tools are accurate and to provide a research baseline. Very interested how you went about parsing it, but I cannot seem to find the code modules online at your usual code location. Have you released them? Thanks for a great post.
 Trying to compare the tools (Jumplister, WFA, and SSV) to see if the tools are accurate and to provide a research baseline.

 How would you determine accuracy? If one tool lists a value as decimal and the other presents it at hex, is one more accurate than another? If a value is located at an offset, and one tool displays it and another doesn't, is the first tool more accurate?

 I have to say, I don't really follow what you're trying to achieve. I think that with the available tools, there's already more than enough for a "research baseline".

 Very interested how you went about parsing it...

 By following the binary format specifications available from MS.

 ...but I cannot seem to find the code modules online at your usual code location. Have you released them?

 No, I haven't released the code yet...I may do so after I add some POD to the modules. I will likely release versions of the modules, and then maintain and keep developing versions with more debugging capabilities (dumping structures, etc.). As I mentioned before, however, these are modules...what gets displayed and the way it gets displayed is totally dependent upon the script you write.

 Also, I still have some of the same concerns as our last conversation; specifically:

 1. I'm not sure that the vast majority of analysts really understand Jump Lists at this point, so what would be the point of releasing yet another tool, particularly one like this, if the analyst doesn't understand enough about the Jump Lists to understand what's being displayed?

 2. I've seen too many times how someone will figure that something doesn't work (usually through operator error), and post to a list but NOT say anything to the tool author. I've seen folks say that they couldn't get SIFT to work, and my response has always been, "...did you contact Rob?", to which they usually respond, "no".

 Upon further reflection, add to that some additional thoughts:

 3. I don't want the code absorbed into another project, after which I have no access to any of the research you mention, or to continued development.

 4. I'm aware of tools that parse Jump Lists, including the DestList stream, but the author of at least one tool does not give credit (by name) for their source for the DestList structure. I've already published a great deal of my Jump List research on my blog as well as to the ForensicsWiki , and there doesn't seem to be any further development of that research, at least none that's available publicly.

 5. RegRipper is used in several training courses (some colleges, and in other for-pay training courses), and for whatever reason, I can't even see/review the materials of those courses, either fully or in part.

 From what I've seen with respect to other open source tools that I've released, as well as the information I've released on Jump Lists, all of these materials are being used within the community. That's great. However, what seems to be happening is that at least some of those who are using this information are benefiting from it, but very little is making it back into the community.
 One additional thought related to determining if a file is copied or moved in a filesystem. You can tell if a file is copied on moved on Win7 machines simply by looking at the MACB times. The information should match up with what you uncovered in the LNK files.

 For a quick reference, the information can be found here: http://computer-forensics.sans.org/blog/2010/04/12/windows-7-mft-entry-timestamp-properties/
 Yeah truly understand how you feel. I have felt that way on similar projects and research that show up in other locations. I have seen the SIFT workstation in many locations... the great thing is that many of them asked to use it in those locations. Others did not. Still thank you for all the hard work you are trying to do, just would have liked to see the PERL code as that is what I'm most familiar in.

 Im glad that you and I were able to sit down for lunch several months ago I was able to show you the SANS books and each slide where we mention Regripper in the SANS courses earlier this year. Sorry others have not had that opportunity.
 Rob,

 Thanks for that opportunity. Too bad I never saw the NDA that we discussed.

 As we discussed in our last exchange, I did reach to Kristinn and make an offer of the code I had available at the time, and never heard back. I know folks get busy...we all do. I've also offered to assist with other items on his roadmap...and I don't see Jump Lists on that roadmap at the moment.

 Any thoughts on the topics of accuracy and research that you mentioned in your first comment?
 I cannot speak for Kristinn, but I'd try again... he has had a lot going on. Kristinn himself and his family from Iceland to start a new job in California. That is a lot if you ask me.

 As for the accuracy, yes I have seen multiple tools parse structures incorrectly. Even structures as simple as index.dat files routinely had misinterpreted data. Others assume the parsing is correct and they blindly build that into their own tools testing only to ensure that it matches the output of the other tool. So accuracy would be to perform my own tests to ensure that what the tool spits out the right time and the right artifacts knowing exactly when and how I created them. Seeing which structures you pulled your data from makes it easier as I personally know PERL.
 First, thank you for commenting on my rather small part in a team effort to bring jump lists into a mainstream discussion. I agree that their importance remains unrecognized by many of my peers. It almost never comes up on most of the lists to which I subscribe.

 Concerning validation, I can say that the one case that you were kind enough to parse for me produced entirely accurate results. I realize that it was a while ago and involved only one data set, but I imagine that your code hasn't varied with respect to that function. I still want to test a little further with respect to the MAC times in the numbered streams, as to whether, or how closely, they parallel the target MACs. However, the importance of that question hasn't been significant in regard to my particular caseload. I've found variances, but that fact could be logical after further study.

 There's little available when it comes to parsers. I pretty much use Mark Woan's and XWF for comparison, and I sometimes triple check with SSV. Mark has been extraordinarily gracious in adopting suggestions, such as simply offering an option to list the stream numbers in hex or decimal. I haven't had a need for a timeline-ready (in the format discussed here) export. Going back through shadows to determine frequency seems like a daunting task, but that's a great point. Often, we can achieve similar results by carving index records, but that depends on whether the desired records are recoverable.

 (On an unrelated subject, thanks very much for you card! It was very thoughtful.)
 Harlan,

 Thanks for your research and this post. There's a wealth of information in the post and the references you've listed. Interesting about the MAC address in lnk files. I wonder where that originated. I went back and looked at the old MS-SHLLINK.pdf I'd downloaded almost 2 years ago when I was porting your lslnk.pl to Metasploit and it says nothing about MAC addresses, nor does the latest version of that document. I know it's in FTK 1.8's parser. I see Parsonage's paper, which I hadn't read until tonight, also mentions the MAC address.
 Jimmy,

 There's little available when it comes to parsers.

 It depends on your perspective. MiTeC's SSV + a LNK file parser make a good combination, and Mark Woan's JumpLister does a great job of parsing the data. My own tools are really more for an analyst who understands Jump Lists and what's available within them.

 Going back through shadows to determine frequency seems like a daunting task...

 Not at all. Corey Harrell has done a fantastic job of posting batch scripts that he uses to automate the ripping of data out of VSCs...having a tool that can go back through the VSCs and parse previous versions of a specific Jump List file would be a pretty simple way of doing this.

 Dave,

 Be sure to take a look at RFC 4122, as well.

 Thanks.
 I did mention SSV and Jumplister, though I don't think it's very efficient to use SSV+LNK parser when one tool will meet my needs. I do, however, have a copy of Paul Sanderson's LinkAlyzer, which does a very thorough job of presenting the data.

 I haven't tried Corey's ripper. However, as you know, I have played with ProDiscover in regard to shadows and jump lists. It actually would be a rather handy way to extract anything that I need. In fact, if PD brings its jump list parser up to the level of your and Mark's tools, it would be remarkably fast and easy. (I'm awaiting the next release that should fix a few issues.)
 Jimmy,

 That's an interesting analysis technique...let's say that you were able to parse the contents of a Jump List's DestList stream, and output the contents in TLN format. If you were interested in a particular file (say, a PDF or MSWord document), you could run that tool against the "current" Jump List file and pipe the output through find to see just the file you were interested in.

 Then, using the technique Corey's discussed, you could automatically mount available VSCs, and run the same command against the previous versions of the Jump List file. This would (hopefully) show you all (or just some) of the previous times that the file had been accessed.

 The same could be done with other tools.
 @Harlan,

 You're comment about analysts not knowing about Jump Lists because they don't encounter Windows 7 systems rings true for my neck of the woods. Most analysts I talk to locally deal with organizations who haven't upgraded to 7 yet (they also skipped the Vista upgrade). I even fall into the same category of not encountering too many 7 boxes. However, I think that this will change over the next year since organizations are going to be upgrading their enterprises from XP. Thanks for putting this post together; it contains a wealth of information. It's a great reference for not only learning but should come in handy when parsing jump lists.

 @Jimmy

 As Harlan already mentioned parsing the jump lists stored in volume shadow copies would be fairly easy. The process would work as you described; run a command to parse the lists from a mounted image followed by running the same command against any VSCs of interest. A cool thing is that you could parse other artifacts at the same time such as registry keys and Windows link files. Then you could either grep all the data looking for specific file types or review the output for leads. It would only take a minute or two to put together a batch script to do this. At some point during the month I'm putting together a post explaining the technique and will release a few batch scripts showing the capability.
 Corey,

 I think you're right about analysts and access to Win7 boxes. In some cases, I've chatted with analysts who have analyzed Windows 7 boxes, but the cases involved malware, not specifically looking at user activity. I also agree with you that as organizations transition from XP, everyone will be seeing more Win7 systems come across their work bench.
 Here in the hinterlands, most our boxes are Win 7 or leftover Vista. That's probably because we rarely see enterprise machiines, so the stuff is off-the-shelf from Staples. Corey, thanks for the tips, and I'll look forward to your post.
 Hi Harlan,

 I came across a handy trialware utility called "Jumplist File Extract 1.2". Life is soo easy when everything's "automated" lol :)
 Does it parse the DestList stream?
 I think yes. It parses customdestination and automaticdestination and provides the results under the following columns: Long Name, Short Name, Modified, Created, Accessed, Attributes, File Size, Arguments, File Size, Title, Description, Vol. GUID and more!

 I want you to use the tool and share your thoughts about it.
 I want you to use the tool and share your thoughts about it.

 Thanks, but I'm not really interested in trying the tool, as I can't find too much information about it other than it's available at a number of download sites.

 Also, I don't find anything that indicates that it parses the DestList stream.
 Which is the difference between the information contained in *.automaticDestinations-ms and in lnk files %UserProfile%\AppData\Roaming\Microsoft\Windows\Recent?
 Post a Comment
 Pages
 Home
 Timelines
 Books
 Malware
 FOSS Tools
 Subscribe To WindowsIR
 WindowsIR Blog List
 Open Source DFIR Plaso 20260119 released 4 days ago
 Brett Shavers AI Won’t Replace DFIR Investigators. But It Will Replace Those Who Don’t
Investigate. 2 weeks ago
 The Philosophy of DFIR The Case Against Limited-Scope Warrants for Digital Evidence 1 month ago
 dfirtnt.wordpress.com Introducing Huntable CTI Studio 2 months ago
 c-APT-ure Using NetBIOS names for pivoting and threat clustering 6 months ago
 CyberDefNerd Xworm – Static Analysis (part 3) 8 months ago
 inversecos An inside look at NSA (Equation Group) TTPs from China’s lense 1 year ago
 ForensicITGuy
 Find Evil
 Blog Archive
 ► 2026 (8) ► March (2)
 ► February (1)
 ► January (5)

 ► 2025 (27) ► December (3)
 ► November (8)
 ► October (2)
 ► September (1)
 ► July (1)
 ► June (4)
 ► May (1)
 ► March (3)
 ► February (2)
 ► January (2)

 ► 2024 (22) ► December (1)
 ► November (1)
 ► October (7)
 ► July (1)
 ► June (1)
 ► March (4)
 ► February (2)
 ► January (5)

 ► 2023 (50) ► December (3)
 ► November (2)
 ► October (1)
 ► September (2)
 ► August (7)
 ► July (6)
 ► June (6)
 ► May (4)
 ► April (7)
 ► March (4)
 ► February (6)
 ► January (2)

 ► 2022 (51) ► December (3)
 ► November (4)
 ► October (6)
 ► September (5)
 ► August (5)
 ► July (9)
 ► May (5)
 ► April (5)
 ► March (4)
 ► February (2)
 ► January (3)

 ► 2021 (26) ► December (3)
 ► November (3)
 ► October (3)
 ► September (5)
 ► August (2)
 ► June (4)
 ► April (4)
 ► March (1)
 ► January (1)

 ► 2020 (26) ► November (2)
 ► October (3)
 ► September (1)
 ► August (3)
 ► July (1)
 ► June (2)
 ► May (2)
 ► April (3)
 ► March (2)
 ► February (4)
 ► January (3)

 ► 2019 (43) ► December (5)
 ► November (2)
 ► October (2)
 ► September (3)
 ► August (4)
 ► July (1)
 ► June (1)
 ► May (9)
 ► April (4)
 ► March (2)
 ► February (5)
 ► January (5)

 ► 2018 (49) ► December (4)
 ► November (4)
 ► October (4)
 ► September (7)
 ► August (6)
 ► July (1)
 ► June (4)
 ► May (2)
 ► April (2)
 ► March (7)
 ► February (5)
 ► January (3)

 ► 2017 (25) ► December (2)
 ► October (3)
 ► September (4)
 ► August (3)
 ► July (1)
 ► June (1)
 ► May (1)
 ► April (3)
 ► March (2)
 ► February (2)
 ► January (3)

 ► 2016 (43) ► December (1)
 ► November (1)
 ► October (3)
 ► September (5)
 ► August (3)
 ► July (2)
 ► June (5)
 ► May (5)
 ► April (4)
 ► March (3)
 ► February (5)
 ► January (6)

 ► 2015 (34) ► December (6)
 ► November (1)
 ► October (3)
 ► September (3)
 ► August (2)
 ► July (2)
 ► June (4)
 ► May (3)
 ► April (4)
 ► March (3)
 ► February (1)
 ► January (2)

 ► 2014 (33) ► December (3)
 ► October (5)
 ► September (2)
 ► August (1)
 ► July (4)
 ► June (1)
 ► May (5)
 ► April (5)
 ► March (4)
 ► February (1)
 ► January (2)

 ► 2013 (64) ► December (4)
 ► November (3)
 ► October (2)
 ► September (5)
 ► July (14)
 ► June (5)
 ► May (4)
 ► April (9)
 ► March (5)
 ► February (5)
 ► January (8)

 ► 2012 (73) ► December (3)
 ► November (4)
 ► October (5)
 ► September (4)
 ► August (3)
 ► July (4)
 ► June (8)
 ► May (11)
 ► April (8)
 ► March (7)
 ► February (7)
 ► January (9)

 ▼ 2011 (109) ▼ December (9) Jump List Parser Code Posted
 Jump List Analysis
 Even More Stuff
 New Stuff
 More Stuff
 Stuff
 Meetup
 Stuff
 New Stuff

 ► November (9)
 ► October (10)
 ► September (15)
 ► August (11)
 ► July (8)
 ► June (10)
 ► May (4)
 ► April (11)
 ► March (9)
 ► February (6)
 ► January (7)

 ► 2010 (90) ► December (12)
 ► November (5)
 ► October (3)
 ► September (2)
 ► August (3)
 ► July (10)
 ► June (9)
 ► May (5)
 ► April (8)
 ► March (10)
 ► February (15)
 ► January (8)

 ► 2009 (166) ► December (15)
 ► November (14)
 ► October (10)
 ► September (9)
 ► August (13)
 ► July (12)
 ► June (13)
 ► May (12)
 ► April (19)
 ► March (22)
 ► February (15)
 ► January (12)

 ► 2008 (108) ► December (9)
 ► November (6)
 ► October (12)
 ► September (9)
 ► August (17)
 ► July (11)
 ► June (9)
 ► May (4)
 ► April (11)
 ► March (4)
 ► February (8)
 ► January (8)

 ► 2007 (83) ► December (6)
 ► November (7)
 ► October (1)
 ► September (3)
 ► August (4)
 ► July (8)
 ► June (10)
 ► May (12)
 ► April (7)
 ► March (11)
 ► February (3)
 ► January (11)

 ► 2006 (118) ► December (1)
 ► November (16)
 ► October (18)
 ► September (15)
 ► August (17)
 ► July (7)
 ► June (8)
 ► May (4)
 ► April (12)
 ► March (3)
 ► February (9)
 ► January (8)

 ► 2005 (163) ► December (5)
 ► November (1)
 ► October (10)
 ► September (21)
 ► August (22)
 ► July (12)
 ► June (15)
 ► May (4)
 ► April (14)
 ► March (21)
 ► February (20)
 ► January (18)

 ► 2004 (16) ► December (16)
