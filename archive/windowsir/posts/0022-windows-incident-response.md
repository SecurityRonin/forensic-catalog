# Windows Incident Response

- URL: https://windowsir.blogspot.com/2007/05/forensic-laws.html
- Published: unknown
- Updated: unknown
- Labels: none

Windows Incident Response
 The Windows Incident Response Blog is dedicated to the myriad information surrounding and inherent to the topics of IR and digital analysis of Windows systems. This blog provides information in support of my books; "Windows Forensic Analysis" (1st thru 4th editions), "Windows Registry Forensics",
as well as the book I co-authored with Cory Altheide, "Digital Forensics with Open Source Tools".
 Saturday, May 12, 2007
 Forensic Laws

 24 comments:
 If you are starting to believe that every action leaves behind evidence you've been working on Windows for far too long :)

 But even on Windows this doesn't hold true. You're trying to postulate that the current state of a given machine contains information about every previous state of said machine, which simply is not true.

 Completely unrelated, you should write a series of articles on the creative ways of -avoiding- forensic analysis. Do you have any personal stories about people who have tried to make their computer "self-destruct" if it ever falls in the wrong hands?
 You're trying to postulate that the current state of a given machine contains information about every previous state of said machine, which simply is not true.

 You're right, but this isn't what the theorem says. It's not saying that evidence can't be destroyed, but it did exist at some time. The theorem is quite simple, which is its beauty.
 You're trying to postulate that...

 Actually, I'm not. See Jesse's comment. Besides, as I said my blog post, Jesse gets credit for the First Law...I don't accept any of the credit or responsibility for that one! ;-) I simply believe it to be true.

 Consider this...some action takes place on a system, and the evidence of that action is in memory. The system is shut down, power is removed from memory...but that does not mean that at one time that evidence did not exist, does it? It simply means that at a later time, the evidence ceased to exist. Hence the First Corollary...destruction is the extreme form of modification.

 ...you should write...

 I should?? Why should I have to be the one to do this? How about you?!?

 Do you have any personal stories about people who have tried to make their computer "self-destruct" if it ever falls in the wrong hands?

 I have a lot of stories about things I've seen where system owners (re: sysadmins) have done things that have destroyed evidence, but no, I really haven't personally seen anything like what you're asking.

 Thanks for the comment! Keep 'em coming!
 Hmmm...this is very interesting. Is there more to the law than the one statement? Jesse, can you define your use of the word 'evidence' and 'action' in this law? I'll reserve comments on this until there is a response.

 Harlan,
 How can we say that evidence is on the thumb drive with no thumb drive? That's an assumption or even a hypothesis that we can make, but I don't think it's fair to make an absolute claim that the evidence we are looking for is there. Once we have the thumb drive in our posession, then and only then can we begin to make a claim that the evidence we are seeking is present. That is, unless you are counting your Corollary.

 Given that you appear to be using both 'artifact' and 'evidence' to describe the same thing, could you also define your use of it?
 How can we say that evidence is on the thumb drive with no thumb drive?

 Because it's an example. I'm just giving an example of how things work sometimes, that's all.

 ...I don't think it's fair to make an absolute claim that the evidence we are looking for is there.

 Why not? If someone plugs a thumb drive into a Windows XP laptop and copies a file over to that thumb drive, when then is it not fair to say that there are artifacts on that thumb drive?

 The purpose of the example was not to make absolute claims, but rather to provide an example to the reader as a way of viewing the context of the entire blog post.

 ...could you also define your use of it?

 I'm using them interchangeably. I generally prefer to use 'artifact' because the word 'evidence' bogs too many people's thinking down, and closes them off. I like 'artifact' in the archeological sense.

 I think that I'd like to see what folks have to say about the content of the post and the idea, rather than getting all bogged down in semantics...
 Ok, so as not to get wrapped up in semantics(even though 'evidence' has many meanings depending on who you talk to)...

 The law in its simplicity is correct. There is evidence of every action, but only if we take the corollary in to account - which I am really liking right now because you include the absence of an artifact being an artifact.
 Before it becomes a law, I'd like to submit the following:

 Person A has a standard Windows PC with the ability to boot from the CD drive. Person A's computer is hard wired to a cable modem (or DSL line). No BIOS password.

 Person A has a party where Person B brings a bootable Linux CD, BackTrack for example. During the course of the party, Person B sneaks on to A's computer and notices it's off. He disconnects the network cable. He fires up the boot CD, mounts a thumb drive, and copies all personal docs, tax statements, love notes, etc. etc. to the thumb drive. NTFS drive is never mounted other than read-only. Of course, B is smart enough to eject the CD, take the thumbdrive, power off, and wipe the keyboard and mouse of fingerprints.

 From an intrusion standpoint there's nothing forensically on A's computer, right? I've done drive hashes before and after and they stay the same. You're leaving the computer in the exact state as before the compromise. Other than having some really good interview techniques, it's forensics-zero.
 From an intrusion standpoint there's nothing forensically on A's computer, right?

 Perhaps.

 I say this because of some of the things I've seen with getting the contents of physical memory from a Windows system...as we've discussed there seems to be nothing that zeros out or resets the contents of RAM. Now, I understand that Linux manages memory differently from Windows, but consider that if Person B shuts of the system and removes that bootable CD, there may be artifacts left in the contents of physical memory

 Also, let's look back at the First Law:

 There is evidence of every action

 Notice that this Law does not state explicitly where that evidence (or, those artifacts) exist.

 To answer your question directly, no, I have not seen anything, from the perspective of computer forensics, that would definitively state that the actions you describe where performed. I would think, however, that witnesses or fingerprints on the system would be physical evidence of the act.
 Responding to Chris, who said From an intrusion standpoint there's nothing forensically on A's computer, right?

 Nope!

 He fires up the boot CD

 Powering up the computer would increment the power-cycle count of the S.M.A.R.T. hard drive.

 NTFS drive is never mounted other than read-only.

 Assuming the NTFS driver is written properly and doesn't actually modify anything. I've never verified that it doesn't change something.

 Of course, B is smart enough to eject the CD, take the thumbdrive, power off, and wipe the keyboard and mouse of fingerprints.

 What about the toolmarks from the USB drive being connected to the computer? If there is a particular protrusion on the thumb drive connector that would leave a corresponding groove on the computer (or vice versa), it might be possible to associate the two devices later on. (Don't forget Locard's Exchange Principle too!)
 How does this new theorem differ from Locard's exchange principle? Or is it just a restatement of that principle?

 - Rossetoecioccolato.
 Jesse, thanks for the info. I was thinking more along the lines of an incident response or an investigation. I'm not familiar with power-cycle information, but unless it's got time/date stamps, i can't see what it could be used for. I don't think anyone would know what their power-cycle count was on last use. The USB toolmarks would probably require a high-powered microscope and the USB device "person B" used. I like Harlan's post about locating artifacts in RAM that may be leftover from using the boot disk. Sounds like some good lab-work activity for later.
 I'm not familiar with power-cycle information, but unless it's got time/date stamps, i can't see what it could be used for. I don't think anyone would know what their power-cycle count was on last use.

 Every S.M.A.R.T. hard drive keeps a count of how many times it has been powered on. When the computer boots, if the count is not one more than it was the last time the system was booted, then the hard drive has been powered up (most likely meaning the system has been booted). If you're not paying attention to that count, well, you're not being paranoid enough. [grin]

 The USB toolmarks would probably require a high-powered microscope and the USB device "person B" used.

 Nobody ever said the evidence created would be easily accessible!
 How does this new theorem differ from Locard's exchange principle?

 Now that I think about it, I'm not sure there is much of a difference. I want to think about this some more.
 I don't agree that [o]nce you understand what actions or conditions create or modify an artifact, then the absence of that artifact is itself an artifact. This seems to be very close to the old and discredited line about "absence of evidence is evidence of absence".

 The absence of an artifact by itself tells us very little other than we haven't found it. It could be absent. We may have overlooked it. We may not understand the actions and/or conditions as well as we think. The new guy may have accidentally contaminated our data and we're none the wiser. Etcetera, etcetera.

 I think forensics needs to be about "yes, Your Honor, we found the gun the murderer used, and as you can see for yourself, it's still smoking"--not "Your Honor, we didn't find the gun, but the fact we didn't find it leads us to conclude, based on our substantial expert experience, that it still exists and is currently smoking."

 Of course, I may have completely misunderstood the thrust of your post. If so, I apologize and look forward to the correction. :)
 Robert,

 The absence of an artifact by itself tells us very little other than we haven't found it. It could be absent.

 Not necessarily. By example, we know that a user's interaction with a Windows system, through the shell, should leave certain artifacts...because we know that certain actions create and/or modify certain artifacts on the system. Normal user behaviour will create or modify those artifacts.

 Now, let's say that you know that the user logged on via the shell, but you do not find some of those artifacts, or perhaps you find some and not others. This would then indicate what to you?

 I fully agree with your possibilities, but that's where other aspects of forensics, those not specifically applicable solely to computer forensics, apply...training, education, documentation, chain of custody, etc.

 Thanks.
 Miss this blog for a couple of weeks while you wrap up a case and you get six months behind....

 This is an argument I've been making for a long time. Every action on a computer leaves artifacts SOMEWHERE, be they digital or physical. I never worry much about the physical because that isn't anything I work on. I look for the presence and absence of artifacts to guide my case work.

 An example that ties into the Linux boot cd is if a person is murdered. The murderer gained access to the home by use a key hidden in the planter by the front door. Now, the key exists, wheerver it is. If the murderer leaves it behind we have live evidence. If he takes it with him, the evidence still exists, we just don;t know where it is, and we may not even know it exists. Absent a witness of interviewee who knew the victim had a hidden key we wouldn't know to look for it, wouldn't know it was missing, wouldn't know we had missing evidence, and wouldn't care. It just wouldn't be a part of our case. I have always wondered in crime scene forensics just what the percentage of available evidence was actually collected. I'll bet it is small. But it's getting the important evidence thats going to be the most useful in trial that matters. Same for us. We can chase "possible" scenarios till we catch our own tail, and ignore some weighty artifacts that are right under our noses.
 Of course, B is smart enough to eject the CD, take the thumbdrive, power off, and wipe the keyboard and mouse of fingerprints.

 A lack of fingerprints would be evidence of tampering as well...
 Fascinating discussion!

 the lack of Artifacts should be noted and reported ..but Artifacts with out their invoked Application(s) are more interesting...or vice versa...Applications that have been invoked , but no artifacts recorded!?

 JohnT
 One of the main themes Harlan mentions is the lack of evidence/artifacts is actually a piece of evidence/artifact because it's not there and should be. Any good investigator can piece together an incident scene, whether it is a physical crime scene or computer incident, based on what they see. A great investigator will know what is supposed to be there, but is not there, and make inferences to what may have happened.

 There are countless means to hide evidence/artifacts. From wiping fingerprints or using Linux boot CD's to access a hard drive. A very simplistic example of the 'something is missing' could be a murder scene where a body was found in the living room. No signs of forced entry, no signs of a struggle, no signs of the murder weapon (a gun for example), no witnesses, nothing but the body. The 'what is not there but should be there' evidence/artifacts are what is important, and is evidence because it is missing. Where is the gun? How did the person get in? Why was there no struggle? Why no fingerprints? Inferences can be made from any of these to assist in the investigation. I have always looked for evidence/artifacts that are not there, but I know they should be.

 Electronically, the concept is the same. If a file is known to exist on only one machine, but is found on another, and there is no evidence of how that same hashed file got there, then something is missing to show how that happened. That's the good stuff.
 Brett,

 Thanks for the comment! Sometimes, tying the physical world to the digital really helps solidfy some concepts.

 Thanks!
 In accordance with the Kornblum theorem, I have a question that I was hoping to get some help with.

 This is the case of the disappearing email. In examining our bad guy's PST files I find that there is no email from late march till 2 days before he leaves the company in Nov (same year). This is a windows XP pro running Outlook 2003 as the email client. I have all PST's from the computer and network that the complianant is aware of (key words are "the the complainant is aware of). My theory is that the suspect, knowing the jig was up, created another PST file and dirverted the email from the key time frame or was using another (personally owned)computer to access the email system.

 In examining the registry (Windows Messaging Subsystem) I find pointers for the PST(s) I already know about. However, I also find two default registry keys with a last access date of the same date as the email stopped in late March.

 I have been doing some simple test with creating PST files and transfering email but I can't figure out what causes the default entries to be created or where else to look for evidence to established what took place during the missing email time frame.

 I have already looked at and for deleted email, deleted PST's, webmail, network access, and the exchange server.

 Any help or suggestions would be greatly appreciated.
 d8tacop...

 Not sure what I can do to help...for example:

 ...However, I also find two default registry keys with a last access date of the same date...

 Which two keys? What are the key names/paths, and how do they pertain to the question at hand?

 Is it possible that the emails in question were simply deleted?
 Can anyone be sued based only on the evidences that is using just the anti-forensic tools?
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

 ▼ 2007 (83) ► December (6)
 ► November (7)
 ► October (1)
 ► September (3)
 ► August (4)
 ► July (8)
 ► June (10)
 ▼ May (12) XP Firewall
 XP Anti-Forensics
 Prefetch Analysis
 New versions of tools released
 Litchfield on Oracle Live Response
 Forensic Visualization
 Forensic Laws
 PPT Metadata
 Event Logs in Unallocated Space
 SOLD OUT!
 Interviews
 Something Else To Look For...

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
