# raskd

**WIP**

This project is currently WIP. I haven't packaging this yet. So for now, if you
want to use this, you need to clone the repo and compile locally.

**NOTE**

Windows user may run into some problems as some packages may not support
windows. Linux and MacOs should be fine. (if not, **PR** are very welcome)

---

## How to setup

1. clone the repo 
2. go to the directory, run `cargo build --release`. This will compile the
   release version of the binary.
3. copy the binary to your path. You can find the binary at `target/release/`.
   There are two binaries for this package, the `raskd` and `rask`. `raskd` is
   the server that listen on the request whereas the `rask` is the command line
   tool to interact with the `raskd` server.
4. before you can start `raskd`, you need to run `raskd init` to initialize the
   sqlite3 database. The default database name is `raskd.db` and it will be
   stored at the directory where you put your `raskd` binary.
5. after the init, you can run `raskd &` if it is in your path, or `./raskd &`
   if the binary is at the current directory. This should start the `raskd`
   server in the background.
6. whenever you need to create a task, you can use `rask` cli to do this. See
   the [wiki](https://github.com/LMJW/raskd/wiki) page to see more use cases.

---

## How it looks like

### init sqlite3 db if you haven't
```
rustd init
```

### start the server in background
```
rustd &
```

### start a task
```
rask start 'leetcode daily'

+----+----------------+-----------+----------+---------------------------------+---------+
| id | name           | task_type | duration | start_at                        | stop_at |
+----+----------------+-----------+----------+---------------------------------+---------+
| 1  | leetcode daily | task      | 00:00:00 | Sun, 19 Apr 2020 17:03:53 +1000 |         |
+----+----------------+-----------+----------+---------------------------------+---------+
```

### list tasks
```
rask ls

+----+----------------+-----------+----------+---------------------------------+---------+
| id | name           | task_type | duration | start_at                        | stop_at |
+----+----------------+-----------+----------+---------------------------------+---------+
| 1  | leetcode daily | task      | 00:38:57 | Sun, 19 Apr 2020 17:03:53 +1000 |         |
+----+----------------+-----------+----------+---------------------------------+---------+
```

### stop task
```
rask stop 1

+----+----------------+-----------+----------+---------------------------------+---------------------------------+
| id | name           | task_type | duration | start_at                        | stop_at                         |
+----+----------------+-----------+----------+---------------------------------+---------------------------------+
| 1  | leetcode daily | task      | 00:39:02 | Sun, 19 Apr 2020 17:03:53 +1000 | Sun, 19 Apr 2020 17:42:55 +1000 |
+----+----------------+-----------+----------+---------------------------------+---------------------------------+
```
