use std::{error, fs, str::FromStr};

pub struct Day {
    pub file_path: String
}

impl super::Runner for Day {    

    fn run_a(&self) -> Result<String, Box<dyn error::Error>> {
        let input = fs::read_to_string(&self.file_path).unwrap();
        let mut sum: i64 = 0;
        'line_loop: for line in input.lines() {
            println!("{line}");
            let parts: Vec<&str> = line.split(':').collect();
            let correct_total: i64 = parts[0].parse().unwrap();
            let terms: Vec<i64> = parts[1].split_whitespace().map(|s| s.parse().unwrap()).collect();
            'op_loop: for i in 0..2i64.pow((terms.len() - 1).try_into().unwrap()) {
                let mut current_total: i64 = terms[0];
                //print!("{current_total} ");
                for j in 1..terms.len() {
                    //determine the operator using i
                    let jth_bit = (i >> (j-1)) & 1;
                    //apply the operator to update the total
                    if jth_bit == 0 {
                        //print!("+ {} ", terms[j]);
                        current_total += terms[j];
                    }
                    else {
                        //print!("* {} ", terms[j]);
                        current_total *= terms[j];
                    }
                    if current_total > correct_total {
                        //println!("CAN'T MATCH (too large)");
                        continue 'op_loop;
                    }
    
                }
                //println!("= {current_total} vs {correct_total}");
                if current_total == correct_total {
                    println!("MATCH");
                    sum += correct_total;
                    continue 'line_loop;
                }
            }
        }
        Ok(sum.to_string())
    }

    fn run_b(&self) -> Result<String, Box<dyn error::Error>> {
        let input = fs::read_to_string(&self.file_path).unwrap();
        let mut sum: i64 = 0;
        'line_loop: for line in input.lines() {
            println!("{line}");
            let parts: Vec<&str> = line.split(':').collect();
            let correct_total: i64 = parts[0].parse().unwrap();
            let input_terms: Vec<&str> = parts[1].split_whitespace().collect();
            for k in 0..2i64.pow((input_terms.len() - 1).try_into().unwrap()) {
                //build terms
                let mut current_term: String = String::from(input_terms[0]);
                let mut terms: Vec<i64> = Vec::new();
                for a in 1..input_terms.len() {
                    let ath_bit = (k >> a-1) & 1;
                    if ath_bit == 0 {
                        //don't concat; parse the current string and push to the input terms 
                        terms.push(current_term.parse().unwrap());
                        current_term = String::from(input_terms[a]);
                    }
                    else {
                        current_term.push_str(input_terms[a]);
                    }
                }
                // push the final input term
                terms.push(current_term.parse().unwrap());
                println!("Considering {:?}", terms);
                'op_loop: for i in 0..2i64.pow((terms.len() - 1).try_into().unwrap()) {
                    let mut current_total: i64 = terms[0];
                    print!("{current_total} ");
                    for j in 1..terms.len() {
                        //determine the operator using i
                        let jth_bit = (i >> (j-1)) & 1;
                        //apply the operator to update the total
                        if jth_bit == 0 {
                            print!("+ {} ", terms[j]);
                            current_total += terms[j];
                        }
                        else {
                            print!("* {} ", terms[j]);
                            current_total *= terms[j];
                        }
                        if current_total > correct_total {
                            println!("CAN'T MATCH (too large)");
                            continue 'op_loop;
                        }
        
                    }
                    println!("= {current_total} vs {correct_total}");
                    if current_total == correct_total {
                        println!("MATCH");
                        sum += correct_total;
                        continue 'line_loop;
                    }
                }
            }
            
        }
        Ok(sum.to_string())
    }

}