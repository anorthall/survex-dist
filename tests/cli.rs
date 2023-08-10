use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_usage_message_is_displayed() {
    let mut cmd = Command::cargo_bin("survex-dist").unwrap();
    cmd.arg("hello")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Usage: survex-dist <FILE> <START> <END>",
        ));
}

#[test]
fn test_invalid_file_error_message() {
    let mut cmd = Command::cargo_bin("survex-dist").unwrap();
    cmd.arg("file-does-not-exist")
        .arg("a")
        .arg("a")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Unable to open file 'file-does-not-exist'.",
        ));
}

#[test]
fn test_invalid_station_error_message() {
    let mut cmd = Command::cargo_bin("survex-dist").unwrap();
    cmd.arg("tests/data/nottsii.3d")
        .arg("node-does-not-exist-1")
        .arg("node-does-not-exist-1")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "There were no full or partial matches for the station name 'node-does-not-exist-1'.",
        ));
}

#[test]
fn test_similar_node_names_are_displayed() {
    let mut cmd = Command::cargo_bin("survex-dist").unwrap();
    cmd.arg("tests/data/nottsii.3d")
        .arg("nottsii.entrance")
        .arg("boxheadconnection.5")
        .assert()
        .failure()
        .stderr(
            r#"There were 2 possible matches for the station name 'boxheadconnection.5'.

The matches were:

  nottsii.countlazloall.brunokranskiesboxheadconnection.50
  nottsii.countlazloall.brunokranskiesboxheadconnection.5

Please provide a more specific station name and try again.
"#,
        );
}

#[test]
fn test_pathfinding_with_short_names() {
    let mut cmd = Command::cargo_bin("survex-dist").unwrap();
    cmd.arg("tests/data/nottsii.3d")
        .arg("nottsii.ent")
        .arg("boxheadconnection.50")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            r#"
| Start station          | nottsii.entrance                                         |
| End station            | nottsii.countlazloall.brunokranskiesboxheadconnection.50 |
| Path length            | 135                                                      |
| Path distance          | 511.26m                                                  |
| Straight line distance | 226.65m                                                  |
"#,
        ));
}

#[test]
fn test_pathfinding_with_json_output() {
    let mut cmd = Command::cargo_bin("survex-dist").unwrap();
    cmd.arg("tests/data/nottsii.3d")
        .arg("nottsii.ent")
        .arg("boxheadconnection.50")
        .arg("--format")
        .arg("json")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            r#"
  "metadata": [
    {
      "name": "Start station",
      "value": "nottsii.entrance"
    },
    {
      "name": "End station",
      "value": "nottsii.countlazloall.brunokranskiesboxheadconnection.50"
    },
    {
      "name": "Path length",
      "value": "135"
    },
    {
      "name": "Path distance",
      "value": "511.26m"
    },
    {
      "name": "Straight line distance",
      "value": "226.65m"
    },
"#,
        ));
}

#[test]
fn test_pathfinding_with_text_output() {
    let mut cmd = Command::cargo_bin("survex-dist").unwrap();
    cmd.arg("tests/data/nottsii.3d")
        .arg("nottsii.ent")
        .arg("boxheadconnection.50")
        .arg("--format")
        .arg("text")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            r#"
Start station: nottsii.entrance
End station: nottsii.countlazloall.brunokranskiesboxheadconnection.50
Path length: 135
Path distance: 511.26m
Straight line distance: 226.65m
"#,
        ));
}

#[test]
fn test_pathfinding_with_no_possible_route() {
    let mut cmd = Command::cargo_bin("survex-dist").unwrap();
    cmd.arg("tests/data/test.3d")
        .arg("1.1")
        .arg("2.1")
        .assert()
        .failure()
        .stderr("Unable to find a route between '1.1' and '2.1'.\n");
}

#[test]
fn test_pathfinding_with_excluded_station() {
    let mut cmd = Command::cargo_bin("survex-dist").unwrap();
    cmd.arg("tests/data/nottsii.3d")
        .arg("nottsii.entrance")
        .arg("boxheadconnection.50")
        .arg("--format")
        .arg("text")
        .arg("--exclude")
        .arg("gordonsinlet.10")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            r#"
Start station: nottsii.entrance
End station: nottsii.countlazloall.brunokranskiesboxheadconnection.50
Path length: 147
Path distance: 623.31m
Straight line distance: 226.65m
Excluded station: committeepotentrance.gordonsinlet.10"#,
        ));
}

#[test]
fn test_pathfinding_with_via_point() {
    let mut cmd = Command::cargo_bin("survex-dist").unwrap();
    cmd.arg("tests/data/nottsii.3d")
        .arg("nottsii.entrance")
        .arg("boxheadconnection.50")
        .arg("--format")
        .arg("text")
        .arg("--via")
        .arg("mainstreamway3.40")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            r#"
Start station: nottsii.entrance
End station: nottsii.countlazloall.brunokranskiesboxheadconnection.50
Path length: 147
Path distance: 623.31m
Straight line distance: 226.65m
Via station: nottsii.mainstreamway.mainstreamway3.40"#,
        ))
        .stdout(predicate::str::contains(
            "47: mainstreamway.mainstreamway3.40 - 4.03m - 252.29m",
        ));
}
