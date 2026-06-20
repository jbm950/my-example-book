An example of a server that takes messages from clients and echoes them to all
the other connected clients. Note, the network reader uses a buffer and .line()
which may have to be changed if sending something other than regular text like
serialized data.
