window.SIDEBAR_ITEMS = {"enum":[["AddressFamily","These constants specify the protocol family to be used in `socket` and `socketpair`"],["ControlMessage","A type-safe zero-copy wrapper around a single control message, as used wih `sendmsg`.  More types may be added to this enum; do not exhaustively pattern-match it."],["ControlMessageOwned","A type-safe wrapper around a single control message, as used with `recvmsg`."],["InetAddr",""],["IpAddr",""],["Shutdown",""],["SockAddr","Represents a socket address"],["SockProtocol","Constants used in `socket` and `socketpair` to specify the protocol to use."],["SockType","These constants are used to specify the communication semantics when creating a socket with `socket()`"]],"fn":[["accept","Accept a connection on a socket"],["accept4","Accept a connection on a socket"],["bind","Bind a name to a socket"],["connect","Initiate a connection on a socket"],["getpeername","Get the address of the peer connected to the socket `fd`."],["getsockname","Get the current address to which the socket `fd` is bound."],["getsockopt","Get the current value for the requested socket option"],["listen","Listen for connections on a socket"],["recv","Receive data from a connection-oriented socket. Returns the number of bytes read"],["recvfrom","Receive data from a connectionless or connection-oriented socket. Returns the number of bytes read and, for connectionless sockets,  the socket address of the sender."],["recvmmsg","An extension of `recvmsg` that allows the caller to receive multiple messages from a socket using a single system call. This has performance benefits for some applications."],["recvmsg","Receive message in scatter-gather vectors from a socket, and optionally receive ancillary data into the provided buffer. If no ancillary data is desired, use () as the type parameter."],["send","Send data to a connection-oriented socket. Returns the number of bytes read"],["sendmmsg","An extension of `sendmsg` that allows the caller to transmit multiple messages on a socket using a single system call. This has performance benefits for some applications."],["sendmsg","Send data in scatter-gather vectors to a socket, possibly accompanied by ancillary data. Optionally direct the message at the given address, as with sendto."],["sendto","Send a message to a socket"],["setsockopt","Sets the value for the requested socket option"],["shutdown","Shut down part of a full-duplex connection."],["sockaddr_storage_to_addr","Return the appropriate `SockAddr` type from a `sockaddr_storage` of a certain size."],["socket","Create an endpoint for communication"],["socketpair","Create a pair of connected sockets"]],"mod":[["sockopt",""]],"struct":[["AlgAddr",""],["CmsgIterator",""],["IpMembershipRequest","Request for multicast socket operations"],["Ipv4Addr",""],["Ipv6Addr",""],["Ipv6MembershipRequest","Request for ipv6 multicast socket operations"],["LinkAddr","Hardware Address"],["MsgFlags","Flags for send/recv and their relatives"],["NetlinkAddr",""],["RecvMmsgData",""],["RecvMsg",""],["SendMmsgData",""],["SockFlag","Additional socket options"],["UnixAddr","A wrapper around `sockaddr_un`."],["UnixCredentials","Unix credentials of the sending process."],["VsockAddr",""],["cmsghdr",""],["msghdr",""],["sockaddr",""],["sockaddr_in",""],["sockaddr_in6",""],["sockaddr_storage",""],["sockaddr_un",""]],"trait":[["GetSockOpt","Represents a socket option that can be accessed or set. Used as an argument to `getsockopt`"],["SetSockOpt","Represents a socket option that can be accessed or set. Used as an argument to `setsockopt`"]],"type":[["sa_family_t",""]]};