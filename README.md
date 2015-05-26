# goals

* develop a decentralized distributed build system, with agents doing the real work

* rust - be native, lightweight, and fast as shit.  Run as a daemon, use tried and true unix apis.

* focus on scripting as the configuration of a job, bash, python, ruby etc.

* Low administrative overhead, focus on workflows through the cli, don't get caughtup with fancy user interfaces(they're all fads and they look terrible in eventually).

* Allow anyone to access the rest interfaces to develop any user-interface they want.

* job controll mainly through command line and chat/bot interfaces

* decentralized logging - event system communicates log, after logging locally to a file, fire event for log messages(buffer and compress these logs).

* fault tolerant leader election(paxos/raft?)

* clients that can re-attach to their leader, once the leader has moved

* agents run in docker containers, as well as the leader

* leader and agent are just modules, an agent can become the leader when the leader is lost(paxos, raft?)

* be better than Jenkins(and buildbot, etc.) at everything.

* be a little like quartz, only better at everything also.

* run in Docker ( or not ).

# Features

* masterless. All agents are capabable of being a leader. 

* templates for builds deploys based on common scenarios sbt, maven, cargo, npm etc.
** do build in docker container
** run build in ec2 instance or other cloud provider


* event oriented - use an event bus to communicate build events, failures, starts, re-starts, custom messages, lifecycle events.

* prioritize mvn, npm, etc. as first couple of builders

* Chat
** notify committer direclty based on preference of their build failures through chat
** allow interaction with build job through chat.

* labels on "agents", jobs can provide labels and agents consume jobs by matching labels labels.

* interactive build repl with colorized display

# Modules

## Storage
* storage is part configuration, part history, job definition, etc.
  Storage is implemented as a git repository, local.  We use git as a
  version control database.

* job control / scheduler - command executor

* logging - agents log, the logs are viewable from any agent or repl.

* build agent - process build and deployment jobs, log locally and broadcast log messages.

* repl - an interactive command line interface for dealing with the agents/jobs/schedules etc. 

* leadership mgmt/discovery - etcd/zookeeper/paxos/raft like


# Contributing
Rustoleum is just getting started.  The name is really just a placeholder, if it sticks fine, but I'm sure there must be a better name.

If you're interested in contributing, jump on irc, and fork the repo.  

```#rustoleum on irc.freenode.net```
