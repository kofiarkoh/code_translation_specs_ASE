#include<stdio.h>

int main(){
  char s[128];
  int y,m,d;
  while(scanf("%s%d%d%d",s,&y,&m,&d),s[0]!='#'){
    if(y<31 || (y==31 && m<5)){
      printf("HEISEI %d %d %d\n",y,m,d);
    }
    else{
      printf("? %d %d %d\n",y-30,m,d);
    }
  }
  return 0;
}

