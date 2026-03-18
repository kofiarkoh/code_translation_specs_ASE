use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };
    let mut m: i32 = it.next().unwrap().parse().unwrap();

    // C arrays:
    // int e[610], c[610][610]={};
    // double d[1010]={};
    // We'll size to at least n+1, but keep original upper bounds to match indexing expectations.
    let mut e = vec![-1i32; 610];
    let mut c = vec![vec![0i32; 610]; 610];
    let mut d = vec![0f64; 1010];

    // for(i=0;i<=n;i++)d[i]=1e9; d[n]=0;
    for i in 0..=n {
        d[i] = 1e9f64;
    }
    d[n] = 0.0;

    // for(i=0;i<=n;i++)e[i]=-1;
    for i in 0..=n {
        e[i] = -1;
    }

    // while(m--){ scanf("%d %d",&i,&j); c[i][j]=1; }
    while m > 0 {
        m -= 1;
        let i: usize = it.next().unwrap().parse().unwrap();
        let j: usize = it.next().unwrap().parse().unwrap();
        c[i][j] = 1;
    }

    // for(i=n-1;i;i--){
    //   s=b=a=0;
    //   for(j=i;j<=n;j++){
    //     if(c[i][j]){
    //       s+=d[j]+1;
    //       if(b==0||d[b]<d[j])b=j;
    //       a++;
    //     }
    //   }
    //   d[i]=s/a;
    //   if(a>1)e[i]=b;
    // }
    //
    // Note: In the C code, if a==0 then d[i]=s/a is division by zero (undefined/inf/NaN).
    // We preserve IEEE-754 behavior by doing the same in f64.
    for i in (1..=n.saturating_sub(1)).rev() {
        let mut s: f64 = 0.0;
        let mut b: usize = 0;
        let mut a: i32 = 0;
        for j in i..=n {
            if c[i][j] != 0 {
                s += d[j] + 1.0;
                if b == 0 || d[b] < d[j] {
                    b = j;
                }
                a += 1;
            }
        }
        d[i] = s / (a as f64);
        if a > 1 {
            e[i] = b as i32;
        }
    }

    let mut minv: f64 = d[1];

    // for(k=1;k<=n;k++){
    //   if(e[k]==-1)continue;
    //   for(j=0;j<n;j++)d[j]=1e9;
    //   d[n]=0;
    //   for(i=n-1;i;i--){
    //     s=a=0;
    //     for(j=i;j<=n;j++){
    //       if(c[i][j]==0||(i==k&&j==e[k]))continue;
    //       s+=d[j]+1;
    //       a++;
    //     }
    //     d[i]=s/a;
    //   }
    //   if(min>d[1])min=d[1];
    // }
    for k in 1..=n {
        if e[k] == -1 {
            continue;
        }
        for j in 0..n {
            d[j] = 1e9f64;
        }
        d[n] = 0.0;

        let ek = e[k] as usize;
        for i in (1..=n.saturating_sub(1)).rev() {
            let mut s: f64 = 0.0;
            let mut a: i32 = 0;
            for j in i..=n {
                if c[i][j] == 0 || (i == k && j == ek) {
                    continue;
                }
                s += d[j] + 1.0;
                a += 1;
            }
            d[i] = s / (a as f64);
        }

        if minv > d[1] {
            minv = d[1];
        }
    }

    // printf("%.10lf\n",min);
    println!("{:.10}", minv);
}
