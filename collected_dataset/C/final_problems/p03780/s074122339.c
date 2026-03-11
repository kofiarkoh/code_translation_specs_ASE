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

/*int struct_ascending_compare(const void *p, const void *q) {
    return ((structname*)p)->member - ((strcutname*)q)->member;
}*/


int field[3][3];
int tate[3][2];
int yoko[2][3];
int total;

int dfs(int level){
    int chokudai = 0;
    if (level == 9){
        for (int i = 0; i < 3; i++){
            for (int j = 0; j < 2; j++){
                if (field[i][j] == field[i][j+1]){
                    chokudai += tate[i][j];
                }
            }
        }
        for (int i = 0; i < 2; i++){
            for (int j = 0; j < 3; j++){
                if (field[i][j] == field[i+1][j]){
                    chokudai += yoko[i][j];
                }
            }
        }
        return chokudai;
    }

    //空いているマスにlevelに応じて書き込む
    int tmp;
    int max = -1;
    int min = total + 1;
    for (int i = 0; i < 3; i++){
        for (int j = 0; j < 3; j++){
            if (field[i][j] == 0){
                if (level % 2){
                    field[i][j] = -1;
                    tmp = dfs(level+1);
                    field[i][j] = 0;
                    min = min>tmp?tmp:min;
                }else{
                    field[i][j] = 1;
                    tmp = dfs(level+1);
                    field[i][j] = 0;
                    max = max<tmp?tmp:max;
                }
            }
        }
    }
    if (level % 2){
        return min;
    }else{
        return max;
    }
}

int dp[5005];
int a[5005];
int n, k;

int check(int mid){
    memset(dp, 0, sizeof(dp));
    dp[0] = 1;

    for (int i = 0; i < n; i++){
        for (int j = 5000; j >= 0; j--){
            if (i != mid && j - a[i] >= 0 && dp[j-a[i]]){
                dp[j] = 1;
                if (i != mid && dp[j-a[i]] && j < k && j >= k-a[mid]) return 0;
            }
        }
    }

    return 1;
}


int main(void){
    scanf("%d %d", &n, &k);
    for (int i = 0; i < n; i++){
        scanf("%d", &a[i]);
    }

    qsort(a, n, sizeof(int), ascending_compare);

    int ng, mid, ok;
    ok = n, ng = -1;
    while (abs(ok - ng) > 1){
        mid = (ok + ng) / 2;
        if (!(a[mid] >= k) && check(mid)){
            ng = mid;
        }else{
            ok = mid;
        }
    }

    printf("%d\n", ng+1);
}

