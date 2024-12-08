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
            let terms: Vec<i64> = input_terms.iter().map(|t| t.parse().unwrap()).collect();
            'concat_loop: for k_64 in 0..2i64.pow((terms.len() - 1).try_into().unwrap()) {
                let k: usize = k_64.try_into().unwrap();
                let mut num_concats = 0;
                for x in 0..terms.len() - 1 {
                    num_concats += (k >> x) & 1;
                }
                //build terms
                let num_arith_ops: usize = terms.len() - 1 - num_concats;
                'op_loop: for i in 0..2i64.pow(num_arith_ops.try_into().unwrap()) {
                    let mut i_bits = i;
                    let mut current_total: i64 = terms[0];
                    //print!("{current_total} ");
                    for j in 1..terms.len() {
                        //determine the operator by looking at k and i bits
                        if (k >> j-1) & 1 == 1 {
                            // apply || operator
                            //print!("|| {} ", terms[j]);
                            let mut total_str = current_total.to_string();
                            total_str.push_str(&input_terms[j]);
                            current_total = total_str.parse().unwrap();
                        }
                        else {
                            if i_bits & 1 == 0 {
                                // apply + operator
                                //print!("+ {} ", terms[j]);
                                current_total += terms[j];
                            }
                            else {
                                // apply * operator
                                //print!("* {} ", terms[j]);
                                current_total *= terms[j];
                            }
                            i_bits >>= 1;  // move to the next bit from the right
                        }
                        if current_total > correct_total {
                            //println!("CAN'T MATCH ({current_total} too large)");
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
            
        }
        Ok(sum.to_string())
    }

}