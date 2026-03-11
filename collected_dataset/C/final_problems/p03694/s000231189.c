#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <math.h>
#define MOD 1000000007
#define ll long long
#define MAX 10001

int cmpfunc(const void *a, const void *b){
    return (*(int*)a - *(int*)b);
}

int main(void){
    int n,i;
    scanf("%d",&n);
    int a[n];
    for(i=0;i<n;i++) scanf("%d",&a[i]);
    if(n==1){
        printf("0\n");
        return 0;
    }
    qsort(a,n,sizeof(int),cmpfunc);
    printf("%d\n",a[n-1]-a[0]);
    return 0;
}
