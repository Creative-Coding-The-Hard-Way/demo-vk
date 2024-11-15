searchState.loadedDescShard("inotify", 0, "Idiomatic inotify wrapper for the Rust programming language\nFile was accessed\nFile was accessed\nWatch for all events\nMetadata (permissions, timestamps, …) changed\nMetadata (permissions, timestamps, …) changed\nWatch for all close events\nFile or directory not opened for writing was closed\nFile or directory not opened for writing was closed\nFile opened for writing was closed\nFile opened for writing was closed\nFile/directory created in watched directory\nFile/directory created in watched directory\nFile/directory deleted from watched directory\nFile/directory deleted from watched directory\nWatched file/directory was deleted\nWatched file/directory was deleted\nDon’t dereference the path if it is a symbolic link\nFilter events for directory entries that have been unlinked\nAn inotify event\nIndicates the type of an event\nAn owned version of <code>Event</code>\nIterator over inotify events\nWatch was removed\nEvent related to a directory\nIdiomatic Rust wrapper around Linux’s inotify API\nIf a watch for the inode exists, amend it instead of …\nFile was modified\nFile was modified\nWatch for all move events\nFile was renamed/moved; watched directory contained old …\nFile was renamed/moved; watched directory contained old …\nFile was renamed/moved; watched directory contains new name\nFile was renamed/moved; watched directory contains new name\nWatched file/directory was moved\nWatched file/directory was moved\nOnly receive one event, then remove the watch\nOnly watch path, if it is a directory\nFile or directory was opened\nFile or directory was opened\nEvent queue overflowed\nFile system containing watched object was unmounted. File …\nRepresents a watch on an inode\nDescribes a file system watch\nInterface for adding and removing watches\nAdds or updates a watch for the given path\nDeprecated: use <code>Inotify.watches().add()</code> instead\nReturns the set containing all flags.\nReturns the set containing all flags.\nReturns the intersection between the two sets of flags.\nReturns the intersection between the two sets of flags.\nDisables all flags disabled in the set.\nDisables all flags disabled in the set.\nReturns the union of the two sets of flags.\nReturns the union of the two sets of flags.\nAdds the set of flags.\nAdds the set of flags.\nReturns the raw value of the flags currently stored.\nReturns the raw value of the flags currently stored.\nReturns the left flags, but with all the right flags …\nReturns the left flags, but with all the right flags …\nToggles the set of flags.\nToggles the set of flags.\nCloses the inotify instance\nReturns the complement of this set of flags.\nReturns the complement of this set of flags.\nReturns <code>true</code> if all of the flags in <code>other</code> are contained …\nReturns <code>true</code> if all of the flags in <code>other</code> are contained …\nConnects related events to each other\nConnects related events to each other\nReturns the difference between the flags in <code>self</code> and <code>other</code>.\nReturns the difference between the flags in <code>self</code> and <code>other</code>.\nReturns an empty set of flags.\nReturns an empty set of flags.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nConvert from underlying bit representation, unless that …\nConvert from underlying bit representation, unless that …\nConvert from underlying bit representation, dropping any …\nConvert from underlying bit representation, dropping any …\nConvert from underlying bit representation, preserving all …\nConvert from underlying bit representation, preserving all …\nGet the inotify event buffer size for an absolute path\nGet the inotify event buffer size\nGetter method for a watcher’s id.\nCreates an <code>Inotify</code> instance\nInserts the specified flags in-place.\nInserts the specified flags in-place.\nReturns the intersection between the flags in <code>self</code> and …\nReturns the intersection between the flags in <code>self</code> and …\nReturns <code>true</code> if there are flags common to both <code>self</code> and …\nReturns <code>true</code> if there are flags common to both <code>self</code> and …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns an owned copy of the event.\nReturns <code>true</code> if all flags are currently set.\nReturns <code>true</code> if all flags are currently set.\nReturns <code>true</code> if no flags are currently stored.\nReturns <code>true</code> if no flags are currently stored.\nIndicates what kind of event this is\nIndicates what kind of event this is\nThe name of the file the event originates from\nThe name of the file the event originates from\nReturns the complement of this set of flags.\nReturns the complement of this set of flags.\nReturns one buffer’s worth of available events\nWaits until events are available, then returns them\nRemoves the specified flags in-place.\nRemoves the specified flags in-place.\nStops watching a file\nDeprecated: use <code>Inotify.watches().remove()</code> instead\nInserts or removes the specified flags depending on the …\nInserts or removes the specified flags depending on the …\nReturns the set difference of the two sets of flags.\nReturns the set difference of the two sets of flags.\nDisables all flags enabled in the set.\nDisables all flags enabled in the set.\nReturns the symmetric difference between the flags in <code>self</code> …\nReturns the symmetric difference between the flags in <code>self</code> …\nReturns an owned copy of the event.\nToggles the specified flags in-place.\nToggles the specified flags in-place.\nReturns the union of between the flags in <code>self</code> and <code>other</code>.\nReturns the union of between the flags in <code>self</code> and <code>other</code>.\nGets an interface that allows adding and removing watches. …\nIdentifies the watch this event originates from\nIdentifies the watch this event originates from")