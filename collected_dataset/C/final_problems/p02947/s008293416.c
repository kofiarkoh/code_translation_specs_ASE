#include<stdio.h>
#include<string.h>

int main(void){
  int N,count = 0; 
  scanf("%d",&N);
  char s[N + 1][11];
  for(int i = 0;i < N;i++){    
    scanf("%s",s[i]);
    for(int j = 0;j < 9;j++){
      for(int k = j + 1;k < 10;k++){
	if(s[i][j] > s[i][k]){
	  char s1 = s[i][j];
	  s[i][j] = s[i][k];
	  s[i][k] = s1;
	  
	}
      }
    }
    for(int j = 0;j < i;j++){
      if(strcmp(s[j],s[i]) == 0)
	count++;
    }
  }
  printf("%d",count);
  return 0;
}
