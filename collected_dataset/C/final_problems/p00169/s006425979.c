#include<stdio.h>
#include<stdlib.h>
#include<string.h>
int main(){
  int i;
  int c[128],m,n;
  char s[256],*t;  
  for(;;){
    fgets(s,256,stdin);
    s[strlen(s)-1]='\0';
    t=strtok(s," ");
    c[0]=atoi(t)<10?atoi(t):10;
    if(c[0]==0)
      break;
    for(n=1;(t=strtok(NULL," "))!=NULL;n++)
      c[n]=atoi(t)<10?atoi(t):10;
    m=0;
    for(i=0;i<n;i++)
      m+=c[i];
    for(i=0;i<n;i++){
      if(m<12&&c[i]==1)
	m+=10;
    }
    if(m>21)
      m=0;
    printf("%d\n",m);
  }
  return 0;
}