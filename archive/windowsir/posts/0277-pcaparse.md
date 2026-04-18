# PCAParse

- URL: https://windowsir.blogspot.com/2024/02/pcaparse.html
- Published: 2024-02-26T20:40:00.001-05:00
- Updated: 2024-02-27T09:59:51.191-05:00
- Labels: none

I was doing some research recently regarding what's new to Windows 11, and ran across an interesting artifact, which seems to be referred to as "PCA". I found a couple of interesting references regarding this artifact, such as this one from Sygnia , and this one from AboutDFIR . Taking a look at the samples of files available from the DFIRArtifactMuseum , I wrote a parser for two of the files from the C:\Windows\appcompat\pca folder, converting the time stamps to Unix epoch format and sending the output to STDOUT, in TLN format so that it can be redirected to an events file.
 An excerpt from the output from the PcaAppLaunchDic.txt file:
 1654524437|PCA|||C:\ProgramData\ProtonVPN\Updates\ProtonVPN_win_v2.0.0.exe
 1661428304|PCA|||C:\Windows\SysWOW64\msiexec.exe
 1671064714|PCA|||C:\Program Files (x86)\Proton Technologies\ProtonVPN\ProtonVPN.exe
 1654780550|PCA|||C:\Program Files\Microsoft OneDrive\22.116.0529.0002\Microsoft.SharePoint.exe
 An excerpt from the output from the PcaGeneralDb0.txt file:
 1652387261|PCA|||%programfiles%\freefilesync\bin\freefilesync_x64.exe - Abnormal process exit with code 0x2
 1652387261|PCA|||%programfiles%\freefilesync\freefilesync.exe - Abnormal process exit with code 0x2
 1652391162|PCA|||%USERPROFILE%\appdata\local\githubdesktop\app-2.9.9\resources\app\git\cmd\git.exe - Abnormal process exit with code 0x80
 1652391162|PCA|||%USERPROFILE%\appdata\local\githubdesktop\app-2.9.9\resources\app\git\mingw64\bin\git.exe - Abnormal process exit with code 0x80
 This output can be redirected to an events file, and included in a timeline, so that we can validate that the artifact does, in fact, illustrate evidence of execution. Incorporating file system information, Prefect and Windows Event Log data (and any other on-disk resources), as well as EDR telemetry (if available) will provide the necessary data to validate program execution.
 Addendum, 2024-02-27 : Okay, so I've been actively seeking out opportunities to use this parser in my role at my day job, and while I've been doing so, some things have occurred to me. First, there's nothing in either file that points to a specific user, so incorporating this data into an overall timeline that includes WEVTX data and EDR telemetry is going to help not only validate the information from the file themselves, but provide the necessary insight around process execution, depending of course on the availability of information. Fossilization on Windows systems is a wonderful thing, but not everyone takes advantage of it, nor really understands where it's simply not going to be available.
 Not only is there no user information, there's also no information regarding process lineage. Still, I firmly believe that once we begin using this information in a consolidated timeline, and begin validating the information, we'll see that it adds yet another clarifying overlay to our timeline, as well as possible pivot points.
