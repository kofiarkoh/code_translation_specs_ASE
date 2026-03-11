#include<stdio.h>
#include<stdlib.h>

#define nummax 100
#define arrmax 99
int top=0,S[1000];

void push(int);
int pop();

int main()
{
  int i,num,x,y;
  char moji[nummax],a;


  while(scanf("%s",moji)!= EOF )
    { 
	  if( moji[0] == '+' )
	    {
	     x = pop();
	     y = pop();
	     push(x+y);
	    }
	  else if( moji[0] == '-' )
	    {
	     x = pop();
	     y = pop();
	     push(y-x);
	    }
	  else if( moji[0] == '*')
	    {
	     x = pop();
	     y = pop();
	     push(x*y);
	    } 
	  else
	    {
	      num = atoi(moji);
	      push(num);
	    }
    }  

  printf("%d\n",pop());

  return 0;
}

void push(int x)
{
  top++;
  S[top] = x;
}

int pop(void)
{
  top--; 
  return S[top+1];
}