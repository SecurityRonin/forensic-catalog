# Windows Incident Response

- URL: https://windowsir.blogspot.com/2021/09/on-writing-dfir-books-pt-i.html
- Published: unknown
- Updated: unknown
- Labels: none

Windows Incident Response
 The Windows Incident Response Blog is dedicated to the myriad information surrounding and inherent to the topics of IR and digital analysis of Windows systems. This blog provides information in support of my books; "Windows Forensic Analysis" (1st thru 4th editions), "Windows Registry Forensics",
as well as the book I co-authored with Cory Altheide, "Digital Forensics with Open Source Tools".
 Monday, September 06, 2021
 On Writing DFIR Books, pt I
 During my time in the industry, I've authored 9 books under three imprints, and co-authored a tenth.
 There, I said it. The first step in addressing a problem is admitting you have one. ;-)
 Seriously, though, this is simply to say that I have some experience, nothing more. During the latter part of my book writing experience, I saw others who wanted to do the same thing, but ran into a variety of roadblocks, roadblocks I'd long since navigated. As a result, I tried to work with the publisher to create a non-paid liaison role that would help new authors overcome many of those issues, so that a greater portfolio of quality books became available to the industry. By the time I convinced one editor of the viability and benefit of such a program, they had decided to leave their profession, and I had to start all over again, quite literally from the very beginning.

 Authoring a book has an interesting effect, in that it tends to create a myth around the author, one that they're not aware of at first. It starts with someone saying, "...you wrote a book, so you must X ..". Let " X " be just about anything.
 "Of course you're good at spelling, you wrote a book." Myth.
 "You must be rolling in money, you wrote a book." Myth.
 All of these things are assumptions, myths built up only to serve as obstacles. The simple fact is that if you feel like you want to write a book, you can. There's nothing stopping you, except...well...you. To that end, I thought I'd write a series of posts that dispel the myths and provide background and a foundation for those considering the possibility of writing a book.

 There are a number of different routes to writing books. Richard Bejtlich has authored or co-authored a number of books, the most recent of which have been reprints of his Tao Security blog posts. Emma Bostian tweeted about her success with "side projects", the majority of which consisted of authoring and marketing her ebooks.
 The Why
 So, why write books at all? In an email that Gen Jim Mattis (ret) authored that later went viral, he stated:

 By reading, you learn through others’ experiences, generally a better way to do business, especially in our line of work where the consequences of incompetence are so final for young men.
 Yes, Gen Mattis was referring to warfighting, but the principle equally well for DFIR work. In his book, " Call Sign Chaos ", Mattis further stated:

 ...your personal experiences alone aren't broad enough to sustain you.
 This is equally true in DFIR work; after all, what is "analysis" but the analyst applying the sum total of their knowledge and experience to the amassed data? As such, the reason to write books is that no one of us knows everything, and we all have vastly different experiences. Even working with another analyst on the same incident response engagement, I've found that we've had different experiences due in large part to our different perspectives.
 The simple fact is that these different perspectives and experiences can be profoundly valuable, but only if they're shared. A while back, I engaged in an analysis exercise where I downloaded an image and memory sample provided online, and conducted analysis based on a set of goals I'd defined. During this exercise, I extracted significantly different information from the memory sample using two different tools; I used Volatility to extract information about netstat-style network connections, and I also used bulk_extractor to retrieve a PCAP file, built from the remnants of actual packets extracted from memory. I shared what I'd done and found with one other analyst, and to be honest, I don't know if they ever had the chance to try it, or remembered to do so the next time the opportunity arose. Since then, I have encountered more than a few analysts to whom this approach never occurred, and while they haven't always seen significant value from the effort, it remains a part of their toolkit. I also included the approach in " Investigating Windows Systems ", where it is available, and I assume more than one analyst has read it and taken note.
 Speaking for myself, I began writing books because I couldn't find what I wanted on the shelves of the bookstore. It's as simple as that. I'd see a title with the words "Windows" and "forensics" in the title, and I'd open it, only to find that the dive did not go deep enough for me. At the time, many of the books related to Windows forensics were written by those who'd "grown up" using Linux, and this was clearly borne out in the approach taken, as well as the coverage, in the books.
 The First Step
 The first step to successfully writing a book is to read. That's right...read. By reading, we get to experience a greater range of authorship, see and decide what we enjoy reading (and what we pass on), and then perhaps use that in our own writing.
 My first book was " Windows Forensics and Incident Recovery ", published in 2004. The format and structure of chapter 6 of that book is based on a book I read while I was on active duty in the military titled " The Defense of Duffer's Drift ". I liked the way that the author presented the material so much that I thought it would be a useful model for sharing my own story. As it turned out, that was the one chapter that my soon-to-be wife actually read completely, as it is the only chapter that isn't completely "technical".
 With that, thoughts, comments and questions are, as always, welcome. Keep an eye open for more to come!

 No comments:
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

 ▼ 2021 (26) ► December (3)
 ► November (3)
 ► October (3)
 ▼ September (5) Imposter Syndrome
 Distros and RegRipper
 On Writing DFIR Books, pt II
 Tips for DFIR Analysts, pt II
 On Writing DFIR Books, pt I

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
