#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#define mod 1000000007

int main(){
  long long dp[16]={0},ndp[16]={0},l,i,j,k;
  char *s=malloc(sizeof(char)*131072);
  dp[0]=1;
  scanf("%s",s);
  l=strlen(s);
  for(i=0;i<l;i++){
    for(j=0;j<13;j++){
      for(k=0;k<=9;k++){
        if(s[i]=='?' || s[i]==('0'+k)){
          ndp[(j*10+k)%13]+=dp[j];
          ndp[(j*10+k)%13]%=mod;
        }
      }
    }
    for(j=0;j<13;j++){
      dp[j]=ndp[j];
      ndp[j]=0;
    }
  }
  printf("%lld\n",dp[5]);
  return 0;
}
