#include <stdio.h>

int main(){
  char a[81];
  int i,j;
  
  while(fgets(a,80,stdin)!=NULL){
    for(i=0;i<9;i++){
      for(j=0;j<(9-i);j++){
        a[j]=a[j]+a[j+1]-'0';
        a[j]=(a[j]>'9'?a[j]-=10:a[j]);
      }
    }
    putchar(a[0]);putchar('\n');
  }
  return 0;
}