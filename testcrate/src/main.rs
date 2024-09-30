use testcrate::version;

fn main() {
    let (major, minor, patch) = version();
    println!("liquid-dsp version: {}.{}.{}", major, minor, patch);
}
