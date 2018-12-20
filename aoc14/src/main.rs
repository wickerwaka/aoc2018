struct Recipes {
    scores: Vec<u8>,
    elf1: usize,
    elf2: usize,
}

impl Recipes {
    fn new() -> Recipes {
        Recipes {
            scores: vec![3, 7],
            elf1: 0,
            elf2: 1,
        }
    }

    fn step(&mut self) {
        let score1 = self.scores[self.elf1];
        let score2 = self.scores[self.elf2];
        let sum = score1 + score2;
        if sum >= 10 {
            self.scores.push(sum / 10);
            self.scores.push(sum % 10);
        } else {
            self.scores.push(sum);
        }

        let len = self.scores.len();
        self.elf1 = (self.elf1 + (score1 as usize) + 1) % len;
        self.elf2 = (self.elf2 + (score2 as usize) + 1) % len;
    }

    fn get_10_after(&mut self, n: usize) -> &[u8] {
        while self.scores.len() < (n + 10) {
            self.step();
        }

        &self.scores[n..n + 10]
    }

    fn get_first(&mut self, pat: &[u8]) -> usize {
        loop {
            self.step();
            let slen = self.scores.len();
            let plen = pat.len();
            if slen >= plen + 1 {
                let sl = &self.scores[slen - (plen + 1)..slen - 1];
                if sl == pat {
                    return slen - (plen + 1);
                }
            }
            if slen >= plen {
                let sl = &self.scores[slen - plen..slen];
                if sl == pat {
                    return slen - plen;
                }
            }
        }
    }
}

fn main() {
    {
        let mut recipes = Recipes::new();
        let score = recipes.get_10_after(607331);
        println!("Part1: {:?}", score);
    }

    {
        let mut recipes = Recipes::new();
        println!("Part2: {}", recipes.get_first(&[6, 0, 7, 3, 3, 1]));
    }
}
