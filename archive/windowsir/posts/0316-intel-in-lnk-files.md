# Intel in LNK Files

- URL: https://windowsir.blogspot.com/2025/11/intel-in-lnk-files.html
- Published: 2025-11-29T15:48:00.000-05:00
- Updated: 2025-11-29T15:48:53.056-05:00
- Labels: none

However, what did interest me more was that the threat actor's efforts included an LNK, something that had to be created on the threat actor's infrastructure before it was included in the archive. As such, from an intel perspective, LNK files are "free money", and something I've talked about here in this blog more than a few times.
 Using the hash provided in the write-up, I was able to find a sample to download and parse myself. The LNK file itself had very little actual metadata beyond what was shared in the write-up, but that was still very interesting to me.
 Take a look at the full set of metadata:
 guid               {00021401-0000-0000-c000-000000000046}
 shitemidlist       My Computer/C:\/Windows/system32/cmd.exe
 **Shell Items Details (times in UTC)**
 C:0                   M:0                   A:0                  Windows  (9)
 C:0                   M:0                   A:0                  system32  (9)
 C:0                   M:0                   A:0                  cmd.exe  (9)
 commandline        /c ftp.exe -s:"offsec-certified-professional.png"
 iconfilename       %ProgramFiles(x86)%\\Microsoft\\Edge\\Application\\msedge.exe
 hotkey             0x0
 showcmd            0x1
 ***LinkFlags***
 HasLinkTargetIDList|IsUnicode|HasArguments|HasIconLocation
 ***PropertyStoreDataBlock***
 GUID/ID pairs:
 {46588ae2-4cbc-4338-bbfc-139326986dce}/4      SID: S-1-5-21-1526495471-1806070692-3097244026-1000
 ***KnownFolderDataBlock***
 GUID  : {1ac14e77-02e7-4e5d-b744-2eb1ae5198b7}
 Folder: CSIDL_SYSTEM
 We can see that the shell item time stamps are zero'd out, there's no machineID or NetBIOS name listed, no volume serial number, etc. The dearth of metadata can be just as important, or even more so, than when an LNK file contains much more metadata. My most recent blog post on LNK file metadata , prior to this post, illustrates an LNK file that is rife with metadata.
 So, the LNK file used in the Seqrite campaign may be the result of using a specific tool to create the LNK file, or it may be the result of applying a process to the LNK file, after it was created, to "scrub" a lot of the metadata that we might expect to see. Either way, tracking this information can be very valuable for CTI teams, as the available metadata tells us something about the adversary. Also, strings such as the SID shown can be used to search for other, similar samples, in an effort to round out the adversary's intel picture.
