#include<stdio.h>
#include<string.h>

int main()
{
  int N,t[50];
  char X[100],s[50][100];
  int i,j,len,x,ans = 0;

  scanf("%d",&N);
  for(i = 0;i < N;i++){
    scanf("%s",s[i]);
    scanf("%d",&t[i]);
  }
  scanf("%s",X);

  len = strlen(X);

  for(i = 0;i < N;i++){
    if(X[0] == s[i][0]){
      j = 0;
      for(j = 0;j < len;j++){
	if(X[j] != s[i][j]){
	  break;
	}	  
      }
    }
    if(j == len){
      x = i;
    }
    j = 0;
  }

  for(i = x + 1;i < N;i++){
    ans += t[i];
  }

  printf("%d\n",ans);

return 0;
}
  
