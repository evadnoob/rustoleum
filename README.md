# goals

## develop a distributed build system, with agents doing the real work

## learn rust

## focus on scripting as the configuration of a job, bash, python, ruby etc.

## Low administrative overhead, focus on workflows through the cli, don't get caught.

## job controll mainly through command line and chat/bot interfaces

## centralized log rollup

## fault tolerant leader election(paxos/raft?)

## clients that can re-attach to their leader, once the leader has moved

## agents run in docker containers, as well as the leader

## leader and agent are just modules, an agent can become the leader when the leader is lost(paxos)

## be better than Jenkins at everything.

## be a little like quartz, only better at everything also.

# Features

## master/slave model

## templates for builds deploys based on common scenarios sbt, maven, cargo, npm etc.

## prioritize mvn, npm, etc. as first couple of builders

## notify committer direclty based on preference of their build failures through chat, allow interaction with build job through chat.

## labels on "nodes", jobs can provide labels and nodes that expose those labels can be used to perform tasks that match labels.

## interactive build repl with colorized display

# Modules

## job control / scheduler - command executor

## logging -

## build agent -

## leadership mgmt/discovery - etcd/zookeeper/paxos/raft like
