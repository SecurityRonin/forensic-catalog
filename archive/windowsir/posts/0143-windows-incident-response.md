# Windows Incident Response

- URL: https://windowsir.blogspot.com/2017/08/beyond-getting-started.html
- Published: unknown
- Updated: unknown
- Labels: none

Windows Incident Response
 The Windows Incident Response Blog is dedicated to the myriad information surrounding and inherent to the topics of IR and digital analysis of Windows systems. This blog provides information in support of my books; "Windows Forensic Analysis" (1st thru 4th editions), "Windows Registry Forensics",
as well as the book I co-authored with Cory Altheide, "Digital Forensics with Open Source Tools".
 Tuesday, August 22, 2017
 Beyond Getting Started

 14 comments:

 For me, the best part which I totally agree with you is about "Questioning". Maybe because I learned myself by questioning most of the things if not everything, and maybe because I truly believe it's the difference between someone whom knows what he/she is doing, and between someone whom just knows how to do it. I always tell my students not to blindly take what they learn even though they believe the person in front of them is an expert or whatever. Maybe at that specific time I lost concentration and said something let's say not 100% wrong, but something that was not totally true (partially wrong), maybe there is a better way too. Why stick to the way you learned from me only, go find your own way, use it and discover yourself whether it works or not.

 Tools, tools, tools, lots believe it's the tool that makes you good or even great, and I like you don't believe in that. What good is a Ferrari, if I'm no good driver? What good is the fanciest toolbox, if I don't know how to use it? The same for computers, and especially DFIR. What good is using a tool if I don't know why it does it retrieves shows me the data that way? Why should I believe it; just because it was developed by company XYZ, and XYZ has a huge reputation in the industry, doesn't mean I must blindly use it and not question it. Tools are tools, we use them to speed up the process and make things much quicker and faster, but it is the Human Intelligence part that is important too. Most of the time during my teaching especially Infosec related courses at the start I get negative feedback from attendees, they want to see using this tool and that, but I try before that to stay focused on the "WHY" and idea behind the technology or part of process I'm explaining is much more important. I totally believe if we manage to understand the path and idea, then the tool could be easily and accurately selected. "Hope I managed to express my ideas here... :)"

 The part were I don't agree with you, is your questioning of people that do "Likes and Re-Tweets" without engaging. Not engaging with the article/post does not mean they didn't read it, but maybe have no addition, didn't have time or maybe didn't like to engage :)
 Or the another issue, is maybe they are shy of sharing their thoughts and afraid that what they share is wrong! I've seen lots under this category, and I always say that we all do mistakes and maybe have understood an idea wrong, so it is okay. Anyway, not engaging does not mean that I didn't read your posts, especially yours!

 Thanks again for your time writing this post.
 Ali,

 Thanks for the comment.

 I see tools posted all the time, and others sharing links to the tools, but almost nothing about how folks have used the tools.

 > Not engaging with the article/post does not mean they didn't read it...

 Well, when the post tells folks to not just Like and Retweet, but to share *why* they're doing so, but all they do is Like or RT...that kind of tells me that they didn't read it. ;-)

 My thought is that if something is worth sharing with others, it's worth telling folks why. It's not so much about having an addition, something to add, it's more about what was said or discussed that led you to think, "yeah, I've gotta share this with others", or "this is worth sharing with others".

 > ...maybe they are shy of sharing their thoughts ...

 Hhhmmm. I guess what I don't understand is this...why are folks who do DFIR work not afraid to write reports that they share with their manager, and more importantly, with their clients? How is it that someone can be fine with sharing their expert opinion and interpretation of their findings, but not be comfortable sharing their thoughts and opinions?

 My guess in not engaging in the community is the fear of being wrong, fear of having words taken out of context, fear of being criticized, and the fear of having public statements used adversely against you in cases. Personally, I believe that sharing publicly does the opposite of these fears. Being peer-reviewed in public increases credibility, corrects errors, and moves forward the ideas that are discussed.

 Credibility and knowledge is in direct relation to the level of engagement in the sharing of ideas.

 I also believe everyone has something to share at some level in the community. It doesn't have to be a groundbreaking discovery that requires a formal peer review in order to be worthy of sharing with the community, but an idea or a spark that ignites a discussion that can eventually solve a previously unsolved problem.
 Brett,

 > ...sharing publicly does the opposite...

 I have to say, I agree with you.

 Several years ago, I was in the audience for a presentation, and the presenter stated that he'd informed his client of their window of compromise based on the (incorrect) interpretation of a single data point. While nothing could be done about what the client was told, at least discussing this privately with the presenter corrected their interpretation of the data.

 For those who don't share out of fear that their words will be used against them, sharing and solidifying your thoughts and understanding lead to that credibility you mentioned.

 > I also believe everyone has something to share ...

 As do I. I believe that there is as much value in letting others know that activity continues as there is in identifying new activity.

 What I don't get about the "fears" you mentioned is that DFIR analysts have to write reports and share their findings with someone...a client, the prosecutor, HR, etc. How is it easy to do so without fear, but those same people are afraid to share an opinion, or something that they've seen?

 For those who choose not to engage in the community, but willingly provide reports and testimony in the public purview, my guess is that if they were not required to provide reports and testimony, they would not. By required, I mean "paid".
 Harlan,

 Great post. Everyone needs to stretch professionally to grow. There are a variety of reasons why someone may not be as public facing with contributions. However, I agree that encouraging public contributions is great for the community. It leads to peer review, as Brett mentioned, and new discovery. That said, I see nothing wrong with retweeting things you find value in as it increases the signal on important findings and news. For example, the work Dan P and David C. Each shared on shellbags this week. I retweeted it because it was important that everyone know the additional scenarios that create a shellbag entry. I didn't discover it, but saw it and wanted to bring it to the attention of others. I am okay with people who retweet things they find of value because it pumps up the signal. Of couse I retweeted this post as well.
 First off, thanks for the mention.

 I've found that many internal IR groups don't actually document much or anything. The reports I have reviewed for customers have gone from a small narrative paragraph to 200+ pages of copy/paste emails. I think there is also an idea that your report is only going to be read by a small set of people, if it even gets read at all. Posting a blog can be intimidating because you are hanging it all out for your very critical peers to see. Take an example of the extreme sides that the members of our industry took in response to MalwareTech being arrested and not having any evidence to logically go either direction.

 Also, for me, the twitter 'like' still functions as the old 'favorite' button which is more like tagging an email for followup. I think we could all improve more on the comments and explaining the value you see as the reason for further sharing.

 Great post!
 Thanks, all, for your comments...

 @Jessica,

 From my experience, there is not a great deal of review (peer or otherwise) with respect to reports, etc. As such, I completely agree with Brett, in that having *some* form of review leads to growth. And I do think that the medium leads misinterpretations

 > I retweeted it because it was important that everyone know...

 I agree that this is important for data interpretation (God knows we still see AppCompatCache entries misinterpreted or misrepresented pretty regularly), but to my point, when you RT'd, did you share your thoughts/reasons for *why* you thought this was important?

 I've worked with people who've shared things internally within the organization, sending a link with nothing more than an "FYSA". When this first arrives, it's noise, and there's nothing about "FYSA" followed by a URL that distinguishes it from noise. As such, I don't agree that RT'ing something inherently "pumps up the signal" as much as it adds to the noise. If you're going to RT or share something because you found value in it, pump up the signal by also sharing what you found to be significant.

 @James,

 > I've found that many internal IR groups don't actually document much or anything.

 Amen, brother!

 > ... if it even gets read at all.

 Oh, agreed. I tend to think that, based on what I've seen produced over the years, that some folks put all their eggs in that basket, hoping that the client will read the executive summary, make their own assumptions off of that, and run with it.

 > ...example of the extreme sides...

 I get that, and you have to be ready for it. You have to assume that in any large group, particularly with the presumption of some modicum of anonymity, that some people are just going to be d*cks. We should also agree that due to the medium, some folks with the very best of intentions are going to viewed as d*cks. One such example I've experienced in responding to questions is when I ask, "which version of Windows are you working with?" No matter how many times this factor is described as being important, that question often elicits the response of, "oh, look at this jerk! Who does he think he is???"

 I'm not suggesting that we comment on *everything*. What I'm saying is that we have a lot of smart folks in this industry, and we're really missing out on some great stuff by doing nothing more than clicking "Like", "RT", or "Favorite".
 Statements I've heard over the years as reasons for not contributing, even just with internal teams:

 "Everyone's already seen this..."

 Not true, and so what? The fact is, I'll be the first to admit that I haven't seen everything. I'll also be the first to say that seeing a bit of malware or a technique that others have seen is as valuable as seeing something for the first time. Even if everything is the same with the malware...IIV, delivery mechanism, persistence mechanism, etc...the fact that it's still being used is important. Look at other aspects...what was the vertical attacked? If it was a phishing attack, *who* was targeted? Years ago, I was involved in a targeted breach where a retired 3-star general was targeted, and I thought that was pure genius, particularly from a cultural perspective. I mean, who else can you pretty much guarantee will click on something, regardless of training.

 Stepping forward to more modern times, take a look at Allison Wikoff's research into the "Mia Ash" persona, particularly the individuals that were targeted.

 "I can't contribute the way you do."

 No one's asking you to. This is one of those responses where someone goes to a ridiculous extreme in order to justify their actions (or lack thereof), and if anything, indicates that you're in for a long, hard battle to get them to contribute.

 When I say, "...if you 'like' something, then there's a reason you 'like' it, so share that along with the 'like' or retweet...", and the response is "...but I can't contribute the way you do...", it's clear that walls are being thrown up.

 *NOT* sharing a thought or considered opinion is a loss for us all. But remember, these interactions go beyond just the act of contributing...it's important to, as @James said, 'be present' while doing so.

 Where's the like/retweet button ;)
 Great article love the comments on learning to program if nothing more then to learn to think about the bigger picture and how to process things.
 @Harlan

 Agreed. I typically include a "SoWhat" factor in a quote retweet. I use the Mute feature to reduce the noise on Twitter. As for reports, in any organization where I worked where peer review did not exist, I instituted it sometimes from the bottom up. Peer review is essential. What I would personally like to see is more of the bloggers in our world submitting more citable scholarly articles. DJI would be even better with more submissions. Time is of course a factor. But that is a peer reviewed journal in our field. But I recognize several hurdles including, but not limited to, time and those whose employers probibit or have high barriers for publication.
 @Jessica,

 Agreed, peer review is essential. It's how we get better at what we do.

 I've reviewed articles for scholar journals, and to be honest, I've been disappointed. I think that the best way to describe it goes back to some of the OSDFCons I've attended that have had academic and practitioner tracks...the practitioners have had a very difficult time with the academic side. Yes, it's great that there's a paper with all this math that someone put a great deal of work into, but how do we *use* it?

 I've flat out turned down some of the recent journal articles I've reviewed because they were just poorly written.

 Yes, it is difficult to write for such journals for all of the reasons you stated. Sometimes it's better to plant the seed that puts others on the road to discovery than to try to plant the tree.
 Oh, and I should also add that other articles I've turned down have started off with a statement describing a problem that apparently did not exist.

 The last academic paper that I can remember being *really* valuable was Jolanta Thomassen's paper on deleted keys and values in the Windows Registry. Her research was well considered and valuable, the paper was well written and easy to understand, and she released a tool along with it. I think that the reason I found so much value in her work is that she'd engaged in the industry to determine what was relevant.
 I'm not a fan of academic papers. As Harlan mentioned, solving non-existent problems using extensive mathematical analysis and algorithms, written at a PhD level, is only fit as a homework project for a theoretical problem that no one has ever seen or expected to see.

 I do enjoy any writing (blog post, article, a simple PDF, or even a PPT) that gets right to the point of (1) the problem and (2) the solution.

 However, I may be alone in what I prefer to read to get the job done...
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

 ▼ 2017 (25) ► December (2)
 ► October (3)
 ► September (4)
 ▼ August (3) Beyond Getting Started
 Updates, New Stuff
 Document Metadata

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
