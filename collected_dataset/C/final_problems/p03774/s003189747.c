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


int lower_bound(int *a, int n, int key){
    int left, mid, right;
    left = 0, right = n;
    mid = (left + right)/2;
    while ((left+1 != mid || mid+1 != right) && mid != left){
        if (key > a[mid]){
            left = mid;
        }else{
            right = mid+1;
        }
        mid = (left + right)/2;
    }
    if (a[left] >= key)return left;
    if (a[mid] >= key)return mid;
    if (a[right] >= key)return right;
    return n;
}

//greatest common divisor
unsigned long  gcd(unsigned long x, unsigned long y){
    if (y == 0){ 
        return x;
    }else if (x > y){
        return gcd(y, x % y);
    }else{
        return gcd(x, y % x);
    }
}



long long factorial(int x){
    long long rtn = 1;
    int i;
    for (i = x; i > 1; i--){
        rtn = (rtn*i);
    }
    return rtn;
}
int a[55];
int b[55];
int c[55];
int d[55];

int main(void){
    int n, m;
    int min;
    int mini;
    scanf("%d %d", &n, &m);
    for (int i = 0; i < n; i++){
        scanf("%d %d", &a[i], &b[i]);
    }
    for (int i = 0; i < m; i++){
        scanf("%d %d", &c[i], &d[i]);
    }
    for (int i = 0; i < n; i++){
        min = 1000000000;
        for (int j = m-1; j >= 0; j--){
            if (min >= abs(a[i]-c[j]) + abs(b[i]-d[j])){
                min = abs(a[i]-c[j]) + abs(b[i]-d[j]);
                mini = j+1;
            }
        }
        printf("%d\n", mini);
    }

    return 0;
    
}
