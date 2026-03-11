/* 
   AOJ 1155
   How can I satisfy thee? Let me count the ways...
   
*/

#include<stdio.h>
#define MAX_STR 100
char not[3] = {'2', '1', '0'};
char and[3][3] = {{'0', '0', '0'}, {'0', '1', '1'}, {'0', '1', '2'}};
char or[3][3] = {{'0', '1', '2'}, {'1', '1', '2'}, {'2', '2', '2'}};

char instr[MAX_STR];
int pp;

  

char interpreter(int p, int q, int r)
{
  char c = instr[pp];
  switch(c)
    {
    case '0':
    case '1':
    case '2':
      return instr[pp++];
    case 'P':
      pp++;
      return p + '0';
    case 'Q':
      pp++;
      return q + '0';
    case 'R':
      pp++;
      return r + '0';
    case '-':
      pp++;
      return not[interpreter(p, q, r) - '0'];
    case '(':
      pp++;
      char f0 = interpreter(p, q, r);
      char op = instr[pp++];
      char f1 = interpreter(p, q, r);
      pp++;
      if(op == '*')
	return and[f0 - '0'][f1 - '0'];
      else
	return or[f0 - '0'][f1 - '0'];
    }
}

int main(void)
{
  int p, q, r, ans = 0;
  while(scanf("%s", instr), instr[0] != '.')
    {
      for(p = 0; p < 3; p++)
	for(q = 0; q < 3; q++)
	  for(r = 0; r < 3; r++)
	    {
	      if(interpreter(p, q, r) == '2')
		ans++;
	      pp = 0;
	    }
      printf("%d\n", ans);
      ans = 0;
      pp = 0;
      for(p = 0; p < MAX_STR; p++)
	instr[p] = '\0';
    }
  return 0;
}