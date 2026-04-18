# Windows Defender Support Logs

- URL: https://windowsir.blogspot.com/2026/01/windows-defender-support-logs.html
- Published: 2026-01-01T19:20:00.001-05:00
- Updated: 2026-01-01T19:20:47.306-05:00
- Labels: none

C:\ProgramData\Microsoft\Windows Defender\Support\
 ...and follow the naming convention:
 MpWppTracing-YYYYMMDD-HHMMSS-00000003-fffffffeffffffff.bin
 The post mentions using strings to parse the files, but I was wondering if there was a parser available, and like Deadpool, I figured I'd go looking...and I found something called mplog_parser . I've had a few opportunities to pull down some of these files from endpoints, but nothing has popped out as being related to the incident in question.
 That's okay, though...I'll keep this one in my kit, and I'll have to give the parser from Github a shot.
