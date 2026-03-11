#include <stdio.h>
#include <stdlib.h>
#include <math.h>
int a[200000], b[200000];
int *tree[200000], d[200000], cnt[200000], cnts[200000];
int n;
long long int modpow(long long int p, long long int n, long long int mod){
    long long int res = 1;
    while(n > 0){
        if(n & 1){
            res = res * p % mod;
        }
        p = p * p % mod;
        n >>= 1;
    }
    return res;
}

long long int modinv(long long int p, long long int mod) {//逆元 p^{-1}の計算
    return modpow(p, mod - 2, mod);
}

int DFS(int x, int y){//xが今いる頂点，yが前にいた頂点
    //2つの値を求める，tree[x][x_cnt] = y, tree[y][y_cnt] = xとなる
    //x_cnt, y_cnt
    int i;
    int t = 1, t_tmp;
    for(i = 0; i < cnt[x]; i++){
        if(y == tree[x][i]){
            continue;
        }
        t += DFS(tree[x][i], x);
    }
    d[x] = t;
    return t;
}

int main(void){
    int i, j;
    long long int mod = 1000000007;
    scanf("%d", &n);
    for(i = 0; i < n; i++){
        cnt[i] = 0;
        cnts[i] = 0;
        d[i] = 1;
    }
 
    for(i = 0; i < n - 1; i++){
        scanf("%d%d", &a[i], &b[i]);
        a[i]--;
        b[i]--;
        cnt[a[i]]++;
        cnt[b[i]]++;
    }
 
    for(i = 0; i < n; i++){
        tree[i] = (int *)malloc(sizeof(int) * (cnt[i] + 1));
        if(tree[i] == NULL){
            return 0;
        }
    }

    for(i = 0; i < n; i++){
        tree[a[i]][cnts[a[i]]] = b[i];
        tree[b[i]][cnts[b[i]]] = a[i];
        cnts[a[i]]++;
        cnts[b[i]]++;
    }

    DFS(0, -1);
    //long long int bunnshi1 = 0;
    long long int bunnshi = 0;
    int x1, x2;
    for(i = 0; i < n-1; i++){
        x1 = d[a[i]];
        x2 = d[b[i]];
        if(x1 > x2){
            x1 = n - x2;
        }else{
            x2 = n - x1;
        }
        bunnshi += modpow(2, n, mod) + 1 - modpow(2, x1, mod) - modpow(2, x2, mod);
        bunnshi %= mod;

        //bunnshi1 += pow(2, n) + 1 - pow(2, x1) - pow(2, x2);
    }
    //bunnshi1 += (pow(2, n) - 1) - pow(2, n-1) * n;
    //long long int bunnbo1 = pow(2, n);
    //printf("%lld %lld\n", bunnshi1 , bunnbo1);

    bunnshi += modpow(2, n, mod) - 1 - modpow(2, n-1, mod) * n;
    long long int bunnbo = modinv(2, mod);//2の逆元
    bunnbo = modpow(bunnbo, n, mod);//2の逆元をn乗する
    long long int ans = bunnshi*bunnbo% mod;
    
    printf("%lld\n", ans);
    return 0;
}