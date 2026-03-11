#include<stdio.h>

int main(){
  int i,d,m,w[12]={31,29,31,30,31,30,31,31,30,31,30,31};
  char week[7][11]={"Thursday","Friday","Saturday","Sunday","Monday","Tuesday","Wednesday"};
  do{
    scanf("%d %d",&m,&d);
    if(!m&&!d) break;
    m--; d--;
    for(i=0;i<m;i++) d+=w[i];
    printf("%s\n",week[d%7]);    
  }while(1);
  return 0;
}