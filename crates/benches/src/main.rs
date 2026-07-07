mod search;

fn main() {
    let n = 5;
    // let keyword = "Z29";
    // let keyword = "anemia kidney";
    // let keyword = "diarrhea infect";
    // let keyword = "infarct stroke";
    let keyword = "diabetes ketoacidosis";
    // let keyword = "dyslipidemia";
    // let keyword = "cerebral concussion";
    // let keyword = "fracture scapular";
    // let keyword = "dyspepsia";
    search::test_search(keyword, n);
    println!("Hello world");

    // ?? tail keyword, need to find head ?? or prepare Reference detail ??
}
