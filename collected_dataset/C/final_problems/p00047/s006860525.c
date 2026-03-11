#include<stdio.h>

int a=1,b=0,c=0;

void change(int x, int y){
  int z;

  z=x; x=y; y=z;
} 

int main(void){
  char x,y;
  
  while(scanf("%c,%c",&x,&y)!=EOF){
    if(x=='A'){
      if(y=='B')
	change(a,b);
      else if(y=='C')
	change(a,c);
    }
    if(x=='B'){
      if(y=='A')
	change(b,a);
      else if(y=='C')
	change(b,c);
    }
    if(x=='C'){
      if(y=='A')
	change(c,a);
      else if(y=='B')
	change(c,b);
    }
  }
  if(a)
    puts("A");
  else if(b)
    puts("B");
  else
    puts("C");
  return 0;
}