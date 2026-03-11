#include<stdio.h>
int main(void)
{
    int a[20];
    int q,i,k,l;
    k=0;
    scanf("%d",&q);
    for(i=0;i<q;i++){
    scanf("%d",&a[i]);}
    
    for(i=1;i<q-1;i++){
    if(a[i]<a[i+1]&&a[i]>a[i-1]){
    k++;}else if(a[i]>a[i+1]&&a[i]<a[i-1]){
    k++;}
    }
    printf("%d",k);
    return 0;
    }
