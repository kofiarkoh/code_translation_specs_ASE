#include<stdio.h>
#include<string.h>
int main(void)
{
  int n,o,p;
  char s[10],b[3];
  scanf("%d",&n);
  while(n--){
    o=3;p=0;
    while(o){
      scanf("%s",s);
      if(!strcmp(s,"HIT")){
	if(b[2])b[2]=0,p++;
	if(b[1])b[2]=1,b[1]=0;
	if(b[0])b[1]=1,b[0]=0;
	b[0]=1;
      }
      if(!strcmp(s,"OUT")){
	o--;
      }
      if(!strcmp(s,"HOMERUN")){
	p+=b[0]+b[1]+b[2]+1;
	memset(b,0,sizeof(b));
      }
    }
    memset(b,0,sizeof(b));
    printf("%d\n",p);
  }
  return 0;
}