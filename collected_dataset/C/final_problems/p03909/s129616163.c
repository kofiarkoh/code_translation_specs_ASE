#include <stdio.h>
#include <string.h>

int main(){
  int H,W;
  char S[6];
  scanf("%d %d\n",&H,&W);
  for(int i=0;i<H;i++){
    for(int j=0;j<W;j++){
      scanf("%s",S);
      if(strcmp(S,"snuke")==0) printf("%c%d",j+'A',i+1);
    }
  }
  return 0;
}