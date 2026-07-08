## mumu cli

OVERVIEW: A utility for control mumu player.

USAGE: <subcommand>

SUBCOMMANDS:
  version        Get player version.
  info           Get players info.
  create         Create players.
  clone          Clone players. (alias: copy)
  delete         Delete players.
  rename         Rename players.
  import         Import .mumudata files.
  export         Export players as .mumudata files.
  control        Control players.
  setting        Config players.
  adb            Run adb cmd for players.
  simulation     Change simulated properties in players.
  sort           Layout player windows to sort.
  driver         Manage player drivers.
  log            Control manager log.
  sh             Run player shell.

---

### Common: --vmindex / -v

Accepted by: info, create, clone, delete, rename, export, control, setting, adb, simulation, sh

  --vmindex 1          single player
  --vmindex 3,5,6,7    multiple players
  --vmindex all        all players

---

### version

USAGE: version
(no arguments)

---

### info

USAGE: info [-v <vmindex>]

---

### create

USAGE: create [-v <vmindex>] [-n <number>] [-m]

OPTIONS:
  -n, --number <number>    Number of players to create.
  -m, --mini               Set mini disk mode for data disk.

---

### clone

USAGE: clone [-v <vmindex>] [-n <number>]

OPTIONS:
  -n, --number <number>    Number of clones to create.

---

### delete

USAGE: delete [-v <vmindex>]

---

### rename

USAGE: rename [-v <vmindex>] [-n <name>]

OPTIONS:
  -n, --name <name>    New player name.

---

### import

USAGE: import [-p <path>] [-n <number>]

OPTIONS:
  -p, --path <path>        .mumudata file path. (one or more)
  -n, --number <number>    Number of action run.

Vital: import cannot target a slot — always creates a NEW instance; -n is a repeat count, not an index (10 ⇒ ten copies).

---

### export

USAGE: export [-v <vmindex>] [-d <dir>] [-n <name>] [-z]

OPTIONS:
  -d, --dir <dir>      Output directory.
  -n, --name <name>    Output file name.
  -z, --zip            Use compressed file format.

---

### control

USAGE: control [-v <vmindex>] <subcommand>

SUBCOMMANDS:
  launch           Launch players.
  shutdown         Shutdown players.
  restart          Restart players.
  show_window      Show player windows.
  hide_window      Hide player windows.
  layout_window    Layout player windows position and size.
  app              Control app in players.
  tool             Control toolbar in players.
  shortcut         Control shortcut in players.

---

### control layout_window

USAGE: {control [-v <vmindex>]} layout_window [-px <pos_x>] [-py <pos_y>] [-sw <size_w>] [-sh <size_h>]

OPTIONS:
  -px, --pos_x <pos_x>      X-axis of window position (screen left = 0).
  -py, --pos_y <pos_y>      Y-axis of window position (screen top = 0).
  -sw, --size_w <size_w>    Width of window.
  -sh, --size_h <size_h>    Height of window.

---

### control app

USAGE: {control [-v <vmindex>]} app <subcommand>

SUBCOMMANDS:
  install      Install app in players.
  uninstall    Uninstall app in players.
  launch       Launch app in players.
  close        Close app in players.
  info         Get app info in players.

---

### control tool

USAGE: {control [-v <vmindex>]} tool <subcommand>

SUBCOMMANDS:
  func        Trigger toolbar function in players.
  cmd         Run toolbar cmd in players.
  downcpu     Set CPU execute cap in players.
  location    Update location in players.
  gyro        Change gravity sensing in players.

---

### control shortcut

USAGE: {control [-v <vmindex>]} shortcut <subcommand>

SUBCOMMANDS:
  create    Create desktop shortcut for players.
  delete    Delete desktop shortcut for players.

---

### setting

USAGE: setting [-v <vmindex>] [-k <key>] [-val <value>] [-a] [-aw] [-i] [-p <path>]

OPTIONS:
  -v, --vmindex            If not specified, global setting is changed.
  -k, --key <key>          Setting key. Repeatable: --key k1 --key k2
  -val, --value <value>    Setting value. Repeatable. Use __null__ to clear.
  -a, --all                Show all keys.
  -aw, --all_writable      Show all writable keys.
  -i, --info               Show info for key.
  -p, --path <path>        Apply settings from a UTF-8 .json file. (overrides -k/-val)

---

### adb

USAGE: adb [-v <vmindex>] [-c <cmd>]

OPTIONS:
  -c, --cmd <cmd>    ADB command to run against the player's ADB port.

Known shorthand commands (passed as --cmd):
  connect / disconnect
  "getprop <key>" / "setprop <key> <val>"
  "input_text <text>"
  go_back / go_home / go_task
  key_delete / key_enter / key_space
  volume_up / volume_down / volume_mute

---

### simulation

USAGE: simulation [-v <vmindex>] [-sk <simu_key>] [-sv <simu_value>]

OPTIONS:
  -sk, --simu_key      android_id | mac_address | imei
  -sv, --simu_value    New value. Use __null__ to clear.

---

### sort

USAGE: sort
(no arguments — layouts all player windows)

---

### driver

USAGE: driver <subcommand>

SUBCOMMANDS:
  install      Install driver for players.
  uninstall    Uninstall driver for players.

---

### log

USAGE: log <subcommand>

SUBCOMMANDS:
  on     Enable manager log.
  off    Disable manager log.

---

### sh

USAGE: sh [-v <vmindex>] [-c <cmd>]

OPTIONS:
  -c, --cmd <cmd>    Shell command to run inside the player.

Same shorthand commands as `adb --cmd` apply here.
