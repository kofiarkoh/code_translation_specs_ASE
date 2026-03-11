#include<stdio.h>

int main()
{
 char st[105];
 int i, midchar;
 int charcount=0;

 for(i=0; i<100; i++)
 {
  scanf("%c",&st[i]);
  if((st[i]<'a')|(st[i]>'z')) break;
  charcount++;
 }

 midchar=charcount-2;
 printf("%c%d%c\n",st[0],midchar,st[charcount-1]);
return 0;
}
