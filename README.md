# pinecrest-rss

A project to scrape sermons from [this web page](https://www.pinecrestbaptistcharleston.org/from-the-pulpit) and allow them to be subscribed to using an [RSS feed reader](https://wiki.archlinux.org/title/list_of_applications#News,_RSS,_and_blogs)/[podcast client](https://wiki.archlinux.org/title/list_of_applications#Podcast_clients).

![Screenshot from 2022-11-14 20-23-53](https://user-images.githubusercontent.com/44626690/201803490-897b43e1-aa0c-44e1-901e-cc2f7484c9dc.png)

### Steps to Run:
```bash
cargo run
```
The RSS feed will be available at http://localhost:8001/sermons.rss

There is also a Prometheus metrics route at http://localhost:8001/metrics

### Todo
 - [X] See if I can get somewhat precise release dates from the metadata. All of the sermons seemingly have the recording date either in the title or description.
 - [X] ~Cache feed XML, and update it on a timed interval.~ It gets updated on the next request that occurs after 10 minutes has passed.
 - [ ] Tests!
 - [ ] Perhaps organize the modules a bit better.
