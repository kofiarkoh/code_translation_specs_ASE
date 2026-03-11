#include<stdio.h>
#include <string.h>
int main(void)
{
  char S[11];
  char A[11]="";
  char B[3]="hi";
  int i;
  int flag=0;
  scanf("%s",S);
  for(i=0;i<5;i++){
  	if(strcmp(S,A)==0){
      flag=1;
      break;
    }
    else{
      strcat(A,B);
    }
  }
  if(flag==0){
    printf("No");
  }
  else{
    printf("Yes");
  }
  return 0;
}

  
