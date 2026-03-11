#include<stdio.h>

int main(){
  float x,y;
  
  while(scanf("%f%f",&x,&y)!=EOF){   
    if(x<35.5&&y<71)
      puts("AAA");
    else if(x<37.5&&y<77)
      puts("AA");
    else if(x<40&&y<83)
      puts("A");
    else if(x<43&&y<89)
      puts("B");
    else if(x<50&&y<105)
      puts("C");
    else if(x<55&&y<116)
      puts("D");
    else if(x<70&&y<148)
      puts("E");
    else
      puts("NA");
  }
  return 0;
}