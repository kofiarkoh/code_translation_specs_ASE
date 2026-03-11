#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

int descending_compare(const void *a, const void *b){
    if (*(int*)a > *(int*)b){
        return -1;
    }else if (*(int*)a == *(int*)b){
        return 0;
    }else{
        return 1;
    }
}

int ascending_compare(const void *a, const void *b){
    if (*(int*)a < *(int*)b){
        return -1;
    }else if (*(int*)a == *(int*)b){
        return 0;
    }else{
        return 1;
    }
}

// array pointer = *a, num of element = n, key key
int lower_bound(int *a, int n, int key){
    int ng, mid, ok;
    ng = -1, ok = n-1;
    while (abs(ok - ng) > 1){
        mid = (ok + ng) / 2;
        if (key <= a[mid]){
            ok = mid;
        }else{
            ng = mid;
        }
    }
    if (a[ok] >= key)return ok;
    return n;
}

//greatest common divisor
unsigned long gcd(unsigned long x, unsigned long y){
    if (y == 0){ 
        return x;
    }else if (x > y){
        return gcd(y, x % y);
    }else{
        return gcd(x, y % x);
    }
}

unsigned long lcm(unsigned long x, unsigned long y){
    unsigned long g = gcd(x, y);
    return x*y/g;

}



long long factorial(int x){
    long long rtn = 1;
    int i;
    for (i = x; i > 1; i--){
        rtn = (rtn*i);
    }
    return rtn;
}

/*
int struct_ascending_compare(const void *p, const void *q) {
return ((struct_name*)p)->member_name - ((struct_name*)q)->member_name;
}*/

/*unsigned long long pascal[100][100] = {0};
void make_pascal(void){
    for (int i = 0; i < 100; i++){
        pascal[i][0] = 1;
    }
    pascal[1][1] = 1;
    for (int i = 2; i < 100; i++){
        for (int j = 1; j < 100; j++){
           pascal[i][j] = (pascal[i-1][j-1]+pascal[i-1][j]) % mod;
        }
    }
}*/
long long mod = 1000000007;

//x ^ n
long long mod_pow(long long x, long long n){
    long long res = 1;
    for(int i = 0;i < 60; i++){
        if(n >> i & 1) res = res * x % mod;
        x = x * x % mod;
    }
    return res;
}


long long imos[100005];
long long a[100005];
long long minus[100005] = {0};
long long plus[100005] = {0};
int main(void){
    int n, k;
    scanf("%d %d", &n, &k);
    for (int i = 0; i < n; i++){
        scanf("%lld", &a[i]);
        if (a[i] < 0) minus[i] = -1*a[i];
        else plus[i] = a[i];
    }

    long long score = 0;

    for (int i = 0; i < n; i++){
        if (a[i] > 0) score += a[i]; 
    }
    imos[0] = a[0];
    for (int i = 1; i < n; i++){
        imos[i] = imos[i-1] + a[i];
        plus[i] += plus[i-1];
        minus[i] += minus[i-1];
    }
    long long white = 100000000000005;
    long long black = 100000000000005;
    for (int i = k-1; i < n; i++){
        if (i - k < 0){
            if (plus[i] < white){
                white = plus[i];
            }
            if (minus[i] < black){
                black = minus[i];
            }
        }else{
            if (plus[i] - plus[i-k] < white){
                white = plus[i] - plus[i-k];
            }
            if (minus[i] - minus[i-k] < black){
                black = minus[i] - minus[i-k];
            }
        }
    }

    long long max_black = score - black;
    long long max_white = score - white;


    score = max_black>max_white?max_black:max_white;
    printf("%lld\n", score);
    
    
}
