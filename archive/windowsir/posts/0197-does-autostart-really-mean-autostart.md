# Does "Autostart" Really Mean "Autostart"?

- URL: https://windowsir.blogspot.com/2022/07/does-autostart-really-mean-autostart.html
- Published: 2022-07-09T14:05:00.001-05:00
- Updated: 2022-07-09T14:05:44.436-05:00
- Labels: none

Most DFIR and SOC analysts are familiar with the Run keys as autostart locations within the Windows Registry:
 [HKLM|HKCU]\Software\Microsoft\Windows\CurrentVersion\Run
 Values beneath these keys are automatically run asynchronously upon system start and user login, respectively. This is something we've know for a while, and we've dutifully incorporated these autostart locations into our "indicators of program execution" artifact category.
 It turns out, that may not be the case.
 Wait...what? Did I just say that a value listed in one of the aforementioned Run keys may not, in fact, be executed at system start or user login??
 Yes...yes, I did.
 Let's first start with validating that the entries themselves have been run. We know that we can parse the Microsoft-Windows-Shell-Core%4Operational Event Log, extracting event ID 9707 and 9708 events to see when execution of the values beneath the Run (and RunOnce ) keys was started, and then completed (respectively). We can use whatever tool we like to open or parse the Windows Event Log file, and the filter through it to see that, yes, on this date, at this time, these entries were, in fact, launched. That's a great way to validate our findings, based on the entries in the Run key.
 It happens that there's another set of Registry keys at play:
 [HKLM|HKCU]\Software\Microsoft\Windows\CurrentVersion\Explorer\ StartupApproved\Run
 If you navigate to the above key, you'll see that the value names within the key reflect the value names beneath the corresponding Run key, but that the data is different. Figure 1 illustrates the data from one of my systems.

 As you see in figure 1, the value names reflect entries in the Run key, but the values all have different data. First, the binary data for each values is 12 bytes in length. If the first 4 bytes (DWORD) is 0x02 or 0x06, then the corresponding value in the Run key is enabled. If the first DWORD is 0x03, then the value is disabled, and the remaining 8 bytes (QWORD) is the FILETIME object representing the data and time that the value was disabled. Figure 2 illustrates this data parsed via RegRipper Pro:

 Now, here's the kicker...what you see in figures 1 and 2 reflect what happens if the values are disabled via the Startup tab in Task Manager. If you use RegEdit or reg.exe to navigate to the Run keys, you can simply delete entries to prevent them from executing. However, if you use the Task Manager Startup tab to enumerate startup entries, and you disable entries that are found in the Run keys, you see what is illustrated in figures 1 and 2.
 What's really interesting about this StartupApproved Registry key is that there's another subkey:
 [HKLM|HKCU]\Software\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\StartupFolder
 As you might have guessed by the name, this applies to the Startup folder in the file system. Yes, MS has added Registry key that, if there are entries in the corresponding Startup folder, there will also be a StartupFolder key what contains value names that mirror those entries. On systems where there are no entries in the Startup folder for the account, I have not found a corresponding StartupFolder key.
 Figure 3 illustrates what this looks like via RegRipper Pro.

 If you use Task Manager to re-enable the values, the data for the corresponding entry beneath the StartupApproved key is changed back to the "enabled" value (first DWORD = 0x02, remaining QWORD all 0s).
 At this point, the takeaway is, rather that just checking the Run key, correlate the entries the value data within the StartupApproved\Run key, and validate both via the corresponding event IDs in the Microsoft-Windows-Shell-Core%4Operational Event Log.
 But wait, there's more!
 If the entries in the Run key were instead disabled via Autoruns , something entirely different happens . If a value is disabled beneath the Run key, then when Autoruns is closed, an AutorunsDisabled subkey is created beneath the Run key, and the disabled value is moved to that subkey. If an entry beneath the Startup folder is disabled via Autoruns, an AutorunsDisable subdirectory is created beneath the Startup folder, and the entry (shortcut, executable, batch file, etc.) is moved to that subdirectory.
 So What?
 Why does any of this matter? Well, first off, it means that if we assume that just because there's an entry in a Run key, that the application pointed to was actually executed, we may be wrong. If anything, this reinforces the need to validate and correctly understand findings and artifacts.
 How could this be used? Well, one thought is that a threat actor could create a distraction; by putting an entry in a Run key and then disabling it in the corresponding StartupApproved\Run key, admins and analysts might be caught unaware and pursue a line of analysis that actually takes them down a rabbit hole, and cause them not to investigate the real issue.
 Think that's a bit far fetched? I've seen it happen .
