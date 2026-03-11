#include<stdio.h>
#include<string.h>

int func(int);

int t;
char str[81];

int main(void){

  int ans,i;

  memset(str,'a',sizeof(str));
  ans=0;
  while(~scanf("%s",str)){
    for(i=0;i<strlen(str);i++)
      if(str[i]<='9' && str[i]>='0'){
	ans+=func(i);
	i+=t;
      }

  }
    printf("%d\n",ans);
  return 0;
}

int func(int a){
  int x=0,i;
  for(i=a;str[i]-'0'<10;i++){
    if(i!=a)x*=10;
    x+=str[i]-'0';
  }
  t=i;
  return x;
}