# StartupApproved\Run, pt II

- URL: https://windowsir.blogspot.com/2022/07/startupapprovedrun-pt-ii.html
- Published: 2022-07-17T08:26:00.001-05:00
- Updated: 2022-07-17T08:26:41.449-05:00
- Labels: none

On the heels of my last blog post on this topic , I had a couple of thoughts and insights that I wanted to research a bit, and then address. I wanted to take a look at ways that the StartupApproved\Run key might be impacted, so I started by grabbing the contents of that key based on what we saw from the previous post, which are illustrated in figure 1.

 Then, I captured the contents of the Run key, illustrated in figure 2.

 As you can see in figure 2, there appears to be an entry missing, the "com.squirrel.Teams.Teams" value. We know from the previous blog post that this value was disabled on 14 Jul 2021, just over a year ago. I have no idea how that happened, as it wasn't part of an intentional test at the time, and was just a matter of me not wanting Teams to launch every time I logged in.

 As part of this research effort, I deleted the OneDrive value from the Run key (see figure 2 above) via RegEdit, and rebooted the system. When I re-opened RegEdit and navigated to the Run key in my user hive, I confirmed that the OneDrive value was no longer in the Run key. However, when I navigated to the corresponding StartupApproved\Run key, I found that the corresponding OneDrive value still appeared as illustrated in figure 1. From this then, yes, it appears that if you delete a value from the Run key via RegEdit, that entry is not removed from the corresponding StartupApproved\Run key.
 For step 2 in this experiment, I added a value to the Run key via RegEdit; I created a new string value, named it "Calc", and then added the path, "C:\Windows\system32\calc.exe". I rebooted the system, logged in, and the calculator opened on my desktop...but there was no "Calc" value in the corresponding StartupApproved\Run key!

 I then removed the Calc value via RegEdit, and then typed the following command:

 reg add HKCU\Software\Microsoft\Windows\CurrentVersion\Run /v Calc /t REG_SZ /d C:\windows\system32\calc.exe /f

 After ensuring that the command succeeded, I looked at the contents of the Run key via RegEdit and could see the new value. However, I could not see a corresponding value in the StartupApproved\Run key!

 Finally, after having removed the "calc" value from the Run key, I added it back via RegEdit, and then opened the Task Manager Startup Tab to see the "Windows Calculator" value. I then disabled the value via the Startup Tab, and verified that a "calc" value was added to the StartupApproved\Run key, as illustrated in figure 3.

 So, the question becomes, how do entries make it into the StartupApproved\Run key? If neither the use of RegEdit nor reg.exe to add a value to the Run key explicitly lead to corresponding values being added to the StartupApproved\Run key (say, by the operating system), then how are they added? Looking back at figure 1, all of the entries in the StartupApproved\Run key were for applications that were added via an installation process, such as an MSI or a setup.exe file. Maybe this is what needs to be better understood and addressed about these keys.
