# pinecrest-rss

A project to scrape sermons from [this web page](https://www.pinecrestbaptistcharleston.org/from-the-pulpit) and allow them to be subscribed to using an [RSS feed reader](https://wiki.archlinux.org/title/list_of_applications#News,_RSS,_and_blogs)/[podcast client](https://wiki.archlinux.org/title/list_of_applications#Podcast_clients).

![gPodder loading the feed](https://user-images.githubusercontent.com/44626690/200210169-aab49826-b6a8-4e72-8879-4e17fd0bc5cf.png)

### Todo
 - [ ] See if I can get somewhat precise release dates from the metadata. All of the sermons seemingly have the recording date either in the title or description.
 - [X] ~Cache feed XML, and update it on a timed interval.~ It gets updated on the next request that occurs after 10 minutes has passed.
