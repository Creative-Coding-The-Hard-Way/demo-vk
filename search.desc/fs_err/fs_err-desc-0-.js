searchState.loadedDescShard("fs_err", 0, "fs-err is a drop-in replacement for <code>std::fs</code> that provides …\nWrapper around <code>std::fs::DirEntry</code> which adds more helpful …\nWrapper around <code>std::fs::File</code> which adds more helpful …\nWrapper around <code>std::fs::OpenOptions</code>\nDefines aliases on <code>Path</code> for <code>fs_err</code> functions.\nWrapper around <code>std::fs::ReadDir</code> which adds more helpful …\nSets the option for the append mode.\nReturns the canonical, absolute form of a path with all …\nCopies the contents of one file to another. This function …\nOpens a file in write-only mode.\nSets the option to create a new file, or open it if it …\nCreates a new, empty directory at the provided path.\nRecursively create a directory and all of its parent …\nSets the option to create a new file, failing if it …\nReturns a reference to the underlying <code>std::fs::File</code>.\nReturns a mutable reference to the underlying <code>std::fs::File</code>…\nReturns the file name of this directory entry without any …\nReturns the file type for the file that this entry points …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nWrapper for <code>OpenOptions::open</code>.\nConstructs <code>Self</code> from <code>std::fs::OpenOptions</code>\nCreates a <code>File</code> from a raw file and its path.\nReturns the canonical, absolute form of a path with all …\nGiven a path, query the file system to get information …\nReturns an iterator over the entries within a directory.\nReads a symbolic link, returning the file that the link …\nQuery the metadata about a file without following symlinks.\nReturns Ok(true) if the path points at an existing entity.\nCreates a new hard link on the filesystem.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nExtract the raw file and its path from this <code>File</code>\nGiven a path, query the file system to get information …\nReturns the metadata for the file that this entry points …\nQueries metadata about the underlying file.\nCreates a blank new set of options ready for configuration.\nAttempts to open a file in read-only mode.\nOpens a file at <code>path</code> with the options specified by <code>self</code>.\nReturns a reference to the underlying <code>std::fs::OpenOptions</code>.\nReturns a mutable reference to the underlying …\nOS-specific functionality.\nReturns the full path to the file that this entry …\nReturns a reference to the path that this file was created …\nRead the entire contents of a file into a bytes vector.\nSets the option for read access.\nReturns an iterator over the entries within a directory.\nReads a symbolic link, returning the file that the link …\nRead the entire contents of a file into a string.\nRemoves an empty directory.\nRemoves a directory at this path, after removing all its …\nRemoves a file from the filesystem.\nRename a file or directory to a new name, replacing the …\nTruncates or extends the underlying file, updating the …\nChanges the permissions found on a file or a directory.\nChanges the permissions on the underlying file.\nWrapper for <code>fs::soft_link</code>.\nQuery the metadata about a file without following symlinks.\nAttempts to sync all OS-internal metadata to disk.\nThis function is similar to [<code>sync_all</code>], except that it …\nSets the option for truncating a previous file.\nCreates a new <code>File</code> instance that shares the same …\nWrite a slice as the entire contents of a file.\nSets the option for write access.\nPlatform-specific extensions for Unix platforms.\nUnix-specific extensions to wrappers in <code>fs_err</code> for <code>std::fs</code> …\nWrapper for <code>std::os::unix::fs::FileExt</code>.\nWrapper for <code>std::os::unix::fs::OpenOptionsExt</code>\nWrapper for <code>OpenOptionsExt::custom_flags</code>\nWrapper for <code>OpenOptionsExt::mode</code>\nWrapper for <code>FileExt::read_at</code>\nCreates a new symbolic link on the filesystem.\nWrapper for <code>FileExt::write_at</code>")