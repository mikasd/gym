/* 
implementation of manachers algorithm
https://www.geeksforgeeks.org/manachers-algorithm-linear-time-longest-palindromic-substring-part-1/
*/


use std::iter;

impl Solution {    
    pub fn longest_palindrome(s: String) -> String {
        if s.is_empty() {
            return "".to_string();
        }
        
        let ss = add_bounderies(&s);
        let chars = ss.chars().collect::<Vec<_>>();
        
        let n = ss.len() as u16;
        let mut p_len = iter::repeat(0).take(n as usize).collect::<Vec<u16>>();
        p_len[0] = 0u16;
        p_len[1] = 1u16;
        
        let mut max_len: u16 = 0;
        let mut max_len_pos: usize = 0;
        
        let mut center = 1u16;
        let mut right = 2u16;
                
        for c in 2..n {
            let i = c as usize;
            
            let mirror = (2 * center).saturating_sub(c) as usize;
            let diff = right.saturating_sub(c);
            
            let expand = if 0 < diff {
                if p_len[mirror] < diff {
                    p_len[i] = p_len[mirror];
                    false
                } else if p_len[mirror] == diff && c == n - 1 {
                    p_len[i] = p_len[mirror];
                    false
                } else if p_len[mirror] == diff && c < n - 1 {
                    p_len[i] = p_len[mirror];
                    true
                } else {
                    p_len[i] = diff;
                    true
                }
            } else {
                p_len[i] = 0;
                true
            };
            
            if expand {
                while (c + p_len[i]) < n && 0 < (c - p_len[i]) && (is_not_boundery_char(&p_len, c) || is_same_char(&chars, &p_len, c)) {
                    p_len[i] += 1;
                }
            }
            
            if max_len < p_len[i] {
                max_len = p_len[i];
                max_len_pos = i;
            }
            
            if right < c + p_len[i] {
                center = c;
                right = c + p_len[i];
            }
        }
        
        let first = (max_len_pos as u16).saturating_sub(max_len) / 2;
        let last = ((first + max_len).saturating_sub(1) + 1) as usize;
        return (&s[first as usize..last]).to_string();
    }
}
    
fn add_bounderies(s: &str) -> String {
    let mut r = String::from("|");
    for c in s.chars() {
        r.push(c.clone());
        r.push('|');
    }
    r
}

fn is_not_boundery_char(p_len: &Vec<u16>, i: u16) -> bool {
    (i + p_len[i as usize] + 1) % 2 == 0
}

fn is_same_char(chars: &Vec<char>, p_len: &Vec<u16>, i: u16) -> bool {
    let left = i - p_len[i as usize] - 1;
    let right = i + p_len[i as usize] + 1;
    match (chars.get(left as usize), chars.get(right as usize)) {
        (Some(a), Some(b)) => a == b,
        _ => false
    }
}