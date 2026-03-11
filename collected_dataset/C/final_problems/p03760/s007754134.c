#include<stdio.h>
#include<string.h>
int main()
{
char a[100], b[100], c[100];
int i, j, m,n,t;
gets(a);
gets(b);
m=strlen(a)+1;
n=strlen(b)+1;
if(m=n)
t=2*n;
else
t=m+n;
for(i=0;i<t;i++)
{ 
if((i)%2==0)
 c[i]=a[i/2];
else
c[i]=b[i/2];
}
c[t]='\0';
puts(c);
return 0;
}