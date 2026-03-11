#include<stdio.h>
int n,a,i,p[200000]={},m[200000]={};
long r=0;
int main(){
    scanf("%d",&n);
    for(i=0;i<n;i++){
        scanf("%d",&a);
        if(i+a<200000)p[i+a]++;
        if(i-a>=0)m[i-a]++;
    }
    for(i=0;i<200000;i++){
        r+=p[i]*m[i];
    }
    printf("%ld",r);
    return 0;
}