/* A1562.c
 */
#include <stdio.h>
int ans[]={0,1,2,4,6,16,12,64,24,36,48,1024,60};

int main()
{ int n;
  scanf("%d",&n);
  printf("%d\n",ans[n]);
  return(0);
}

