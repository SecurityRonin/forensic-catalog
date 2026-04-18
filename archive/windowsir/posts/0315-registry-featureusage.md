# Registry: FeatureUsage

- URL: https://windowsir.blogspot.com/2025/11/registry-featureusage.html
- Published: 2025-11-26T09:40:00.001-05:00
- Updated: 2025-11-26T09:40:08.112-05:00
- Labels: none

Maurice posted on LinkedIn recently about one of the FeatureUsage Registry key subkeys; specifically, the AppSwitched subkey. Being somewhat, maybe even only slightly aware of the Windows Registry, I read the post with casual, even mild interest.
 Someone posted recently that cybersecurity runs on caffeine and sarcasm...I've got my cup of coffee right in front of me, and I've placed the sarcasm in front of us all.  ;-)

 After finding and reading the blog post, I wrote a brief blog post here on this blog that mentioned the Registry key, and referenced Jai's blog post.
 To Maurice's point, this is, in fact, a valuable artifact, so kudos to Maurice for pointing it out and mentioning it again. If you read through the comments to Maurice's post, there are some hints there as to how to use the value data in your analysis. As you'll note, neither the value names nor the data itself  includes a timestamp, but the AppSwitched subkey does have a LastWrite time, and this can be included in a timeline. The value names can be used as pivot points into a timeline, adding something of a "third dimension" to timeline analysis. You can use this alongside timestamped information, such as Prefetch entries, Registry data, SRUM DB data, etc.
