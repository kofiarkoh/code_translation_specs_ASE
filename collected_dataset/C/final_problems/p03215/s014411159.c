#include <stdio.h>
#include <math.h>

int main(){
  
  int N,K,M,i,j,k;
  scanf("%d%d",&N,&K);
  M=(N+1)*N/2;
  long long int a[N],s[N+1],C[M];
  for(i=0;i<N;i++){scanf("%lld", &a[i]);}
  s[0]=0;
  for(i=1;i<=N;i++){s[i]=s[i-1]+a[i-1];}
  k=0;
  for(i=0;i<N;i++){
    for(j=i+1;j<=N;j++){C[k]=s[j]-s[i];/*printf("%lld ",C[k])*/;k++;}
  }
  
  int S[M][100];
  long long int temp;
  for(i=0;i<M;i++){
    temp=C[i];
    for(j=0;j<100;j++){
      S[i][j]=temp%2;
      //printf("%d",S[i][j]);
      temp=(long long int)(temp/2);
    }
    //printf("\n");
  }
  
  int num;int flag[M],ans[100];
  for(i=0;i<M;i++){flag[i]=1;}
  for(i=0;i<100;i++){ans[i]=0;}
  for(i=99;i>=0;i--){
    num=0;
    for(j=0;j<M;j++){
      if(S[j][i]==1 && flag[j]==1){num++;}
    }
    if(num>=K){
      ans[i]=1;
      for(j=0;j<M;j++){
	if(S[j][i]==0 && flag[j]==1){flag[j]=0;}
      }
    }
  }
  
  long long int ANS=0;
  for(i=0;i<100;i++){ANS=ANS+pow(2,i)*ans[i];}
  printf("%lld",ANS);
  
  return 0;
}
	
    
  
