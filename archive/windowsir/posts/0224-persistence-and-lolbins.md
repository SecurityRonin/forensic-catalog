# Persistence and LOLBins

- URL: https://windowsir.blogspot.com/2022/12/persistence-and-lolbins.html
- Published: 2022-12-31T18:43:00.001-05:00
- Updated: 2022-12-31T18:43:59.955-05:00
- Labels: persistence, RegRipper

Grzegorz/@0gtweet tweeted something recently that I thought was fascinating, suggesting that a Registry modification might be considered an LOLBin. What he shared was pretty interesting, so I tried it out.
 First, the Registry modification:
 reg add "HKLM\System\CurrentControlSet\Control\Terminal Server\Utilities\query" /v LOLBin /t REG_MULTI_SZ /d 0\01\0LOLBin\0calc.exe
 Then the command to launch calc.exe:
 query LOLBin
 Now, I've tried this on a Windows 10 system and it works great, even though Terminal Services isn't actually running on this system. Running just the "query" command on both Windows 10 and Windows 11 systems (neither with Terminal Services running) results in the same output on both:
 C:\Users\harlan>query
 Invalid parameter(s)
 QUERY { PROCESS | SESSION | TERMSERVER | USER }
 Running the "query" command with different parameters (i.e., "process", "user", etc.) proxies that command to the appropriate entry based on the value in the Registry, as illustrated in figure 1.

 As such, running "query user" runs quser.exe, and you see the same output as if you simply ran "quser".
 Note that the Utilities key has two other subkeys, in addition to "query"; "change" and "reset", as illustrated in fig. 2.

 So, I thought, what if I change the key path from "query", and make the same modification (via the 'reg add' command above) to the "change" subkey...would that have the same effect? Well, I tried it with an elevated command prompt, for both the "change" and "reset" subkeys, and got "Access is denied." both times. Okay, so we can only use this...at an Admin level, anyway...with the "query" subkey.
 So what?
 Part of what makes this particular persistence so insidious (IMHO) is how it can be launched. When we use the Run keys (or a Scheduled Task, or a Windows service) for persistence, an analyst may not have any trouble seeing the launch mechanism for the program/malware. In fact, even outside of Registry analysis, some SOC consoles will allow the analyst to see notable events, and even provide enough process lineage to allow the analyst to deduce that a notable event occurred.
 To Grzegorz's point, this is an interesting LOLBin, because query.exe exists on the system by default. IMHO, this is important because when I started learning computers over 40 yrs ago (yes, circa 1982), all we had was the command line; as such, understanding which commands were available on the system, as well as things like STDOUT, STDERR, file redirection, etc., were all just part of what we learned. Now, 40 yrs later, we have entire generations of analysts (SOC, DFIR, etc.) who "grew up" without ever touching a command line. So, when I saw Grzegorz's tweet, the first thing I did was go the command prompt and type "query", and saw the response listed above. Easy peasy...but how many analysts do that? What's going to happen if a SOC analyst sees telemetry for "query user"? Or, what happens if a DFIR analyst sees a Prefetch file for query.exe ? What assumptions will they make, and how will those assumptions drive the rest of their analysis and response?
 What is a way to use this? Well, let's say you gain access to a system...and this is just hypothetical...and run a script that enables RDP (if it's not already running), enables StickyKeys, writes a Trojan to an alternate data stream (thanks to Dr. Hadi for some awesome research!!) and then creates a value beneath the "query" key for the Trojan. That way, if your activity on other systems gets discovered, you have a way back into the infrastructure that doesn't require authentication. Connect to the system via the Remote Desktop Client, access StickyKeys so that you get a System-level command prompt, and you type "query LOLBin". Boom, you're back in!
 As you might expect, I did write a RegRipper plugin for this persistence mechanism, the output of which makes it pretty straightforward to see any changes that may have been made. For example, "normal" look like the following:
 utilities v.20221231
 (System) Get TS Utilities subkey values
 Category: persistence - T1546
 ControlSet001\Control\Terminal Server\Utilities\change
 LastWrite time: 2018-04-12 09:20:04Z
 logon           0 1 LOGON chglogon.exe
 port            0 1 PORT chgport.exe
 user            0 1 USER chgusr.exe
 winsta          1 WINSTA chglogon.exe
 ControlSet001\Control\Terminal Server\Utilities\query
 LastWrite time: 2018-04-12 09:20:04Z
 appserver       0 2 TERMSERVER qappsrv.exe
 process         0 1 PROCESS qprocess.exe
 session         0 1 SESSION qwinsta.exe
 user            0 1 USER quser.exe
 winsta          1 WINSTA qwinsta.exe
 ControlSet001\Control\Terminal Server\Utilities\reset
 LastWrite time: 2018-04-12 09:20:04Z
 session         0 1 SESSION rwinsta.exe
 winsta          1 WINSTA rwinsta.exe
 Analysis Tip: The "query" subkey beneath "\Terminal Server\Utilities" can be used for persistence. Look for unusual value names.
 Ref: https://twitter.com/0gtweet/status/1607690354068754433
