#include <stdio.h>

int main()
{
 int n,m,a,mxh,mih,yoko[1001][3];
 int i,t,e;
 for(scanf("%d%d%d",&n,&m,&a); n || m || a;scanf("%d%d%d",&n,&m,&a))
 {
   mxh=0,mih=1000;
  for(i=1;i<=m;i++)
   {
    scanf("%d%d%d",&yoko[i][0],&yoko[i][1],&yoko[i][2]);
    if(yoko[i][0]>mxh)mxh=yoko[i][0];
    if(yoko[i][0]<mih)mih=yoko[i][0];
   }

  for(t=mxh;t>=mih;t--)
   for(i=1;i<=m;i++)
     {if(t==yoko[i][0] && (a==yoko[i][1] || a==yoko[i][2]))
       a=a==yoko[i][1]?yoko[i][2]:yoko[i][1];}
printf("%d\n",a);
}
  

 return 0;
}
