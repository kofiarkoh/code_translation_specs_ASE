#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <string.h>
#define NEW(p,n) {p = malloc((n)*sizeof(p[0]));}


int main()
{
  int a=0,b=400000,i,k,n;
  scanf("%d\n",&n);
  char s[n];
  for(i=0;i<n;i++){
    scanf("%c",&s[i] );
//  printf("%c",s[i] );
  }
for (i = 0; i < n; i++) {
  a=0;
    for (k = 0; k < i; k++) {
      if(s[k]=='W')
      a+=1;
    }
    for (k = i+1; k < n; k++) {
      if(s[k]=='E')
      a+=1;
    }
    if(a<=b)
    b=a;
    }
  printf("%d\n",b );
  return 0;
}
