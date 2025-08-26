use assert_cmd::Command;

#[test]
fn run_node_template_example() {
    Command::new("cargo")
        .args(["run", "--example", "node_template"])
        .assert()
        .success();
}
