#include <stdio.h>

int main (void)
{
  char coffee[6];
   
  scanf("%s",coffee);
  
  if (coffee[2] == coffee[3]) {
    if (coffee[4] == coffee[5])
      printf("Yes\n");
  }
  
  else printf("No\n");
}