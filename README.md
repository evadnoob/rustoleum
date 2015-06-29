# Introduction

bldr is a distributed build system. Why? mainly to learn Rust. But
there are several other reasons, I list some of the goals below, but
primarily, I'm just interested in building systems, and I really enjoy
build systems, and automating developer tasks. I've used Jekins,
Hudson, Bamboo. I've never really enjoyed them, they always seem like
they are just good enough. All seem built on Java/JVM technology, born
out of the 2000s, when the jvm was king. We're entering a new era of
runtime-less languages.



# bldr demo

In 1st terminal

    $ ./target/debug/bldr agent
    2015-05-28 09:25:53 INFO [bldr::agent::cluster:src/agent/cluster.rs:3] - joining cluster

In 2nd terminal

    $ ./target/debug/bldr repl
	> ping
	"ping"
	2015-05-28 09:26:16 INFO [bldr::agent::cluster:src/agent/cluster.rs:3] - joining cluster
	2015-05-28 09:26:16 INFO [bldr::agent::client:src/agent/client.rs:19] - sending ping 0
	2015-05-28 09:26:16 INFO [bldr::agent::client:src/agent/client.rs:23] - Received Yes she does: 0
	2015-05-28 09:26:17 INFO [bldr::agent::client:src/agent/client.rs:19] - sending ping 1
	2015-05-28 09:26:17 INFO [bldr::agent::client:src/agent/client.rs:23] - Received Yes she does: 1
	2015-05-28 09:26:18 INFO [bldr::agent::client:src/agent/client.rs:19] - sending ping 2
	2015-05-28 09:26:18 INFO [bldr::agent::client:src/agent/client.rs:23] - Received Yes she does: 2
	2015-05-28 09:26:19 INFO [bldr::agent::client:src/agent/client.rs:19] - sending ping 3
	2015-05-28 09:26:19 INFO [bldr::agent::client:src/agent/client.rs:23] - Received Yes she does: 3
	> quit

Meanwhile, back at the ranch

    2015-05-28 09:25:53 INFO [bldr::agent::cluster:src/agent/cluster.rs:3] - joining cluster
	2015-05-28 09:26:16 INFO [bldr::agent::agent:src/agent/agent.rs:18] - Received Zoe Rocks!!
	2015-05-28 09:26:17 INFO [bldr::agent::agent:src/agent/agent.rs:18] - Received Zoe Rocks!!
	2015-05-28 09:26:18 INFO [bldr::agent::agent:src/agent/agent.rs:18] - Received Zoe Rocks!!
	2015-05-28 09:26:19 INFO [bldr::agent::agent:src/agent/agent.rs:18] - Received Zoe Rocks!!

# goals

* develop a decentralized distributed build system, with agents doing the real work

* rust - be native, lightweight, and fast as shit.  Run as a daemon, use tried and true unix apis.

* focus on scripting as the configuration of a job, bash, python, ruby etc.

* Low administrative overhead, focus on workflows through the cli, don't get caughtup with fancy user interfaces(they're all fads and they look terrible in eventually).

* Allow anyone to access the rest interfaces to develop any user-interface they want.

* job control mainly through command line and chat/bot interfaces

* decentralized logging - event system communicates log, after logging locally to a file, fire event for log messages(buffer and compress these logs).

* fault tolerant leader election(paxos/raft?)

* clients that can re-attach to their leader, once the leader has moved

* agents run in docker containers, as well as the leader

* leader and agent are just modules, an agent can become the leader when the leader is lost(paxos, raft?)

* be better than Jenkins(and buildbot, etc.) at everything.

* be a little like quartz, only better at everything also.

* run in Docker ( or not ).

# Features

* masterless. All agents are capable of being a leader. 

* templates for builds deploys based on common scenarios sbt, maven, cargo, npm etc.
** do build in docker container
** run build in ec2 instance or other cloud provider


* event oriented - use an event bus to communicate build events, failures, starts, re-starts, custom messages, lifecycle events.

* prioritize mvn, npm, etc. as first couple of builders

* Chat
** notify committer directly based on preference of their build failures through chat
** allow interaction with build job through chat.

* labels on "agents", jobs can provide labels and agents consume jobs by matching labels labels.

* interactive build repl with colorized display

# Modules

#### Storage
* storage is part configuration, part history, job definition, etc.
  Storage is implemented as a git repository, local.  We use git as a
  version control database.

#### Job Control
* job control / scheduler - command executor

#### Logging
* logging - agents log, the logs are viewable from any agent or repl.

#### Agent
* build agent - process build and deployment jobs, log locally and broadcast log messages.

#### Repl
* repl - (the repl is going away)an interactive command line interface for dealing with the agents/jobs/schedules etc. 

#### Cluster/Group Management
* leadership mgmt/discovery - etcd/zookeeper/paxos/raft like


# Development

    $ cargo build
    $ ./target/debug/bldr agent
    $ ./target/debug/bldr repl

# Contributing
Rustoleum is just getting started.  The name is really just a placeholder, if it sticks fine, but I'm sure there must be a better name.

If you're interested in contributing, jump on irc, and fork the repo. 

```#rustoleum on irc.freenode.net```

# Building from source

* clone the repo
* ```brew install zeromq```
* ```brew install nanomsg```
* ```cargo build```
* ```target/debug/bldr repl``` or ```target/debug/bldr agent```
