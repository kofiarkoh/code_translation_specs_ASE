#include<stdio.h>
#include<string.h>

int main(){
  int N,n=0,t;
  char S[99];

  scanf("%d\n", &N);
  scanf("%[^\n]", &S);
  for( int i=0;S[i]!='\0';i++) {
    n=i+1;
  }
  t=n%2;
  if(N>1&&N==n&&t==0){
    for( int i=0;i<n/2;i++){
      if(S[i]!=S[i+n/2]){
        printf("No");
        return 0;
      }
      if(i+1==n/2){
        printf("Yes");
        return 0;
      }
      
    }
  }else{
    printf("No");
    return 0;
  }
  return 0;
}