#include<stdio.h>
int main(){
int n,k,i,mod=1000000007;
long int ans=1;
scanf("%d %d\n",&n,&k);
for(i=0;i<n;i++){
ans=ans*k;
k--;
ans=ans%mod;
}
printf("%ld\n",ans);
return 0;
}
