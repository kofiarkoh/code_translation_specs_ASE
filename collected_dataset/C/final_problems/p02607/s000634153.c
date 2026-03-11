#include<stdio.h>

int main(void){
    register int i;
    int n;
    scanf("%d",&n);
    int a[n];
    for(i = 0;i < n;i++)scanf("%d",&a[i]);
    int ans = 0;
    for(i = 0;i < n;i++)if(!(i%2) && a[i] % 2)ans++;
    printf("%d",ans);
}