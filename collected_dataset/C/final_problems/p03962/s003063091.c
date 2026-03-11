#include<stdio.h>
int main() {
    int a[3],cnt=3;
    for(int i=0;i<=2;i++) 
        scanf("%d",&a[i]);
    if(a[0]==a[1]) cnt--;
    if(a[1]==a[2]) cnt--;
    if(a[0]==a[2]) cnt--;
    printf("%d",cnt);
    return 0;
}