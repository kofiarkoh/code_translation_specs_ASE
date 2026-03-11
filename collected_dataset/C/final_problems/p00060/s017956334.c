#include <stdio.h>

int main(void){
  int c1,c2,c3;
  int i,card;
  while(scanf("%d %d %d",&c1,&c2,&c3)!=EOF){
    card=0;
    for(i=1;i<=10;i++){
      if(i!=c1&&i!=c2&&i!=c3){
	if(c1+c2+i<=20)  card++;
	else break;
      }
    }
    if(card>=4) printf("YES\n");
    else        printf("NO\n");
  }
  return 0;
}