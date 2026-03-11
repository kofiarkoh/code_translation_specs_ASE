#include <stdio.h>
#include <string.h>

int main(void){
  int i,j,flg,rst=0;
  char x[81];

  while(scanf("%s",x) != EOF){
    for(flg=1,i=0,j=strlen(x)-1;i!=j && i!=j+1;i++,j--){
      if(x[i] != x[j]){
	flg = 0;
	break;
      }
    }
    if(flg) rst++;
  }
  printf("%d\n",rst);

  return 0;
}