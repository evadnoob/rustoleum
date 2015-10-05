this is a placeholder.


Example adding a job:

```

target/debug/bldr add job '{"name": "test_xyz", "repository": {"repo_type": "Github"}, "description": "thisa test"}'
args: -h, --help => Switch(false)
--port => Plain(None)
-v, --verbose => Switch(false)
<json> => Plain(Some("{\"name\": \"test_xyz\", \"repository\": {\"repo_type\": \"Github\"}, \"description\": \"thisa test\"}"))
<peers> => List([])
add => Switch(true)
agent => Switch(false)
init => Switch(false)
job => Switch(true)
repl => Switch(false)
show => Switch(false)
storage => Switch(false)
arg vector: []
2015-07-05 09:10:38 INFO [bldr::storage:src/storage.rs:53] - The current directory is /Users/daveboon/Projects/rustoleum
2015-07-05 09:10:38 INFO [bldr::storage:src/storage.rs:65] - does file exist ? "/Users/daveboon/Projects/rustoleum"
2015-07-05 09:10:38 INFO [bldr::storage:src/storage.rs:81] - skipped repo initialization, directory already exists.
2015-07-05 09:10:38 INFO [bldr::storage:src/storage.rs:88] - opened repo "/Users/daveboon/Projects/rustoleum/bldr-repo-data/.git/"
2015-07-05 09:10:38 INFO [bldr::storage:src/storage.rs:93] - statuses 0
2015-07-05 09:10:38 INFO [bldr::storage:src/storage.rs:94] - bootstrap done
2015-07-05 09:10:38 INFO [bldr:src/main.rs:105] - json: {"name": "test_xyz", "repository": {"repo_type": "Github"}, "description": "thisa test"}
2015-07-05 09:10:38 INFO [bldr::jobs:src/jobs.rs:12] - from_raw_json: {"name": "test_xyz", "repository": {"repo_type": "Github"}, "description": "thisa test"}
2015-07-05 09:10:38 INFO [bldr::jobs:src/jobs.rs:20] - decoded Job { name: "test_xyz", description: Some("thisa test"), repository: RepositoryDescriptor { name: None, url: None, repo_type: Github } }
2015-07-05 09:10:38 INFO [bldr::storage:src/storage.rs:237] - save
2015-07-05 09:10:38 INFO [bldr::storage:src/storage.rs:242] - adding...{
  "name": "test_xyz",
  "description": "thisa test",
  "repository": {
    "name": null,
    "url": null,
    "repo_type": "Github"
  }
}
2015-07-05 09:10:38 INFO [bldr::storage:src/storage.rs:243] - job_as_json_path /Users/daveboon/Projects/rustoleum/bldr-repo-data/job_test_xyz.json
2015-07-05 09:10:38 INFO [bldr::storage:src/storage.rs:247] - created file, ready for writing
2015-07-05 09:10:38 INFO [bldr::storage:src/storage.rs:258] - opened repo "/Users/daveboon/Projects/rustoleum/bldr-repo-data/.git/"
2015-07-05 09:10:38 INFO [bldr::storage:src/storage.rs:121] - path: job_test_xyz.json
2015-07-05 09:10:38 INFO [bldr::storage:src/storage.rs:136] - status length 1
2015-07-05 09:10:38 INFO [bldr::storage:src/storage.rs:143] - stage
2015-07-05 09:10:38 INFO [bldr::storage:src/storage.rs:156] - add 'job_test_xyz.json'
2015-07-05 09:10:38 INFO [bldr::storage:src/storage.rs:169] - added "/Users/daveboon/Projects/rustoleum/bldr-repo-data/job_test_xyz.json"
2015-07-05 09:10:38 INFO [bldr::storage:src/storage.rs:121] - path: job_test_xyz.json
2015-07-05 09:10:38 INFO [bldr::storage:src/storage.rs:133] - job_test_xyz.json modified:
2015-07-05 09:10:38 INFO [bldr::storage:src/storage.rs:136] - status length 1
2015-07-05 09:10:38 INFO [bldr::storage:src/storage.rs:173] - index write ok.
2015-07-05 09:10:38 INFO [bldr::storage:src/storage.rs:179] - commit
2015-07-05 09:10:38 INFO [bldr::storage:src/storage.rs:187] - commit: 98b360a6e150e11d31f01ba9ff175126f8228da8
got raw json ok.

```

