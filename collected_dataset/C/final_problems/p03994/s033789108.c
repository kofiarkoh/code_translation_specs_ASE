#include <stdio.h>
#include <string.h>
int main() {
  char x[20000];
  
  int num;
  scanf("%s",x);
  scanf("%d",&num);
  char y[strlen(x)];
  memset( y, '\0' , strlen(y));
  //printf("%s",x);
  //printf("%lu",strlen(x));
  for(int i=0; i<strlen(x);i++){
	int a = 26-((int)x[i]-97);
	if(a <= num){
	  num-=a;
	  y[i]='a';
	}else{
	  y[i]=x[i];
	}
  }
  int a = (int)y[strlen(x)-1]-97;
  if (a + num < 26){
	y[strlen(x)-1]=y[strlen(x)-1]+num;
  }else{
	a = a+num-26;
	y[strlen(x)-1]=y[strlen(x)-1]+a;
  }
  printf("%s\n",y);
  return 0;
}
