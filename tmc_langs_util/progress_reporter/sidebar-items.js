initSidebarItems({"struct":[["ProgressReporter","The reporter contains a RefCell for the timer, meaning care should be taken when using in a multithreaded context. The first call to progress and each step completion should be called from one thread. Other progress calls can be done from separate threads."],["StatusUpdate","The format for all status updates. May contain some data."]]});