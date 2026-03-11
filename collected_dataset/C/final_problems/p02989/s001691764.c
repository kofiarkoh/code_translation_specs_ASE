#include<stdio.h>
int main()
{
int a,c,d,e,f,i,j;
scanf("%d",&a);
  f=0;
  e=a/2;
  int b[a];
for(i=0;i<a;++i)
  {
  scanf("%d",&c);
  b[i]=c;
  }
  
for(i=0;i<100000;++i)  
  {d=0;
   for(j=0;j<a;++j) {
     if(b[j]>=i) d=d+1;
    }    
   if(d==e) f=f+1;
  }
  printf("%d",f);
return 0;
}