#include<stdio.h>
#include<stdlib.h>

int main(){
  int S_length=0,count_a=0,count_b=0,count_c=0;
  char S[100001];
  scanf("%s",S);
  while (S[S_length]!='\0'){ S_length++; }


for (int i = 0; i < S_length; i++)
{
  switch (S[i])
  {
  case 'a':
    count_a++;
    break;
  case 'b':
    count_b++;
    break;
  case 'c':
    count_c++;
    break;
  }
}

if (abs(count_a-count_b)<2&&
    abs(count_a-count_c)<2&&
    abs(count_b-count_c)<2  )
{
  puts("YES");
}else
{
  puts("NO");
}

  return 0;
}