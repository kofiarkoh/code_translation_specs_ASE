#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#include<stdint.h>
#include<tgmath.h>
#include<stdbool.h>
#include<sys/queue.h>
#include<time.h>
#define ll long long
#define min(a, b) ((a) > (b) ? (b) : (a))
#define max(a, b) ((a) > (b) ? (a) : (b))

int n, k, a[200200], b[200200];

int main(){
    scanf("%d%d", &n, &k);
    for(int i=0; i<n; i++)scanf("%d", &a[i]);
    while(k--){
        for(int i=0; i<n; i++)b[i] = 0;
        for(int i=0; i<n; i++){
            for(int j=max(0, i-a[i]); j<=min(n-1,i+a[i]); j++){
                b[j]++;
            }
        }
        bool flg = true;
        for(int i=0; i<n; i++){
            flg &= (a[i] == b[i]);
            a[i] = b[i];
        }
        if(flg)break;
    }
    for(int i=0; i<n; i++){
        if(i)putchar(32);
        printf("%d", a[i]);
    }
    putchar(10);
    return 0;
}