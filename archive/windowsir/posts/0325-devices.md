# Devices

- URL: https://windowsir.blogspot.com/2026/02/devices.html
- Published: 2026-02-02T14:07:00.002-05:00
- Updated: 2026-03-02T13:42:34.049-05:00
- Labels: none

As such, it's not only important to keep up with what's available from the default installation of an endpoint, and what sources can be used to validate your findings, but also note what's available depending upon the applications installed.
 However, what's most important are your analysis goals. For example, it's easy to say, "I want to see all USB devices connected to the endpoint", and not provide a timeframe. However, to what are these devices pursuant? IP theft? Violation of acceptable use policies? What about CSAM production? After all, smartphones and digital cameras can be connected to a computer via a USB cable, but use different protocols, and information about the devices being connected can be found in other locations than those that we traditionally look to as a result of training we may receive.
 When it comes to devices connected to a Windows system, things have definitely changed over time. Cory and I published a peer-reviewed paper in 2005 , covering USB artifacts on Windows XP systems. Since then, there's been quite a few changes to Windows, and as one would expect, how devices connected to an endpoint are tracked. Nicole mentioned some time ago (2014) that different protocols used by different devices (in this case, smartphones) have an impact on investigations; unfortunately, as time has gone on, I've had a hard time linking back to Nicole's published research or even her presentations, but one of her presentations can be found here .
 This WindowsIR blog post describes the fact that some of the Windows Event Log files we might look to in order to investigate USB devices connected to an endpoint have been seen to be disabled, and recommended some alternative data sources. Similarly, this NirSoft page for the USBDeView application similarly recommends data sources.
 Speaking of the WindowsIR blog, here's a whole series of blog posts about just "USB devices".
 Now, as to why I'm writing this blog post, in the first place...
 I recently ran across this LinkedIn post from Garrett Moreau, which I thought was interesting. He's right, in that information about devices connected to/removed from the endpoint is stored in the Registry, and not just in the USBStor key. RegRipper , for example, includes multiple plugins within the "devices" category. What's really eye-opening is to read through the comments to the post; one comment, for example, states the following:
 Quick note for defenders, default logging settings don’t provide much depth.
 USB events are recorded under the Applications and Services Logs > Microsoft > Windows > DriverFrameworks-UserMode > Operational path, but this log must be activated manually.

 Previous versions of Windows required activation of these events using PowerShell.

 While the statement is true, this goes back to what I said earlier about traditional training, and the need to not only keep playbooks up to date, but to also pursue other alternative data sources. A data source we're familiar with, the Microsoft-Windows-DriverFrameworks-UserMode Event Log , is now disabled by default, and as stated, doesn't provide "much depth". However, if understanding the USB devices connected to an endpoint are part of your investigative goals, then keeping playbooks updated is key.
 Some of the comments go one to suggest other data sources that might be used to add granularity or context to your investigations, such as shellbags (for directory transversal), etc. In the past, I've found the Application Event Log, and in particular, MsiInstaller events to be useful in illustrating attempts to install applications from a USB device.
 My recommendation is this...first and foremost, understand your analysis goals; what are you attempting to show, to prove or disprove? From there, maintain playbooks associated with these goals, and if you're in a role where you're dealing with different implementations and applications, keep documentation on any new findings that apply to specific edge cases, as you never know when you will see such things again. An excellent example of this is when Don Weber worked a case where the LANDesk Software Monitoring application was installed on an endpoint, and found that the application provided valuable information regarding program execution .

 Addendum, 2 Mar : Just yesterday, I became aware of this USB Devices article from ElComSoft, which addresses different types of USB-connected devices, and resources for investigating them.
