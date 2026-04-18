# Researching the Windows Registry

- URL: https://windowsir.blogspot.com/2022/08/researching-windows-registry.html
- Published: 2022-08-10T19:37:00.000-05:00
- Updated: 2022-08-10T19:37:35.402-05:00
- Labels: none

The Windows Registry is a magical place that I love to research because there's always something new and fun to find, and apply to detections and DFIR analysis! Some of my recent research topics have included default behaviors with respect to running macros in Office documents downloaded from the Internet, default settings for mounting ISO/IMG files, as well as how to go about enabling RDP account lockouts based on failed login attempts.
 Not long ago I ran across some settings specific to nested VHD files , and thought, well...okay, I've seen virtual machines installed on systems during incidents, as a means of defense evasion, and VHD/VHDX files are one such resource. Further, they don't require another application, like VMWare or VirtualBox.
 Digging a bit further, I found this MS documentation :
 "VHDs can be contained within a VHD, so Windows limits the number of nesting levels of VHDs that it will present to the system as a disk to two, with the maximum number of nesting levels specified by the registry value HKLM\System\CurrentControlSet\Services\FsDepends\Parameters\VirtualDiskMaxTreeDepth .
 Mounting VHDs can be prevented by setting the registry value HKLM\System\CurrentControlSet\Services\FsDepends\Parameters\VirtualDiskNoLocalMount to 1."
 Hhhmmm...so I can modify a Registry value and prevent the default behavior of mounting VHD files! Very cool! This is pretty huge, because admins can set this value to "1" within their environment, and protect their infrastructure.
 Almost 3 yrs ago, Will Dormann published an article about the dangers of VHD/VHDX files. Some of the issues Will points out are:
 - VHD/VHDX files downloaded from the Internet do not propagate MOTW the way some archive utilities do , so even if the VHD is downloaded from the Internet and MOTW is applied, this does not transfer to any of the files within the VHD file. This behavior is similar to what we see with ISO/IMG files.
 - AV doesn't scan inside VHD/VHDX files.
 So, it may be worth it to modify the VirtualDiskNoLocalMount value.
 To check the various settings from a DFIR perspective, I use RegRipper:
 (System) Get VHD[X] Settings
 ControlSet001\Services\FsDepends\Parameters
 LastWrite time: 2019-12-07 09:15:07Z
 VirtualDiskExpandOnMount  0x0001
 VirtualDiskMaxTreeDepth   0x0002
 VirtualDiskNoLocalMount   0x0000
 Analysis Tip: The values listed impact how Windows handles VHD[X] files, which can be used to bypass security measures, including AV and MOTW.
 VirtualDiskMaxTreeDepth determines how deep to do with embedding VHD files.
 VirtualDiskNoLocalMount set to 1 prevents mounting of VHD[X] files.
 Ref: https://insights.sei.cmu.edu/blog/the-dangers-of-vhd-and-vhdx-files/
 From what's in the Registry (above), we can see what's possible. In this case, per the Analysis Tip in the output of the RegRipper plugin, this system allows automatic mounting of the virtual disk file. You can look for access to .vhd/.vhdx files in the user's RecentDocs key. Also from a DFIR perspective, look for indications of files being mounted in the Microsoft-Windows-VHDMP%4Operational Event Log.
