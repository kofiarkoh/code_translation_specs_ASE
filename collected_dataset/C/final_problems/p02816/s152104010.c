#include <stdio.h>
long long int a[200000], b[200000];
long long int a_xor[200000], b_xor[200000];
int main(void){
    int n;
    int m = 10;//baseの数，最大10まで
    long long int base[10] = {222, 337, 511, 7, 11, 13, 17, 19, 23, 29};
    long long int mod = 9007199254740997;
    scanf("%d", &n);
    int i, j, k;
    for(i = 0; i < n; i++){
        scanf("%lld", &a[i]);
    }
    for(i = 0; i < n; i++){
        scanf("%lld", &b[i]);
    }
    //b_iとb_i+1のxorを求める, aも同様に求める
    for(i = 0; i < n; i++){
        b_xor[i] = (b[i] ^ b[(i+1)%n]) + 1;//b []が0になるのを防ぐため1を足す
        a_xor[i] = (a[i] ^ a[(i+1)%n]) + 1;
    }
    // for(i = 0; i < n; i++){
    //     printf("a: %d b: %d\n", a_xor[i], b_xor[i]);
    // }

    //初期(k=0)のhash値を求める
    long long int ahash[m], bhash[m];
    for(j = 0; j < m; j++){
        ahash[j] = 0;
        bhash[j] = 0;
        for(i = 0; i < n; i++){
            ahash[j] += a_xor[i];
            ahash[j] %= mod;
            bhash[j] += b_xor[i];
            bhash[j] %= mod;
            if(i == n - 1){
                break;
            }
            ahash[j] *= base[j];
            ahash[j] %= mod;
            bhash[j] *= base[j];
            bhash[j] %= mod;
        }
    }

    //base^(n-1)%mod の値を求める
    long long int tmp[m];
    for(j = 0; j < m; j++){
        tmp[j] = 1;
        for(i = 0; i < n - 1; i++){
            tmp[j] *= base[j];
            tmp[j] %= mod;
        }
    }

    //kでfor文を回す
    for(i = 0; i < n; i++){
        int cnt = 0;
        for(j = 0; j < m; j++){
            if(ahash[j] == bhash[j]){
                cnt++;
            }
        }
        
        if(cnt == m){
            printf("%lld %lld\n", i, b[0]^a[i]);
        }

        //ahashを1つずらす
        for(j = 0; j < m; j++){
            ahash[j] = (ahash[j] - tmp[j]*a_xor[i])*base[j] + a_xor[i];
            while(ahash[j] < 0){
                ahash[j] += mod; 
            }
            ahash[j] %= mod;
        }
        
    }
    return 0;
}