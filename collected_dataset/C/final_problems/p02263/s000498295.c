#include<stdio.h>
#include<stdlib.h>
#include<string.h>
int pop(void);
void push(int);

int main(){
  int x, y, top, n[101];
  char s[100];
  top = 1;

  while(scanf("%s", s) != EOF){
    if(s[0] == '+'){
      x = n[top];
      y = n[top-1];
      top--;
      n[top] = x+y;
    }
 else if(s[0] == '-'){
      y = n[top];
      x = n[top-1];
      top--;
      n[top] = x-y;
    }
 else if(s[0] == '*'){
      x=n[top];
      y=n[top-1];
      top--;
      n[top] = x*y;
    }
 else{
      top++;
      n[top] = atoi(s);
    }
  }
  printf("%d\n",n[top]);
  return 0;
}