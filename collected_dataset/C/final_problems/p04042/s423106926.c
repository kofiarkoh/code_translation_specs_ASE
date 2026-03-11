#include<stdio.h>
#include<stdlib.h>
#include<string.h>

int main(void){
  int N,K;
  char *str[2000];
  int len[2000];
  char **dp[2];
  int i,j,i1,i2;

  scanf("%d%d%*c",&N,&K);

  for(i=0;i<N;i++){
    str[i]=(char *)calloc(K+1,sizeof(char));
    scanf("%s%*c",str[i]);
    len[i]=strlen(str[i]);
  }

  dp[0]=(char **)calloc(K+1,sizeof(char *));
  dp[1]=(char **)calloc(K+1,sizeof(char *));
  for(i=0;i<=K;i++){
    dp[0][i]=(char *)calloc(i+1,sizeof(char));
    dp[1][i]=(char *)calloc(i+1,sizeof(char));
  }

  strcpy(dp[0][len[0]],str[0]);
  for(i=1;i<N;i++){
    i1=i%2;
    i2=1-i1;
    for(j=0;j<=K;j++){
      if(j<len[i]) strcpy(dp[i1][j],dp[i2][j]);
      else{
	strcpy(dp[i1][j],dp[i2][j-len[i]]);
	strcat(dp[i1][j],str[i]);
	//	printf("(i,j)=(%d,%d):%s\n",i,j,dp[i1][j]);
	if((j!=len[i] && strcmp(dp[i2][j-len[i]],"")==0) || (strcmp(dp[i2][j],"")!=0 && strcmp(dp[i2][j],dp[i1][j])<0)){
	  //  puts("in if");
	  strcpy(dp[i1][j],dp[i2][j]);
	}
      }
    }
  }
  puts(dp[i1][K]);
  return 0;
}
