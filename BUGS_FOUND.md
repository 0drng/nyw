# Bugs Found in Code Quality Review

## Bug 1: Critical - todo!() panic in command_service.rs

**Severity:** Critical
**Location:** `src/service/command_service.rs:81`

### Description
There is an incomplete implementation that uses the `todo!()` macro. This will cause the application to panic at runtime.

### Issue
The `build_command()` method has a `todo!()` macro in the code path when:
- NOT requiring root privileges
- BUT already running as root

```rust
if !require_root && is_root {
    todo!();
}
```

### Impact
Application will panic and crash if this code path is executed. This happens when running as root but executing commands that don't require root privileges.

### Steps to Reproduce
1. Run nyw as root
2. Execute a command that doesn't require root privileges
3. Application will panic with `not yet implemented` error

### Suggested Fix
Implement proper privilege de-escalation or command execution logic for this scenario.

---

## Bug 2: Database schema syntax error

**Severity:** High
**Location:** `migrations/2_applications.sql`

### Description
The database migration file contains a SQL syntax error - missing comma between PRIMARY KEY and FOREIGN KEY constraints.

### Issue
```sql
CREATE TABLE applications (
    id INTEGER NOT NULL,
    name VARCHAR(255) NOT NULL,
    action TEXT NOT NULL,
    PRIMARY KEY(id, name)    -- Missing comma here
    FOREIGN KEY (id) REFERENCES locks(id)
);
```

### Impact
Database migration will fail when creating the applications table. Application won't be able to initialize properly.

### Suggested Fix
Add comma after PRIMARY KEY constraint:

```sql
CREATE TABLE applications (
    id INTEGER NOT NULL,
    name VARCHAR(255) NOT NULL,
    action TEXT NOT NULL,
    PRIMARY KEY(id, name),    -- Add comma here
    FOREIGN KEY (id) REFERENCES locks(id)
);
```

---

## Bug 3: Type mismatch in ApplicationAdd struct

**Severity:** High
**Location:** `src/model/application.rs:5`

### Description
The `ApplicationAdd` struct has an incorrect type for the `name` field. It's defined as `i64` but should be `String`.

### Issue
```rust
pub struct ApplicationAdd {
    pub id: i64,
    pub name: i64,  // Should be String
    pub action: String,
}
```

The `Application::new()` constructor expects a `String` for the name parameter, causing a type mismatch.

### Impact
Type mismatch will cause compilation errors or runtime failures. Package names are strings, not integers.

### Suggested Fix
Change the type of `name` field:

```rust
pub struct ApplicationAdd {
    pub id: i64,
    pub name: String,  // Correct type
    pub action: String,
}
```

---

## Bug 4: Incorrect log label in main.rs

**Severity:** Low
**Location:** `src/main.rs:56`

### Description
The log message says "ExecutingPostScript" but it's actually executing pre-scripts, which is confusing for debugging.

### Issue
```rust
// Line 56
LogService::log(LanguageService::get_message("Info_ExecutingPostScript", vec![]), LogLevel::INFO);
// But this is in the PRE-script execution section
```

### Impact
Misleading log messages make debugging difficult. Users will see "Executing post script" when pre-scripts are actually running.

### Suggested Fix
Change to the correct message key:

```rust
LogService::log(LanguageService::get_message("Info_ExecutingPreScript", vec![]), LogLevel::INFO);
```

---

## Bug 5: get_installed() hardcodes pacman-specific parameters

**Severity:** Medium
**Location:** `src/model/package_manager.rs:211`

### Description
The `get_installed()` method hardcodes `-Qqe` parameters, which are specific to pacman. This breaks compatibility with other package managers.

### Issue
```rust
pub fn get_installed(&self) -> Result<Vec<String>, PackageManagerError> {
    let command = format!("{} -Qqe", self.get_binary());  // Hardcoded pacman params
    // ...
}
```

The class already has a proper method `get_list_programms_param()` that returns the correct parameters for each package manager, but it's not being used.

### Impact
`get_installed()` will fail or return incorrect results for non-pacman package managers (apt, apk, brew, winget). This breaks the package removal feature for most supported platforms.

### Suggested Fix
Use the existing `get_list_programms_param()` method:

```rust
pub fn get_installed(&self) -> Result<Vec<String>, PackageManagerError> {
    let command = format!("{} {}", self.get_binary(), self.get_list_programms_param());
    // ...
}
```

---

## Bug 6: SQL type mismatch in application_repository.rs

**Severity:** Medium
**Location:** `src/repository/application_repository.rs:27`

### Description
The SQL query casts `id` as `u32` but the `Lock.id` field is defined as `i64`, causing a type mismatch.

### Issue
```rust
let apps = sqlx::query_as!(
    Application,
    r#"SELECT id as "id!:u32", name as "name!:String", action as "action!:String"
       FROM applications WHERE id = ?"#,
    lock_id
)
```

But `Lock.id` is `i64`, not `u32`.

### Impact
Type mismatch may cause runtime errors when querying applications. Potential data truncation if IDs exceed u32 range.

### Suggested Fix
Change the query to use `i64`:

```rust
let apps = sqlx::query_as!(
    Application,
    r#"SELECT id as "id!:i64", name as "name!:String", action as "action!:String"
       FROM applications WHERE id = ?"#,
    lock_id
)
```

---

## Bug 7: Typos in example.jsonc

**Severity:** Low
**Location:** `example.jsonc`

### Description
The example configuration file contains typos that would cause configuration errors if users copy this example.

### Issues Found

**1. Incorrect filename (Line 14)**
```json
"src": "https://raw.githubusercontent.com/prasanthrangan/hyprdots/main/Configs/.config/hypr/hyprand.conf",
```

Should be `hyprland.conf` not `hyprand.conf`

**2. Incorrect path format (Line 23)**
```json
"dest": "~\\hypr\\"
```

Uses Windows-style backslashes `\` instead of Unix forward slashes `/`. Should be `~/.config/hyprland/` or similar.

### Impact
Users copying the example will get incorrect configuration. Files won't be downloaded or placed in correct locations.

### Suggested Fix

Line 14:
```json
"src": "https://raw.githubusercontent.com/prasanthrangan/hyprdots/main/Configs/.config/hypr/hyprland.conf",
```

Line 23:
```json
"dest": "~/.config/hyprland/"
```
