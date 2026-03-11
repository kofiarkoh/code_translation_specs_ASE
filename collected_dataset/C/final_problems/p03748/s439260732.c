#include<stdio.h>
#define mod 1000000007

int main(){
  long long n,m,i,j,k,l,np,nf,nr,nb,res=0;
  long long dp[8192][2]={0},ndp[8192][2]={0};
  scanf("%lld%lld",&n,&m);
  for(i=1;i<=n;i++){
    dp[i][0]=1;
  }
  dp[0][1]=1;
  for(i=0;i<m;i++){
    for(j=0;j<=n;j++){
      for(k=0;k<2;k++){
        for(l=0;l<4;l++){
          nr=j;
          nb=n-j;
          np=dp[j][k];nf=k;
          if((l/2)==0){
            nb--;
            if(nb<0){np=0;}
          }
          else{
            nr--;
            if(nr==0){nf=1;}
            if(nr<0){np=0;}
          }

          nr++;
          nb++;

          if((l%2)==0){
            nb--;
            if(nb<0){np=0;}
          }
          else{
            nr--;
            if(nr==0){nf=1;}
            if(nr<0){np=0;}
          }
          if(np==0){continue;}
          ndp[nr][nf]+=np;
        }
      }
    }
    for(j=0;j<=n;j++){
      dp[j][0]=ndp[j][0]%mod;
      dp[j][1]=ndp[j][1]%mod;
      ndp[j][0]=0;
      ndp[j][1]=0;
    }
  }
  for(i=0;i<=n;i++){
    res+=dp[i][1];
  }
  printf("%lld\n",res%mod);
}
