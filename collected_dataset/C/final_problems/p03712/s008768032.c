#include <stdio.h>

int main()
{
  int h, w;
  int i;
  char a[102][102] = {};
  scanf("%d %d",&h ,&w);
  for(i=0; i<h; i++)
  {
    scanf("%s",&a[i][0]);
  }

  for(i=0; i<w+2; i++)
  {
    printf("#");
  }
  printf("\n");

  for(i=0; i<h; i++)
  {
    printf("#%s#\n",a[i]);
  }
  for(i=0; i<w+2; i++)
  {
    printf("#");
  }
  printf("\n");

  return 0;
}
