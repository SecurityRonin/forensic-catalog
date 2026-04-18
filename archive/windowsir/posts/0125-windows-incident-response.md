# Windows Incident Response

- URL: https://windowsir.blogspot.com/2014/08/what-does-that-look-like.html
- Published: unknown
- Updated: unknown
- Labels: none

Windows Incident Response
 The Windows Incident Response Blog is dedicated to the myriad information surrounding and inherent to the topics of IR and digital analysis of Windows systems. This blog provides information in support of my books; "Windows Forensic Analysis" (1st thru 4th editions), "Windows Registry Forensics",
as well as the book I co-authored with Cory Altheide, "Digital Forensics with Open Source Tools".
 Thursday, August 21, 2014
 What does that "look like"?

 8 comments:
 So first I'm not sure if you are aware of this but GRR now has basic "artifact" support and for plaso it is on the roadmap. You might or might not seen the talk at blackhat about the GRR artifacts.

 I'm going to address only the surface of a couple of the challenges.

 1. A lot of the information is locked up in books, tools or blog posts. The information is often poorly documentation and validated. Recent examples in the book your are referring to made this very clear again. If people writing the books, programs and post can't get it right how do you suppose others will?

 So the first challenge is making sure to create an authoritative set of information that is validated that is agreed upon.

 2. So our take on it was. Let's put the basic knowledge into the tools and have contextual information on a public accessible source, e.g. forensicswiki. Now the challenge becomes what an "artifact" and how to define it pro-grammatically. You also mention "indicator", another conceptual term where various attempts exists to define them [http://forensicswiki.org/wiki/Cyber_Threat_Intelligence].

 My take on it keep-it-simple first and let's try to share something between a limited set of tools. Once we have a foundation that proofs to work for 80% cases we already have made significant progress and start scaling up.

 3. There exists a lot of indicators in reports, but the problem is that these were never tested against a larger sample set. So the false positive/negative ratio is unknown and without the original context pretty useless. This is a "friction field" between forensic analysis and malware analysis. If you want to tackle APT cases successfully you'll have to master them both.

 4. Regarding clustering; I like to think about it beyond 1 system. A typical Windows 7 system will have roughly 4 to 5 VSS, so 5 to 6 copies of the same system in time. A case with lateral movement will have spread across multiple systems. Do our current tools allow us to analyze this at scale? IMO largely no. At least one refreshing take on this is: timesketch.org

 Last but not least, I see a lot of people talking about it, but very little people doing actual work on it. So if you're one of these people that would like to see this happen, drop me a mail.
 Joachim,

 A lot of the information is locked up in books, tools or blog posts. The information is often poorly documentation and validated.

 That's kind of what I'm getting at. For me, I keep my case notes for the individual cases I work, and have a separate document into which I put indicators. That way, I have one place to go to find them, and validate them as they apply to the case.

 Many of the indicators that I have and use have been successfully validated across multiple cases.

 Let's put the basic knowledge into the tools and have contextual information on a public accessible source, e.g. forensicswiki.

 Into which tools do we put this basic knowledge?

 Another concern that I've heard expressed is, what happens to the contextual information when someone is in a situation where they cannot access the Internet?

 ...various attempts exists to define them.

 I tend to believe the message gets lost of we spend too much time and effort in defining things.

 For the most part, I agree with what you've shared. For me, asking for input from the community at large hasn't worked...maybe this is another instance where someone just needs to start taking steps in a particular direction, making adjustments along the way, and wait for others to follow.

 Regarding #4, you and I have different experiences regarding Windows 7 systems, but I tend to think that's a good thing. With respect to scalability, what I've done for the past 5 or so years when looking at multiple systems in targeted threat cases is to simply merge my timelines...this is surprisingly revealing, and very often reveals TTPs.

 I see a lot of people talking about it, but very little people doing actual work on it.

 I agree, but only partially. I really don't see many people talking about it, but when I bring it up, I tend to find agreement...but everything ends there. I also tend to see a lot of excuses/reasons for why people don't share artifacts and indicators.

 I've also found over time that providing examples of how to share artifacts without exposing specific case information, and having discussions, does nothing to get analysts to start sharing.

 I've also found that when you do share artifacts, not many analysts incorporate what you've shared into what they do. I've been told by a number of analysts that they'll just save the email, or put the document into a folder, and when they feel that they need to revisit what was said (assuming that they remember it...), they'll search for it. If information isn't used and incorporated into analysis processes, then it doesn't get validated, nor does it get extended.

 For example, I've included a number of indicators in the tools I released with WFA 4/e . The file eventmap.txt allows a number of Windows Event Log records to be "tagged" so that they are easier to understand when included in a timeline. The malware detection checklist includes additional indicators...but at this point, I have no idea if anyone is even using them, to say nothing of validating them.

 My final concern is those who will take and use the indicators, without contributing indicators nor validation back to the project.
 > Into which tools do we put this basic knowledge?

 For context:
 https://www.blackhat.com/docs/us-14/materials/us-14-Castle-GRR-Find-All-The-Badness-Collect-All-The-Things-WP.pdf

 Though others have been talking about similar ideas. Sorry for not directly mentioning those, but can't find the links atm.

 > Another concern that I've heard expressed is, what happens to the contextual information
 > when someone is in a situation where they cannot access the Internet?

 In the current information age sharing without Internet connectivity is a hassle, but IMO this problem can be easily overcome.

 > I tend to believe the message gets lost of we spend too much time
 > and effort in defining things.

 Can't agree more ;) but for sharing pro-grammatically you'll have to have some standards. Preferable ones that don't make matter worse.

 > Regarding #4, you and I have different experiences regarding Windows 7 systems,
 > but I tend to think that's a good thing.

 Definitely a good thing the more different perspectives we have the better coverage we can attain.

 > what I've done for the past 5 or so years when looking at multiple systems in targeted
 > threat cases is to simply merge my timelines..

 That is exactly where timesketch comes in useful ;)
 but focusing on supporting multiple analysts at the same time as well, directly sharing interesting search filters among one and other. Take that idea one step further, what about sharing this information with e.g. trusted third parties? Though now things become more challenging, largely from non-technical aspects.

 > I agree, but only partially.

 That is fine ;)

 > The file eventmap.txt allows a number of Windows Event Log records to be "tagged"
 > so that they are easier to understand when included in a timeline.

 Not sure I'd not heard about it yet, have not read WFA 4/e so likely not. Do you have a direct link for me?

 Though it sounds similar to a mix of where plaso is heading with filtering and tagging and what I'm trying to add with winevt-kb [https://code.google.com/p/winevt-kb/] by creating a redistribute-able database of event messages. I know this was done before by grokevt, but alas their code and documentation did not fulfill my needs.

 > My final concern is those who will take and use the indicators,
 > without contributing indicators nor validation back to the project.

 Can't agree more. Though in my experience giving back to a project sometimes can be not possible, e.g. if it is no longer maintained, or people feel unsure or slightly intimidated by the idea. There are definitely, for a lack of a better term, "free loaders" out there.
 > For context:...

 I printed the PDF out...it's interesting that there are thoughts there like what Corey Harrell and I have discussed, such as artifact categories.

 > ... IMO this problem can be easily overcome.

 I completely agree.

 >> The file eventmap.txt allows a number of Windows Event Log records to be "tagged"
 >> so that they are easier to understand when included in a timeline.

 > Not sure I'd not heard about it yet, have not read WFA 4/e so likely not. Do you have a direct link for me?

 http://windowsir.blogspot.com/p/books.html

 Go to the line for WFA 4/e, click on the "Book Materials" link.

 >> great indicator that I've used comes from pg 553

 For those of us with the Digital version (no page number) can you give us the title of the section. I purchased the Kindle Edition of the book.
 > For those of us with the Digital version (no page number) can you give us the title of the section.
 > I purchased the Kindle Edition of the book.

 Not sure what so "great" about the indicator and which one Harlan is referring to specifically, that is something that Harlan needs to answer. However the page discusses strings in gs.exe [gsecdump]. I think the bottom line message is that, if you understand the behavior of tools the attackers use, you can use secondary behavior e.g. Registry keys being updated as indicators.

 Note that it is better to strings with -t option to see the actual location of the string in the executable and check before hand if the executable is packed. Also know that strings in an executable have a high false positive rate by itself. It is very easy to hide tools in legit looking executable.

 > Go to the line for WFA 4/e, click on the "Book Materials" link.

 Thanks for the link. I had a short look at eventmap.txt.

 I understand that tagging them will give you "event log entries of interest", though it would be nice to take this idea a bit further. The problem with solely using tagging is the noise/signal ratio, e.g. if you rely on an event log entry in the security log to be present as an indicator you must be very lucky to find it, if the environment you're analyzing is poorly administrated.

 So in plaso we are working towards something we dubbed it "anomaly detection", which is one of the possibilities the analysis plugins allow us to do. The clustering you mentioned earlier is a subset that could be possible with the analysis plugins. Alas atm I'm heavily revising this part of the codebase and other priorities allow me only slow progress. Also see:
 http://plaso.kiddaland.net/analysis-technique/analysis-plugins

 Not sure what so "great" about the indicator...

 It was great because the Volatility folks shared it, and others were able to validate it and find it reliable, making one of the points of my blog post.

 ...you can use secondary behavior e.g. Registry keys being updated as indicators.

 This is something I've been suggesting for a LONG time. Seeing ripples at the edge of a pond, I don't need to see that someone threw a stone in the water to know that something happened. I don't need to actually see a deer to know that one lives in the woods, if I see droppings, prints, etc. Chris Pogue referred this as "expert eyes".

 ... though it would be nice to take this idea a bit further.

 I agree. However, I have not received a single comment on this tool since the book was released. I had attempted to do something similar by building alerts into RegRipper 2.8...similarly, I have no idea if others find it useful, or if others have thoughts on how it might be improved.
 So in plaso we are working towards something we dubbed it "anomaly detection",...

 I think that "we" is the key part of the above sentence. You appear to be part of a team working on this, whereas I've been looking to the community for input.
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

 ▼ 2014 (33) ► December (3)
 ► October (5)
 ► September (2)
 ▼ August (1) What does that "look like"?

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

 ► 2011 (109) ► December (9)
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
