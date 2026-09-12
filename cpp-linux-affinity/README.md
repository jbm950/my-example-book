Simple example to showcase how to pin work to a specific CPU core (set
affinity). Note, the CPU can still be time shared with other tasks. This
doesn't ensure that the program is the only thing running on that thread, just
that the program doesn't run on any other threads.
