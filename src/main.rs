use std::io::stdin;

fn main() -> eyre::Result<()> {
    let mut input = String::with_capacity(200);
    while stdin().read_line(&mut input)? != 0 {}
    let result: serde_json::Value = patched_ron::from_str(&input)?;
    println!("{}", serde_json::to_string(&result)?);

    Ok(())
}
