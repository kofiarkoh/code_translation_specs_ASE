#include <stdio.h>
#include <stdlib.h>

#define N 10

int main(void){
    int n, i;
    int a[N];
    int cnt[N];
    int sum=1, odd=1;

    scanf("%d", &n);
    for(i=0; i<n; i++)
        scanf("%d", &a[i]);

    for(i=0; i<n; i++){
        if(a[i]&0x01) odd *= 1;
        else odd *= 2;
        sum *= 3;
    }

    printf("%d\n", sum-odd);
    

    return 0;
}