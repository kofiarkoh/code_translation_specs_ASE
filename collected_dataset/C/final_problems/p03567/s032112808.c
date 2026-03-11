#include <stdio.h>
#include <string.h>

int main(void)
{
  char str[10];
  int i,j,len,yes = 0,a = 100,c = 0;

  fgets(str,10,stdin);

  len = strlen(str);
  for(i = 0;i < len - 1;i++)
    {
      if(str[i] == 'A')
        {
          if(str[i+1] == 'C')
            {
              yes = 1;
              break;
            }
        }
    }

  if(yes == 1)
    printf("Yes\n");
  else
    printf("No\n");

  return 0;
}
